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

use crate::AdjacencyMatrix;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Differentiate;
use crate::DumpGraphML;
use crate::Matching;
use crate::Sorting;
use crate::SymbolicJacobian;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

/*
 * relaxation from gausian elemination
 *
 */
pub(crate) fn relaxSystem(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(
        inDAE,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: bool| relaxSystem0(&__a0, __a1, __a2),
        false,
    )?;
    Ok(outDAE)
}

fn relaxSystem0(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inChanged: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outChanged: bool;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut b2: bool;
    let __pa0 = ::match_deref::match_deref! { match &((*isyst)) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    (osyst, outShared, b2) = relaxSystem1(isyst, inShared, &comps)?;
    outChanged = inChanged || b2;
    Ok((osyst, outShared, outChanged))
}

fn relaxSystem1(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outRunMatching: bool;
    (osyst, oshared, outRunMatching) = 'mc: {
        let __mc_input = (ishared.clone(), &**inComps);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((isyst.clone(), ishared.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (shared @ Deref @ BackendDAE::Shared { functionTree: funcs, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }, tail: comps }) => {
                    let mut eorphans: metamodelica::List<i32>;
                    let mut vorphans: metamodelica::List<i32>;
                    let mut unassigned: metamodelica::List<i32>;
                    let mut otherorphans: metamodelica::List<i32>;
                    let mut roots: metamodelica::List<i32>;
                    let mut constraints: metamodelica::List<i32>;
                    let mut constraintresidual: metamodelica::List<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut subsyst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut colummarks: metamodelica::Array<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut orowmarks: metamodelica::Array<i32>;
                    let mut ocolummarks: metamodelica::Array<i32>;
                    let mut size: i32;
                    let mut mark: i32;
                    let mut esize: i32;
                    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut vars: BackendDAE::Variables;
                    let mut tvars: BackendDAE::Variables;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut teqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut m1: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mc: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mct: metamodelica::Array<metamodelica::List<i32>>;
                    let mut beqs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut matrix: metamodelica::Array<metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>>;
                    let mut crefexps: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
                    let mut crefexplst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut vorphansarray1: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                    let mut ass22: metamodelica::Array<metamodelica::List<i32>>;
                    let mut vec1: metamodelica::Array<metamodelica::List<i32>>;
                    let mut neweqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut shared = (*shared).clone();
                    let mut jac = (*jac).clone();
                    metamodelica::print(literal!("try to relax\n"));
                    Util::profilerinit()?;
                    Util::profilerstart2()?;
                    Util::profilerstart1()?;
                    size = ((vindx).len() as i32);
                    esize = ((eindex).len() as i32);
                    ass1 = arrayCreate(size, -1);
                    ass2 = arrayCreate(size, -1);
                    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst))?;
                    eqns = BackendEquation::listEquation(&eqn_lst)?;
                    var_lst = List::map1r(vindx.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), BackendVariable::daeVars(isyst))?;
                    vars = BackendVariable::listVar1(&var_lst)?;
                    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (subsyst, m, mt, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(subsyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(&ishared))?;
                    (_, ass1, ass2) = List::fold1(&eqn_lst, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: BackendDAE::Variables, __a2: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)| vectorMatching(&__a0, &__a1, __a2), vars.clone(), (1, ass1.clone(), ass2.clone()))?;
                    (_, ass1, ass2) = List::fold1(&eqn_lst, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: BackendDAE::Variables, __a2: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)| aliasMatching(&__a0, &__a1, __a2), vars.clone(), (1, ass1.clone(), ass2.clone()))?;
                    m1 = arrayCreate(size, metamodelica::nil());
                    transformJacToAdjacencyMatrix2(metamodelica::AsArg::as_arg(&jac), m1.clone(), mapIncRowEqn.clone(), eqns.clone(), ass1.clone(), ass2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isConstOneMinusOne(&__a0)) })?;
                    Matching::matchingExternalsetAdjacencyMatrix(size, size, m1.clone())?;
                    let true = (BackendDAEEXT::setAssignment(size, size, ass2.clone(), ass1.clone())) else { return Err("pattern mismatch") };
                    BackendDAEEXT::matching(size, size, 5, -1, metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    m1 = arrayCreate(size, metamodelica::nil());
                    transformJacToAdjacencyMatrix1(metamodelica::AsArg::as_arg(&jac), m1.clone(), ass1.clone(), ass2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isConstOneMinusOne(&__a0)) })?;
                    Matching::matchingExternalsetAdjacencyMatrix(size, size, m1.clone())?;
                    let true = (BackendDAEEXT::setAssignment(size, size, ass2.clone(), ass1.clone())) else { return Err("pattern mismatch") };
                    BackendDAEEXT::matching(size, size, 1, -1, metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    unassigned = Matching::getUnassigned(size, ass2.clone(), metamodelica::nil())?;
                    colummarks = arrayCreate(size, -1);
                    onefreeMatchingBFS(&unassigned, m.clone(), mt.clone(), size, ass1.clone(), ass2.clone(), colummarks.clone(), 1, &(metamodelica::nil()))?;
                    Util::profilerstop1()?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Matching  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vorphans = getOrphans(1, size, ass1.clone(), metamodelica::nil())?;
                    eorphans = getOrphans(1, size, ass2.clone(), metamodelica::nil())?;
                    ass1 = BackendDAETransform::varAssignmentNonScalar(ass1.clone(), mapIncRowEqn.clone())?;
                    ass22 = BackendDAETransform::eqnAssignmentNonScalar(mapEqnIncRow.clone(), ass2.clone())?;
                    eorphans = List::uniqueIntN(&(List::map1r(eorphans.clone(), &arrayGet, mapIncRowEqn.clone())?), metamodelica::arrayLength(mapIncRowEqn.clone()))?;
                    (subsyst, m, mt) = BackendDAEUtil::getAdjacencyMatrix(subsyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(&ishared))?;
                    rowmarks = arrayCreate(size, -1);
                    colummarks = arrayCreate(size, -1);
                    orowmarks = arrayCreate(size, -1);
                    ocolummarks = arrayCreate(size, -1);
                    vorphansarray1 = arrayCreate(size, metamodelica::nil());
                    mc = arrayCreate(esize, metamodelica::nil());
                    mct = arrayCreate(size, metamodelica::nil());
                    mc = Array::copy(m.clone(), mc.clone())?;
                    mct = Array::copy(mt.clone(), mct.clone())?;
                    mark = 1;
                    (mark, constraintresidual) = generateCliquesResidual(&eorphans, ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), &vars, &(metamodelica::nil()));
                    (mark, roots, constraints) = prepairOrphansOrder(&vorphans, ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), vorphansarray1.clone(), &vars, metamodelica::nil(), metamodelica::nil())?;
                    mark = prepairOrphansOrder2(&vorphans, ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), vorphansarray1.clone());
                    Util::profilerstop1()?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Identifikation  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vorphansarray1 = arrayCreate(size, metamodelica::nil());
                    List::map2_0(&roots, &doMark, rowmarks.clone(), mark)?;
                    List::map2_0(&constraints, &doMark, rowmarks.clone(), mark)?;
                    otherorphans = List::select2(vorphans.clone(), (std::sync::Arc::new(unmarked) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark)?;
                    mark = getOrphansOrderEdvanced(&otherorphans, ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), vorphansarray1.clone());
                    List::map2_0(&otherorphans, &move |__a0: i32, __a1: metamodelica::Array<metamodelica::List<i32>>, __a2: metamodelica::List<i32>| -> metamodelica::Result<_> { ::std::result::Result::Ok(removeRootConnections(__a0, __a1, &__a2)) }, vorphansarray1.clone(), roots.clone())?;
                    mark = getConstraintesOrphansOrderEdvanced(&constraints, ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), vorphansarray1.clone());
                    (vorphans, mark) = getOrphansOrderEdvanced3(&roots, &otherorphans, &constraints, vorphans.clone(), vorphansarray1.clone(), mark, rowmarks.clone())?;
                    Util::profilerstop1()?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Reihenfolge  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    List::map2_0(&constraints, &doMark, rowmarks.clone(), mark)?;
                    otherorphans = List::select2(vorphans.clone(), (std::sync::Arc::new(unmarked) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark)?;
                    List::map2_0(&constraintresidual, &doAssign, ass22.clone(), list![-1])?;
                    mark = getOrphansPairs(&otherorphans, ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone());
                    List::map2_0(&constraintresidual, &doAssign, ass22.clone(), metamodelica::nil())?;
                    mark = getOrphansPairsConstraints(&constraints, ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), eqns.clone());
                    Util::profilerstop1()?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Paarung  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vec1 = arrayCreate(esize, metamodelica::nil());
                    vec2 = arrayCreate(esize, -1);
                    orowmarks = List::fold1(&vorphans, &markOrphans, 1, orowmarks.clone())?;
                    ocolummarks = List::fold1(&eorphans, &markOrphans, 1, ocolummarks.clone())?;
                    mark = getIndexesForEqnsAdvanced(&vorphans, 1, m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass22.clone(), vec1.clone(), vec2.clone(), arrayCreate(esize, false), &vars, eqns.clone(), metamodelica::AsArg::as_arg(&shared), size);
                    (_, _, _, eqns, vars) = Array::fold(vec2.clone(), &move |__a0: i32, __a1: (metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, BackendDAE::Variables)| getEqnsinOrder(__a0, &__a1), (eqns.clone(), vars.clone(), ass22.clone(), BackendEquation::listEquation(&(metamodelica::nil()))?, BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    Util::profilerstop1()?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Indizierung  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (subsyst, m, _) = BackendDAEUtil::getAdjacencyMatrix(subsyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(&ishared))?;
                    let __pa0 = ::match_deref::match_deref! { match &(SymbolicJacobian::calculateJacobian(vars.clone(), eqns.clone(), m.clone(), true, ishared.clone())) {
                        (Some(__pa0), _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    jac = metamodelica::Own::own(__pa0);
                    (beqs, _) = BackendDAEUtil::getEqnSysRhs(eqns.clone(), vars.clone(), Some(funcs.clone()))?;
                    beqs = beqs.clone().reverse();
                    matrix = arrayCreate(size, metamodelica::nil());
                    transformJacToMatrix(metamodelica::AsArg::as_arg(&jac), 1, 1, size, &beqs, matrix.clone())?;
                    (tvars, teqns) = gaussElimination(1, size, matrix.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), BackendEquation::listEquation(&(metamodelica::nil()))?, (1, 1))?;
                    eqn_lst = BackendEquation::equationList(teqns.clone())?;
                    var_lst = BackendVariable::varList(&tvars)?;
                    syst = List::fold(&eqn_lst, &BackendEquation::equationAddDAE, isyst.clone())?;
                    syst = List::fold(&var_lst, &BackendVariable::addVarDAE, syst.clone())?;
                    crefexplst = List::map(BackendVariable::varList(&vars)?, &move |__a0: metamodelica::Ref<BackendDAE::Var>| makeCrefExps(&__a0))?;
                    crefexps = metamodelica::arrayFromVec(crefexplst.clone().into_iter().cloned().collect());
                    neweqns = makeGausElimination(1, size, matrix.clone(), crefexps.clone(), metamodelica::nil())?;
                    Util::profilerstop1()?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Gaus Elimination time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    syst = replaceEquationsAddNew(eindex.clone(), neweqns.clone(), syst.clone())?;
                    Util::profilerstop2()?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Gesamt  time: ")); __mm_s.push_str(&*realString(Util::profilertime2())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    metamodelica::print(literal!("Ok system relaxed\n"));
                    (syst, shared, _) = relaxSystem1(&syst, shared.clone(), metamodelica::AsArg::as_arg(&comps))?;
                    Ok((syst.clone(), shared.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: comps }) => {
                    let mut b: bool;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    (syst, shared, b) = relaxSystem1(isyst, ishared.clone(), metamodelica::AsArg::as_arg(&comps))?;
                    Ok((syst.clone(), shared.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outRunMatching))
}

fn removeRootConnections(
    mut orphan: i32,
    mut orphansarray: metamodelica::Array<metamodelica::List<i32>>,
    mut roots: &metamodelica::List<i32>,
) -> () {
    let () = 'mc: {
        let __mc_input = &**roots;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lst: metamodelica::List<i32>;
                    lst = ({let __elt = (*metamodelica::index_checked(&orphansarray.borrow(), orphan)?).clone(); __elt});
                    let true = (intGt(((lst).len() as i32), 1)) else { return Err("pattern mismatch") };
                    lst = List::fold1(roots, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst.clone())?;
                    metamodelica::arrayUpdate(orphansarray.clone(), orphan, lst.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn replaceFinalParameter(
    mut itpl: &(metamodelica::Ref<DAE::Exp>, BackendDAE::Variables),
) -> Result<(metamodelica::Ref<DAE::Exp>, BackendDAE::Variables)> {
    let mut outTpl: (metamodelica::Ref<DAE::Exp>, BackendDAE::Variables);
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut knvars: BackendDAE::Variables;
    let mut b: bool;
    (e, knvars) = itpl.clone();
    let (__pa0, (__pa1, __pa2)) = Expression::traverseExpBottomUp(
        e,
        &fnptr!(
            traverserExpreplaceFinalParameter,
            metamodelica::Ref<DAE::Exp>,
            (BackendDAE::Variables, bool)
        ),
        (knvars, false),
    )?;
    e = metamodelica::Own::own(__pa0);
    knvars = metamodelica::Own::own(__pa1);
    b = metamodelica::Own::own(__pa2);
    (e, _) = ExpressionSimplify::condsimplify(b, e)?;
    outTpl = (e, knvars);
    Ok(outTpl)
}

fn traverserExpreplaceFinalParameter(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (BackendDAE::Variables, bool),
) -> (metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, bool);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (&*inExp, &tpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (knvars, _)) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&knvars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    v = metamodelica::Own::own(__pa0);
                    let true = (BackendVariable::isFinalVar(&v)) else { return Err("pattern mismatch") };
                    e1 = BackendVariable::varBindExpStartValue(&v)?;
                    Ok((e1.clone(), (knvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn replaceEquationsAddNew(
    mut inEqnIndxes: metamodelica::List<i32>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEqnIndxes, inEqns.clone(), inEqSystem.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok(BackendEquation::equationsAddDAE(&inEqns, inEqSystem)?)
            },
            (Deref @ metamodelica::ListNode::Cons { head: index, tail: indices }, Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns }, Deref @ BackendDAE::EqSystem { orderedEqs, .. }) => {
                let mut eqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
                eqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem, BackendEquation::setAtIndex(orderedEqs.clone(), index.clone(), eqn.clone())?);
                { (inEqnIndxes, inEqns, inEqSystem) = (indices.clone(), eqns.clone(), eqSystem); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn dumpVar(mut id: i32, mut vars: &BackendDAE::Variables) -> Result<()> {
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    v = BackendVariable::getVarAt(vars, id)?;
    metamodelica::print(ComponentReferenceBasics::printComponentRefStr(
        &(BackendVariable::varCref(&v)),
    )?);
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn transposeOrphanVec(mut c: i32, mut vec3: metamodelica::Array<metamodelica::List<i32>>, mut inId: i32) -> i32 {
    let mut outId: i32;
    outId = 'mc: {
        let __mc_input = inId;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut lst: metamodelica::List<i32>;
            let true = (intGt(c, 0)) else {
                return Err("pattern mismatch");
            };
            lst = ({
                let __elt = (*metamodelica::index_checked(&vec3.borrow(), c)?).clone();
                __elt
            });
            metamodelica::arrayUpdate(vec3.clone(), c, metamodelica::cons(inId, lst.clone()))?;
            Ok(inId + 1)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inId + 1)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outId
}

fn markOrphans(mut o: i32, mut mark: i32, mut rowmark: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut orowmark: metamodelica::Array<i32>;
    orowmark = metamodelica::arrayUpdate(rowmark.clone(), o, mark)?;
    Ok(orowmark)
}

fn generateCliquesResidual(
    mut inOrphans: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut vars: &BackendDAE::Variables,
    mut iconstraints: &metamodelica::List<i32>,
) -> (i32, metamodelica::List<i32>) {
    let mut omark: i32 = 0;
    let mut oconstraints: metamodelica::List<i32>;
    (omark, oconstraints) = 'mc: {
        let __mc_input = &**inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((mark + 2, iconstraints.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } => {
                    let mut constraints: metamodelica::List<i32>;
                    let mut rlst: metamodelica::List<i32>;
                    let mut elst: metamodelica::List<i32>;
                    let mut partner: metamodelica::List<i32>;
                    let mut foundflow: bool;
                    let mut blst: metamodelica::List<bool>;
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut omark: i32 = omark.clone();
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), o.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(colummarks.clone(), o.clone(), mark)?;
                    rlst = ({let __elt = (*metamodelica::index_checked(&m.borrow(), o.clone())?).clone(); __elt});
                    elst = List::select1(List::flatten(List::map1r(rlst.clone(), &arrayGet, mt.clone())?)?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    partner = List::select1(elst.clone(), (std::sync::Arc::new(isResOrphan) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<metamodelica::List<i32>>) -> Result<bool> + 'static>), ass2.clone())?;
                    partner = List::uniqueIntN(&(List::removeOnTrue(o.clone(), &fnptr!(intEq, i32, i32), partner.clone())?), metamodelica::arrayLength(colummarks.clone()))?;
                    List::map2_0(&partner, &doMark, colummarks.clone(), mark)?;
                    vlst = List::map1r(rlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    blst = List::map(vlst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isFlowVar(&__a0)) })?;
                    foundflow = List::any(&blst, &fnptr!(Util::id, _))?;
                    rlst = selectNonFlows(&rlst, blst.clone())?;
                    foundflow = generateCliquesResidual1(&rlst, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), foundflow, vars.clone())?;
                    generateCliquesResidual2(&rlst, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), &(metamodelica::cons(o.clone(), partner.clone())))?;
                    constraints = if (!(foundflow)) {listAppend(metamodelica::cons(o.clone(), partner.clone()), iconstraints.clone())} else {iconstraints.clone()};
                    (omark, constraints) = generateCliquesResidual(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), vars, &constraints);
                    Ok(((omark, constraints.clone()), omark.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            omark = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut constraints: metamodelica::List<i32>;
                    let mut omark: i32 = omark.clone();
                    (omark, constraints) = generateCliquesResidual(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), vars, iconstraints);
                    Ok(((omark, constraints.clone()), omark.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            omark = __wb0;
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (omark, oconstraints)
}

fn generateCliquesResidual1(
    mut rows: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut ifoundFlow: bool,
    mut vars: BackendDAE::Variables,
) -> Result<bool> {
    let mut ofoundFlow: bool = ifoundFlow;
    let mut e: i32;
    let mut next: metamodelica::List<i32>;
    let mut rlst: metamodelica::List<i32>;
    let mut b1: bool;
    let mut blst: metamodelica::List<bool>;
    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    for mut r in &**rows {
        if !(intEq(
            ({
                let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone();
                __elt
            }),
            mark,
        )) {
            next = List::select1(
                ({
                    let __elt = (*metamodelica::index_checked(&mt.borrow(), r.clone())?).clone();
                    __elt
                }),
                (std::sync::Arc::new(isNoResOrphan)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(i32, metamodelica::Array<metamodelica::List<i32>>) -> Result<bool> + 'static,
                    >),
                ass2.clone(),
            )?;
            next = List::select2(
                next,
                (std::sync::Arc::new(unmarked)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static,
                    >),
                colummarks.clone(),
                mark,
            )?;
            next = List::removeOnTrue(
                ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), r.clone())?).clone();
                    __elt
                }),
                &fnptr!(intEq, i32, i32),
                next,
            )?;
            if (next).is_empty() {
                metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), mark)?;
                e = ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), r.clone())?).clone();
                    __elt
                });
                metamodelica::arrayUpdate(colummarks.clone(), e, mark)?;
                rlst = ({
                    let __elt = (*metamodelica::index_checked(&ass2.borrow(), e)?).clone();
                    __elt
                });
                next = List::fold1(
                    &rlst,
                    &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2),
                    (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                    ({
                        let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone();
                        __elt
                    }),
                )?;
                vlst = List::map1r(
                    next.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    vars.clone(),
                )?;
                blst = List::map(
                    vlst,
                    &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isFlowVar(&__a0))
                    },
                )?;
                b1 = List::any(&blst, &fnptr!(Util::id, _))?;
                next = selectNonFlows(&next, blst)?;
                ofoundFlow = generateCliquesResidual1(
                    &next,
                    ass1.clone(),
                    ass2.clone(),
                    m.clone(),
                    mt.clone(),
                    mark,
                    rowmarks.clone(),
                    colummarks.clone(),
                    b1 || ofoundFlow,
                    vars.clone(),
                )?;
            }
        }
    }
    Ok(ofoundFlow)
}

fn selectNonFlows(
    mut rows: &metamodelica::List<i32>,
    mut flowFlag: metamodelica::List<bool>,
) -> Result<metamodelica::List<i32>> {
    let mut oAcc: metamodelica::List<i32> = metamodelica::nil();
    let mut brest: metamodelica::List<bool> = flowFlag;
    let mut b: bool;
    for mut r in &**rows {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(brest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        b = metamodelica::Own::own(__pa0);
        brest = metamodelica::Own::own(__pa1);
        if !(b) {
            oAcc = metamodelica::cons(r.clone(), oAcc);
        }
    }
    Ok(oAcc)
}

fn generateCliquesResidual2(
    mut eqns: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut orphan: &metamodelica::List<i32>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match eqns {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } if (!(intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), mark))) => {
            let mut e: i32;
            let mut lst: metamodelica::List<i32>;
            let mut rlst: metamodelica::List<i32>;
            let mut lst1: metamodelica::List<i32>;
            e = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), r.clone())?).clone(); __elt});
            rlst = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e)?).clone(); __elt});
            lst = List::fold1(&rlst, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ({let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone(); __elt}))?;
            let __pa0 = ::match_deref::match_deref! { match &(List::select2(lst.clone(), (std::sync::Arc::new(unmarked) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark - 1)?) {
                __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            lst1 = metamodelica::Own::own(__pa0);
            List::map4_0(&lst1, &move |__a0: i32, __a1: metamodelica::Array<metamodelica::List<i32>>, __a2: metamodelica::Array<metamodelica::List<i32>>, __a3: metamodelica::List<i32>, __a4: i32| generateResidualClique(__a0, __a1, __a2, &__a3, __a4), m.clone(), mt.clone(), orphan.clone(), e)?;
            List::map2_0(&rlst, &doMark, rowmarks.clone(), mark)?;
            lst = List::select2(lst, (std::sync::Arc::new(marked) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark - 1)?;
            metamodelica::arrayUpdate(colummarks.clone(), e, mark)?;
            generateCliquesResidual2(&lst, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan)?;
            generateCliquesResidual2(rest, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            generateCliquesResidual2(rest, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn prepairOrphansOrder<'__b>(
    mut inOrphans: &'__b metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut orphans: metamodelica::Array<metamodelica::List<i32>>,
    mut vars: &'__b BackendDAE::Variables,
    mut iroots: metamodelica::List<i32>,
    mut iconstraints: metamodelica::List<i32>,
) -> Result<(i32, metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut omark: i32;
    let mut oroots: metamodelica::List<i32>;
    let mut oconstraints: metamodelica::List<i32>;
    (omark, oroots, oconstraints) = (::match_deref::match_deref! { match inOrphans {
        Deref @ metamodelica::ListNode::Nil => {
            (mark, iroots, iconstraints)
        },
        Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } if (!(intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), o.clone())?).clone(); __elt}), mark))) => {
            let mut roots: metamodelica::List<i32>;
            let mut constraints: metamodelica::List<i32>;
            let mut elst: metamodelica::List<i32>;
            let mut rlst: metamodelica::List<i32>;
            let mut foundflow: bool;
            let mut constr: bool;
            let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
            elst = ({let __elt = (*metamodelica::index_checked(&mt.borrow(), o.clone())?).clone(); __elt});
            rlst = List::flatten(List::map1r(elst, &arrayGet, ass2.clone())?)?;
            vlst = List::map1r(rlst, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
            constr = List::all(&vlst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isFlowVar(&__a0)) })?;
            constraints = List::consOnTrue(constr, o.clone(), iconstraints);
            foundflow = prepairOrphansOrder1(&(({let __elt = (*metamodelica::index_checked(&mt.borrow(), o.clone())?).clone(); __elt})), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), &(list![o.clone()]), false, vars.clone())?;
            roots = List::consOnTrue(foundflow && !(constr), o.clone(), iroots);
            (omark, roots, constraints) = prepairOrphansOrder(rest, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), orphans.clone(), vars, roots, constraints)?;
            (omark, roots, constraints)
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            let mut roots: metamodelica::List<i32>;
            let mut constraints: metamodelica::List<i32>;
            (omark, roots, constraints) = prepairOrphansOrder(rest, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphans.clone(), vars, iroots, iconstraints)?;
            (omark, roots, constraints)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((omark, oroots, oconstraints))
}

fn prepairOrphansOrder1(
    mut eqns: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut preorphan: i32,
    mut orphans: metamodelica::Array<metamodelica::List<i32>>,
    mut prer: &metamodelica::List<i32>,
    mut ifoundFlow: bool,
    mut vars: BackendDAE::Variables,
) -> Result<bool> {
    let mut ofoundFlow: bool = ifoundFlow;
    let mut next: metamodelica::List<i32>;
    let mut r: metamodelica::List<i32>;
    let mut elst: metamodelica::List<i32>;
    let mut b1: bool;
    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    for mut e in &**eqns {
        if !(intEq(
            ({
                let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone();
                __elt
            }),
            mark,
        )) {
            next = List::select1(
                ({
                    let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone();
                    __elt
                }),
                (std::sync::Arc::new(isNoOrphan)
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>),
                ass1.clone(),
            )?;
            next = List::select2(
                next,
                (std::sync::Arc::new(unmarked)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static,
                    >),
                rowmarks.clone(),
                mark,
            )?;
            next = List::fold1(
                &({
                    let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone();
                    __elt
                }),
                &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2),
                (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                next,
            )?;
            if (next).is_empty() {
                metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                r = ({
                    let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone();
                    __elt
                });
                List::map2_0(&r, &doMark, rowmarks.clone(), mark)?;
                elst = List::select1(
                    List::map1r(r.clone(), &arrayGet, ass1.clone())?,
                    (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                    0,
                )?;
                next = List::flatten(List::map1r(r.clone(), &arrayGet, mt.clone())?)?;
                next = List::fold1(
                    &elst,
                    &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2),
                    (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                    next,
                )?;
                List::map2_0(&r, &addPreOrphan, preorphan, orphans.clone())?;
                vlst = List::map1r(
                    r.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    vars.clone(),
                )?;
                b1 = List::any(
                    &vlst,
                    &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isFlowVar(&__a0))
                    },
                )?;
                ofoundFlow = prepairOrphansOrder1(
                    &next,
                    ass1.clone(),
                    ass2.clone(),
                    m.clone(),
                    mt.clone(),
                    mark,
                    rowmarks.clone(),
                    colummarks.clone(),
                    preorphan,
                    orphans.clone(),
                    &r,
                    b1 || ofoundFlow,
                    vars.clone(),
                )?;
            }
        }
    }
    Ok(ofoundFlow)
}

fn prepairOrphansOrder2(
    mut inOrphans: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut imark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut orphans: metamodelica::Array<metamodelica::List<i32>>,
) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = &**inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(imark + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } => {
                    let mut elst: metamodelica::List<i32>;
                    let mut rlst: metamodelica::List<i32>;
                    let mut partner: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), o.clone())?).clone(); __elt}), imark)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), imark)?;
                    elst = List::select1(({let __elt = (*metamodelica::index_checked(&mt.borrow(), o.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    rlst = List::select1(List::flatten(List::map1r(elst.clone(), &arrayGet, m.clone())?)?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    partner = List::select1(rlst.clone(), (std::sync::Arc::new(isOrphan) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), ass1.clone())?;
                    partner = List::unique(&partner);
                    List::map2_0(&partner, &doMark, rowmarks.clone(), imark)?;
                    prepairOrphansOrder3(&(({let __elt = (*metamodelica::index_checked(&mt.borrow(), o.clone())?).clone(); __elt})), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark, rowmarks.clone(), colummarks.clone(), o.clone(), &partner, orphans.clone(), &(list![o.clone()]));
                    Ok(prepairOrphansOrder2(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(prepairOrphansOrder2(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn prepairOrphansOrder3(
    mut eqns: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut preorphan: i32,
    mut partner: &metamodelica::List<i32>,
    mut orphans: metamodelica::Array<metamodelica::List<i32>>,
    mut prer: &metamodelica::List<i32>,
) -> () {
    let () = 'mc: {
        let __mc_input = &**eqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
                    let mut next: metamodelica::List<i32>;
                    let mut r: metamodelica::List<i32>;
                    let mut elst: metamodelica::List<i32>;
                    let mut lst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    r = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone(); __elt});
                    lst = List::unique(&(List::flatten(List::map1r(r.clone(), &arrayGet, orphans.clone())?)?));
                    let true = (listMember(preorphan, lst.clone())) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    List::map2_0(&r, &doMark, rowmarks.clone(), mark)?;
                    elst = List::select1(List::map1r(r.clone(), &arrayGet, ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = List::flatten(List::map1r(r.clone(), &arrayGet, mt.clone())?)?;
                    next = List::fold1(&elst, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone())?;
                    prepairOrphansOrder3(&next, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, partner, orphans.clone(), &r);
                    prepairOrphansOrder3(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, partner, orphans.clone(), prer);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    List::map4_0(prer, &move |__a0: i32, __a1: metamodelica::Array<metamodelica::List<i32>>, __a2: metamodelica::Array<metamodelica::List<i32>>, __a3: metamodelica::List<i32>, __a4: i32| generateClique(__a0, __a1, __a2, &__a3, __a4), m.clone(), mt.clone(), partner.clone(), e.clone())?;
                    prepairOrphansOrder3(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, partner, orphans.clone(), prer);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    prepairOrphansOrder3(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, partner, orphans.clone(), prer);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn generateClique(
    mut r: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut orphans: &metamodelica::List<i32>,
    mut e: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match orphans {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: orphan, tail: rest } => {
            let mut lst: metamodelica::List<i32>;
            lst = ({let __elt = (*metamodelica::index_checked(&mt.borrow(), r)?).clone(); __elt});
            lst = List::removeOnTrue(e, &fnptr!(intEq, i32, i32), lst)?;
            metamodelica::arrayUpdate(mt.clone(), r, lst)?;
            lst = ({let __elt = (*metamodelica::index_checked(&mt.borrow(), orphan.clone())?).clone(); __elt});
            lst = List::unique(&(metamodelica::cons(e, lst)));
            metamodelica::arrayUpdate(mt.clone(), orphan.clone(), lst)?;
            lst = ({let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone(); __elt});
            lst = List::removeOnTrue(r, &fnptr!(intEq, i32, i32), lst)?;
            lst = List::unique(&(metamodelica::cons(orphan.clone(), lst)));
            metamodelica::arrayUpdate(m.clone(), e, lst)?;
            generateClique(r, m.clone(), mt.clone(), rest, e)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn generateResidualClique(
    mut r: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut orphans: &metamodelica::List<i32>,
    mut e: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match orphans {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: orphan, tail: rest } => {
            let mut lst: metamodelica::List<i32>;
            lst = ({let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone(); __elt});
            lst = List::removeOnTrue(r, &fnptr!(intEq, i32, i32), lst)?;
            metamodelica::arrayUpdate(m.clone(), e, lst)?;
            lst = ({let __elt = (*metamodelica::index_checked(&m.borrow(), orphan.clone())?).clone(); __elt});
            lst = List::unique(&(metamodelica::cons(r, lst)));
            metamodelica::arrayUpdate(m.clone(), orphan.clone(), lst)?;
            lst = ({let __elt = (*metamodelica::index_checked(&mt.borrow(), r)?).clone(); __elt});
            lst = List::removeOnTrue(e, &fnptr!(intEq, i32, i32), lst)?;
            lst = List::unique(&(metamodelica::cons(orphan.clone(), lst)));
            metamodelica::arrayUpdate(mt.clone(), r, lst)?;
            generateResidualClique(r, m.clone(), mt.clone(), rest, e)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn getOrphansOrderEdvanced(
    mut inOrphans: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mc: metamodelica::Array<metamodelica::List<i32>>,
    mut mct: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut orphans: metamodelica::Array<metamodelica::List<i32>>,
) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = &**inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(mark)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } => {
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), o.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
                    getOrphansOrderEdvanced1(&({let __elt = (*metamodelica::index_checked(&mct.borrow(), o.clone())?).clone(); __elt}), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), &(metamodelica::nil()))?;
                    Ok(getOrphansOrderEdvanced(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getOrphansOrderEdvanced(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn hasOrphanAdvanced(
    mut rows: metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((rows, iAcc.clone())) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                return Ok(iAcc)
            },
            (Deref @ metamodelica::ListNode::Cons { head: r, tail: rest }, _) => {
                if (!(intGt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), r.clone())?).clone(); __elt}), 0))) {{ (rows, ass1, iAcc) = (rest.clone(), ass1.clone(), metamodelica::cons(r.clone(), iAcc)); continue '__tco; }} else {{ (rows, ass1, iAcc) = (rest.clone(), ass1.clone(), iAcc); continue '__tco; }}
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn addPreOrphan(
    mut orphan: i32,
    mut preorphan: i32,
    mut arr: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut olst: metamodelica::List<i32>;
    olst = ({
        let __elt = (*metamodelica::index_checked(&arr.borrow(), orphan)?).clone();
        __elt
    });
    olst = List::unionElt(preorphan, olst);
    metamodelica::arrayUpdate(arr.clone(), orphan, olst)?;
    Ok(())
}

fn addPreOrphans(
    mut orphan: i32,
    mut preorphans: &metamodelica::List<i32>,
    mut arr: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match preorphans {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } => {
            addPreOrphan(orphan, o.clone(), arr.clone())?;
            addPreOrphans(orphan, rest, arr.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn getOrphansOrderEdvanced1(
    mut eqns: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut preorphan: i32,
    mut orphans: metamodelica::Array<metamodelica::List<i32>>,
    mut nextQueue: &metamodelica::List<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**eqns, &**nextQueue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    getOrphansOrderEdvanced1(nextQueue, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), &(metamodelica::nil()))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: _ }, _) => {
                    let mut r: metamodelica::List<i32>;
                    let mut olst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    r = List::removeOnTrue(preorphan, &fnptr!(intEq, i32, i32), ({let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone(); __elt}))?;
                    olst = hasOrphanAdvanced(r.clone(), ass1.clone(), metamodelica::nil())?;
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    addPreOrphans(preorphan, &olst, orphans.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, _) => {
                    let mut next: metamodelica::List<i32>;
                    let mut r: metamodelica::List<i32>;
                    let mut r1: metamodelica::List<i32>;
                    let mut elst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    r = List::removeOnTrue(preorphan, &fnptr!(intEq, i32, i32), ({let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone(); __elt}))?;
                    r1 = List::select1(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    r = List::fold1(&r1, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone())?;
                    elst = List::select1(List::map1r(r.clone(), &arrayGet, ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), elst.clone());
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    getOrphansOrderEdvanced1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), &next)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    getOrphansOrderEdvanced1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), nextQueue)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn getConstraintesOrphansOrderEdvanced(
    mut inOrphans: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mc: metamodelica::Array<metamodelica::List<i32>>,
    mut mct: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut orphans: metamodelica::Array<metamodelica::List<i32>>,
) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = &**inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(mark)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } => {
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), o.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
                    getConstraintesOrphansOrderEdvanced1(&({let __elt = (*metamodelica::index_checked(&mct.borrow(), o.clone())?).clone(); __elt}), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), &(metamodelica::nil()))?;
                    Ok(getConstraintesOrphansOrderEdvanced(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getConstraintesOrphansOrderEdvanced(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark, rowmarks.clone(), colummarks.clone(), orphans.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn getConstraintesOrphansOrderEdvanced1(
    mut eqns: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut preorphan: i32,
    mut orphans: metamodelica::Array<metamodelica::List<i32>>,
    mut nextQueue: &metamodelica::List<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**eqns, &**nextQueue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    getConstraintesOrphansOrderEdvanced1(nextQueue, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), &(metamodelica::nil()))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, _) => {
                    let mut next: metamodelica::List<i32>;
                    let mut r: metamodelica::List<i32>;
                    let mut r1: metamodelica::List<i32>;
                    let mut elst: metamodelica::List<i32>;
                    let mut olst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    r = List::removeOnTrue(preorphan, &fnptr!(intEq, i32, i32), ({let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone(); __elt}))?;
                    olst = hasOrphanAdvanced(r.clone(), ass1.clone(), metamodelica::nil())?;
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    addPreOrphans(preorphan, &olst, orphans.clone())?;
                    r1 = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone(); __elt});
                    r = List::fold1(&r1, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone())?;
                    elst = List::select1(List::map1r(r.clone(), &arrayGet, ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), elst.clone());
                    getConstraintesOrphansOrderEdvanced1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), &next)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, _) => {
                    let mut next: metamodelica::List<i32>;
                    let mut r: metamodelica::List<i32>;
                    let mut r1: metamodelica::List<i32>;
                    let mut elst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    r = List::removeOnTrue(preorphan, &fnptr!(intEq, i32, i32), ({let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone(); __elt}))?;
                    r1 = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone(); __elt});
                    r = List::fold1(&r1, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone())?;
                    elst = List::select1(List::map1r(r.clone(), &arrayGet, ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), elst.clone());
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    getConstraintesOrphansOrderEdvanced1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), &next)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    getConstraintesOrphansOrderEdvanced1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), preorphan, orphans.clone(), nextQueue)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn mergeOrphanParents(
    mut links: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut iAcc: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oAcc: metamodelica::List<i32>;
    oAcc = 'mc: {
        let __mc_input = &**links;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(iAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: l, tail: rest } => {
                    ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&m.borrow(), l.clone())?).clone(); __elt})) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(mergeOrphanParents(metamodelica::AsArg::as_arg(&rest), m.clone(), iAcc)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: l, tail: rest } => {
                    let mut lst: metamodelica::List<i32>;
                    lst = ({let __elt = (*metamodelica::index_checked(&m.borrow(), l.clone())?).clone(); __elt});
                    Ok(mergeOrphanParents(metamodelica::AsArg::as_arg(&rest), m.clone(), &(listAppend(lst.clone(), iAcc.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oAcc)
}

fn getLinkPosition(
    mut orphans: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut iAcc: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut ochilds: metamodelica::List<i32>;
    ochilds = 'mc: {
        let __mc_input = &**orphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(iAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } => {
                    let mut childs: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), o.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
                    childs = getLinkPosition1(&(({let __elt = (*metamodelica::index_checked(&m.borrow(), o.clone())?).clone(); __elt})), m.clone(), mt.clone(), mark, rowmarks.clone(), o.clone(), iAcc)?;
                    Ok(getLinkPosition(metamodelica::AsArg::as_arg(&rest), m.clone(), mt.clone(), mark, rowmarks.clone(), &childs))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getLinkPosition(metamodelica::AsArg::as_arg(&rest), m.clone(), mt.clone(), mark, rowmarks.clone(), iAcc))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ochilds
}

fn getLinkPosition1(
    mut orphans: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut preorphan: i32,
    mut iAcc: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut childs: metamodelica::List<i32>;
    childs = 'mc: {
        let __mc_input = &**orphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::cons(preorphan, iAcc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), o.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), o.clone(), mark)?;
                    Ok(getLinkPosition1(&(({let __elt = (*metamodelica::index_checked(&m.borrow(), o.clone())?).clone(); __elt})), m.clone(), mt.clone(), mark, rowmarks.clone(), o.clone(), iAcc)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut lst: metamodelica::List<i32>;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), o.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    lst = listAppend(({let __elt = (*metamodelica::index_checked(&mt.borrow(), 0)?).clone(); __elt}), iAcc.clone());
                    Ok(lst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error in getLinkPosition1! Found Orphan with more than one parents ")); __mm_s.push_str(&*stringDelimitList(List::map(orphans.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(childs)
}

fn getOrphansOrderEdvanced5<'__b>(
    mut linklst: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut imark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<metamodelica::List<i32>>,
) -> Result<(metamodelica::List<metamodelica::List<i32>>, i32)> {
    '__tco: loop {
        ::match_deref::match_deref! { match linklst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iAcc.reverse(), imark))
            },
            Deref @ metamodelica::ListNode::Cons { head: links, tail: rest } => {
                let mut mark: i32;
                let mut lst: metamodelica::List<i32>;
                let mut childs: metamodelica::List<i32>;
                let mut acc: metamodelica::List<metamodelica::List<i32>>;
                lst = mergeOrphanParents(metamodelica::AsArg::as_arg(&links), m.clone(), &(metamodelica::nil()))?;
                childs = getLinkPosition(&lst, m.clone(), mt.clone(), imark, rowmarks.clone(), &(metamodelica::nil()));
                { (linklst, m, mt, imark, rowmarks, iAcc) = (rest, m.clone(), mt.clone(), imark + 1, rowmarks.clone(), metamodelica::cons(childs, iAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getOrphansOrderEdvanced6(
    mut linklst: &metamodelica::List<metamodelica::List<i32>>,
    mut childslst: &metamodelica::List<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (linklst, childslst) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: links, tail: rest }, Deref @ metamodelica::ListNode::Cons { head: childs, tail: acc }) => {
            let mut lst: metamodelica::List<i32>;
            lst = List::unique(&(List::flatten(List::map1r(childs.clone(), &arrayGet, m.clone())?)?));
            List::map2_0(metamodelica::AsArg::as_arg(&links), &doAssign, m.clone(), lst)?;
            List::map2_0(metamodelica::AsArg::as_arg(&childs), &doAssign, m.clone(), links.clone())?;
            getOrphansOrderEdvanced6(rest, acc, m.clone())?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn getOrphansOrderEdvanced4(
    mut linklst: &metamodelica::List<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut imark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut iorder: &metamodelica::List<i32>,
    mut iAcc: &metamodelica::List<i32>,
) -> Result<i32> {
    let mut omark: i32;
    let mut childs: metamodelica::List<metamodelica::List<i32>>;
    (childs, omark) = getOrphansOrderEdvanced5(
        linklst,
        m.clone(),
        mt.clone(),
        imark,
        rowmarks.clone(),
        metamodelica::nil(),
    )?;
    getOrphansOrderEdvanced6(linklst, &childs, m.clone())?;
    Ok(omark)
}

fn getInvMap(mut orphan: i32, mut invmap: metamodelica::Array<i32>, mut index: i32) -> Result<i32> {
    let mut oindex: i32;
    metamodelica::arrayUpdate(invmap.clone(), orphan, index)?;
    oindex = index + 1;
    Ok(oindex)
}

fn getOrphansAdjacencyMatrix(
    mut orphans: &metamodelica::List<i32>,
    mut invmap: metamodelica::Array<i32>,
    mut vorphansarray: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut addself: bool,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let __ab_vorphansarray = vorphansarray.borrow();
    let mut outM: metamodelica::Array<metamodelica::List<i32>>;
    let mut outMT: metamodelica::Array<metamodelica::List<i32>> = mT.clone();
    let mut m: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut lst: metamodelica::List<i32>;
    let mut i: i32;
    for mut o in &**orphans {
        lst = List::map1r(
            (*metamodelica::index_checked(&__ab_vorphansarray, o.clone())?).clone(),
            &arrayGet,
            invmap.clone(),
        )?;
        i = ({
            let __elt = (*metamodelica::index_checked(&invmap.borrow(), o.clone())?).clone();
            __elt
        });
        lst = List::consOnTrue(addself, i, lst);
        outMT = List::fold1(&lst, &Array::consToElement, i, outMT.clone())?;
        m = metamodelica::cons(lst, m);
    }
    outM = List::listArrayReverse(m)?;
    outMT = mT.clone();
    Ok((outM, outMT))
}

fn getOrder(
    mut comp: metamodelica::List<i32>,
    mut inorder: &(metamodelica::List<i32>, metamodelica::List<metamodelica::List<i32>>),
) -> (metamodelica::List<i32>, metamodelica::List<metamodelica::List<i32>>) {
    let mut outorder: (metamodelica::List<i32>, metamodelica::List<metamodelica::List<i32>>);
    outorder = (::match_deref::match_deref! { match &((comp.clone(), inorder.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: o, tail: Deref @ metamodelica::ListNode::Nil }, (order, links)) => {
            (metamodelica::cons(o.clone(), order.clone()), links.clone())
        },
        (_, (order, links)) => {
            (order.clone(), metamodelica::cons(comp, links.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outorder
}

fn getOrphansOrderEdvanced3(
    mut roots: &metamodelica::List<i32>,
    mut otherorphans: &metamodelica::List<i32>,
    mut constraints: &metamodelica::List<i32>,
    mut vorphans: metamodelica::List<i32>,
    mut vorphansarray: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut sortvorphans: metamodelica::List<i32>;
    let mut omark: i32;
    let mut order: metamodelica::List<i32>;
    let mut size: i32;
    let mut map: metamodelica::Array<i32>;
    let mut ass: metamodelica::Array<i32>;
    let mut invmap: metamodelica::Array<i32>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut range: metamodelica::List<i32>;
    let mut comps: metamodelica::List<metamodelica::List<i32>>;
    let mut linkslst: metamodelica::List<metamodelica::List<i32>>;
    map = metamodelica::arrayFromVec(vorphans.clone().into_iter().cloned().collect());
    size = metamodelica::arrayLength(map.clone());
    invmap = arrayCreate(metamodelica::arrayLength(vorphansarray.clone()), 0);
    List::fold1(&vorphans, &getInvMap, invmap.clone(), 1)?;
    range = List::intRange(size);
    (m, mt) = getOrphansAdjacencyMatrix(
        &vorphans,
        invmap.clone(),
        vorphansarray.clone(),
        arrayCreate(size, metamodelica::nil()),
        true,
    )?;
    ass = metamodelica::arrayFromVec(range.into_iter().cloned().collect());
    comps = Sorting::TarjanTransposed(mt.clone(), ass.clone())?;
    (order, linkslst) = List::fold(
        &comps,
        &move |__a0: metamodelica::List<i32>,
               __a1: (metamodelica::List<i32>, metamodelica::List<metamodelica::List<i32>>)|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(getOrder(__a0, &__a1)) },
        (metamodelica::nil(), metamodelica::nil()),
    )?;
    (m, mt) = getOrphansAdjacencyMatrix(
        &vorphans,
        invmap.clone(),
        vorphansarray.clone(),
        arrayCreate(size, metamodelica::nil()),
        false,
    )?;
    reduceOrphancMatrix(&(comps.reverse()), m.clone())?;
    omark = getOrphansOrderEdvanced4(
        &linkslst,
        m.clone(),
        mt.clone(),
        mark,
        rowmarks.clone(),
        &order,
        &(metamodelica::nil()),
    )?;
    mt = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), metamodelica::arrayLength(mt.clone()))?;
    comps = Sorting::TarjanTransposed(mt.clone(), ass.clone())?;
    sortvorphans = List::flattenReverse(comps)?;
    sortvorphans = List::map1r(sortvorphans, &arrayGet, map.clone())?;
    Ok((sortvorphans, omark))
}

fn reduceOrphancMatrix(
    mut comps: &metamodelica::List<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match comps {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, tail: rest } => {
            reduceOrphancMatrix(rest, m.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: comp, tail: rest } => {
            reduceOrphancMatrix1(metamodelica::AsArg::as_arg(&comp), metamodelica::AsArg::as_arg(&comp), m.clone())?;
            reduceOrphancMatrix(rest, m.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn reduceOrphancMatrix1(
    mut comps: &metamodelica::List<i32>,
    mut comps1: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match comps {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
            let mut lst: metamodelica::List<i32>;
            lst = ({let __elt = (*metamodelica::index_checked(&m.borrow(), c.clone())?).clone(); __elt});
            lst = List::setDifference(lst, comps1)?;
            metamodelica::arrayUpdate(m.clone(), c.clone(), lst.reverse())?;
            reduceOrphancMatrix1(rest, comps1, m.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn hasResidualOrphan1(
    mut eqns: &metamodelica::List<i32>,
    mut ass: metamodelica::Array<metamodelica::List<i32>>,
    mut eqnsarr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<i32> {
    let mut Orphan: i32;
    Orphan = 'mc: {
        let __mc_input = &**eqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: _ } => {
                    let mut len: i32;
                    let mut size: i32;
                    len = ((({let __elt = (*metamodelica::index_checked(&ass.borrow(), e.clone())?).clone(); __elt})).len() as i32);
                    size = BackendEquation::equationSize(&(BackendEquation::get(eqnsarr.clone(), e.clone())?))?;
                    let true = (intLt(len, size)) else { return Err("pattern mismatch") };
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(hasResidualOrphan1(metamodelica::AsArg::as_arg(&rest), ass.clone(), eqnsarr.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(Orphan)
}

fn hasResidualOrphan(
    mut eqns: &metamodelica::List<i32>,
    mut ass: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<i32> {
    let mut Orphan: i32;
    Orphan = 'mc: {
        let __mc_input = &**eqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: _ } => {
                    ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&ass.borrow(), e.clone())?).clone(); __elt})) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(hasResidualOrphan(metamodelica::AsArg::as_arg(&rest), ass.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(Orphan)
}

fn makeCrefExps(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    e = Expression::crefExp(BackendVariable::varCref(v))?;
    Ok(e)
}

fn makeGausEliminationRow<'__b>(
    mut lst: &'__b metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>,
    mut size: i32,
    mut vars: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match lst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((inExp, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))
            },
            Deref @ metamodelica::ListNode::Cons { head: (c, e), tail: _ } if (intGt(c.clone(), size)) => {
                return Ok((inExp, e.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: (c, e), tail: rest } => {
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut b: metamodelica::Ref<DAE::Exp>;
                e1 = Expression::expMul(e.clone(), ({let __elt = (*metamodelica::index_checked(&vars.borrow(), c.clone())?).clone(); __elt}))?;
                e1 = Expression::expAdd(e1, inExp)?;
                { (lst, size, vars, inExp) = (rest, size, vars.clone(), e1); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn makeGausElimination(
    mut row: i32,
    mut size: i32,
    mut matrix: metamodelica::Array<metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>>,
    mut vars: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut iAcc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    '__tco: loop {
        let mut e: metamodelica::Ref<DAE::Exp>;
        let mut b: metamodelica::Ref<DAE::Exp>;
        let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
        if intGt(row, size) {
            return Ok(iAcc.reverse());
        } else {
            (e, b) = makeGausEliminationRow(
                &({
                    let __elt = (*metamodelica::index_checked(&matrix.borrow(), row)?).clone();
                    __elt
                }),
                size,
                vars.clone(),
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                }),
            )?;
            eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: e,
                scalar: b,
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
            });
            {
                (row, size, matrix, vars, iAcc) = (
                    row + 1,
                    size,
                    matrix.clone(),
                    vars.clone(),
                    metamodelica::cons(eqn, iAcc),
                );
                continue '__tco;
            }
        }
    }
}

fn dumpMatrix(
    mut row: i32,
    mut size: i32,
    mut matrix: metamodelica::Array<metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>>,
) -> Result<()> {
    if !(intGt(row, size)) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*intString(row));
            __mm_s.push_str(&*literal!(": "));
            ArcStr::from(__mm_s)
        });
        BackendDump::debuglst(
            &({
                let __elt = (*metamodelica::index_checked(&matrix.borrow(), row)?).clone();
                __elt
            }),
            &move |__a0: (i32, metamodelica::Ref<DAE::Exp>)| dumpMatrix1(&__a0),
            &(literal!(", ")),
            &(literal!("\n")),
        )?;
        dumpMatrix(row + 1, size, matrix.clone())?;
    }
    Ok(())
}

fn dumpMatrix1(mut inTpl: &(i32, metamodelica::Ref<DAE::Exp>)) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut c: i32;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut cs: ArcStr;
    let mut es: ArcStr;
    (c, e) = inTpl.clone();
    cs = intString(c);
    es = ExpressionBasics::printExpStr(e)?;
    s = stringAppendList(list![cs, literal!(":"), es]);
    Ok(s)
}

fn addRows(
    mut inA: &metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>,
    mut inB: &metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>,
    mut col: i32,
    mut inVars: &BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inTpl: (i32, i32),
    mut inElst: &metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>,
) -> Result<(
    metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>,
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    (i32, i32),
)> {
    let mut outElst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outTpl: (i32, i32);
    (outElst, outVars, outEqns, outTpl) = 'mc: {
        let __mc_input = (&**inA, &**inB);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((inElst.clone().reverse(), inVars.clone(), inEqns.clone(), inTpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((List::append_reverse(inElst, inB.clone()), inVars.clone(), inEqns.clone(), inTpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((List::append_reverse(inElst, inA.clone()), inVars.clone(), inEqns.clone(), inTpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ca, _), tail: resta }, Deref @ metamodelica::ListNode::Cons { head: (cb, _), tail: restb }) => {
                    let mut elst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intEq(ca.clone(), cb.clone())) else { return Err("pattern mismatch") };
                    let true = (intEq(ca.clone(), col)) else { return Err("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(metamodelica::AsArg::as_arg(&resta), metamodelica::AsArg::as_arg(&restb), col, inVars, inEqns.clone(), inTpl, inElst)?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ca, ea), tail: resta }, Deref @ metamodelica::ListNode::Cons { head: (cb, eb), tail: restb }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut elst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intEq(ca.clone(), cb.clone())) else { return Err("pattern mismatch") };
                    e = Expression::expAdd(ea.clone(), eb.clone())?;
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    (vars, eqns, e, tpl) = makeDummyVar(inTpl, e.clone(), inVars.clone(), inEqns.clone())?;
                    (elst, vars, eqns, tpl) = addRows(metamodelica::AsArg::as_arg(&resta), metamodelica::AsArg::as_arg(&restb), col, &vars, eqns.clone(), tpl, &(metamodelica::cons((ca.clone(), e.clone()), inElst.clone())))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ca, _), tail: _ }, Deref @ metamodelica::ListNode::Cons { head: (cb, _), tail: restb }) => {
                    let mut elst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intGt(ca.clone(), cb.clone())) else { return Err("pattern mismatch") };
                    let true = (intEq(cb.clone(), col)) else { return Err("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(inA, metamodelica::AsArg::as_arg(&restb), col, inVars, inEqns.clone(), inTpl, inElst)?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ca, _), tail: _ }, Deref @ metamodelica::ListNode::Cons { head: (cb, eb), tail: restb }) => {
                    let mut elst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intGt(ca.clone(), cb.clone())) else { return Err("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(inA, metamodelica::AsArg::as_arg(&restb), col, inVars, inEqns.clone(), inTpl, &(metamodelica::cons((cb.clone(), eb.clone()), inElst.clone())))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ca, _), tail: resta }, Deref @ metamodelica::ListNode::Cons { head: (cb, _), tail: _ }) => {
                    let mut elst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intLt(ca.clone(), cb.clone())) else { return Err("pattern mismatch") };
                    let true = (intEq(ca.clone(), col)) else { return Err("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(metamodelica::AsArg::as_arg(&resta), inB, col, inVars, inEqns.clone(), inTpl, inElst)?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ca, ea), tail: resta }, Deref @ metamodelica::ListNode::Cons { head: (cb, _), tail: _ }) => {
                    let mut elst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intLt(ca.clone(), cb.clone())) else { return Err("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(metamodelica::AsArg::as_arg(&resta), inB, col, inVars, inEqns.clone(), inTpl, &(metamodelica::cons((ca.clone(), ea.clone()), inElst.clone())))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outElst, outVars, outEqns, outTpl))
}

fn mulRow(
    mut inTpl: &(i32, metamodelica::Ref<DAE::Exp>),
    mut e1: metamodelica::Ref<DAE::Exp>,
) -> Result<(i32, metamodelica::Ref<DAE::Exp>)> {
    let mut outTpl: (i32, metamodelica::Ref<DAE::Exp>);
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut c: i32;
    (c, e) = inTpl.clone();
    e = Expression::negate(Expression::expMul(e, e1)?)?;
    outTpl = (c, e);
    Ok(outTpl)
}

fn removeFromCol<'__b>(
    mut i: i32,
    mut inTpl: &'__b metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>,
    mut inAcc: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>,
) -> metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inTpl {
            Deref @ metamodelica::ListNode::Nil => {
                return inAcc.reverse()
            },
            Deref @ metamodelica::ListNode::Cons { head: (c, _), tail: rest } if (intEq(i, c.clone())) => {
                return listAppend(inAcc.reverse(), rest.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: (c, e), tail: rest } => {
                { (i, inTpl, inAcc) = (i, rest, metamodelica::cons((c.clone(), e.clone()), inAcc)); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn makeDummyVar(
    mut inTpl: (i32, i32),
    mut e: metamodelica::Ref<DAE::Exp>,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<DAE::Exp>,
    (i32, i32),
)> {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (i32, i32);
    (outVars, outEqns, outExp, outTpl) = 'mc: {
        let __mc_input = (inTpl, &*e);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::RCONST { .. }) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Expression::isConst(e.clone())?) else { return Err("pattern mismatch") };
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((a, b), _) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut sa: ArcStr;
                    let mut sb: ArcStr;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut vars: BackendDAE::Variables;
                    let mut cexp: metamodelica::Ref<DAE::Exp>;
                    sa = intString(a.clone());
                    sb = intString(b.clone());
                    cr = ComponentReferenceBasics::makeCrefIdent(stringAppendList(list![literal!("$tmp"), sa.clone(), literal!("_"), sb.clone()]), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
                    cexp = Expression::crefExp(cr.clone())?;
                    eqns = BackendEquation::add(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: cexp.clone(), scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), inEqns.clone())?;
                    v = metamodelica::Ref::new(BackendDAE::Var { varName: cr.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
                    vars = BackendVariable::addVar(v.clone(), inVars.clone())?;
                    Ok((vars.clone(), eqns.clone(), cexp.clone(), (a.clone(), b.clone() + 1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVars, outEqns, outExp, outTpl))
}

fn gaussElimination1(
    mut col: i32,
    mut row: i32,
    mut size: i32,
    mut ce: &metamodelica::Ref<DAE::Exp>,
    mut matrix: metamodelica::Array<metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>>,
    mut inVars: &BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inTpl: (i32, i32),
) -> (
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    (i32, i32),
) {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outTpl: (i32, i32);
    (outVars, outEqns, outTpl) = 'mc: {
        let __mc_input = inTpl;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intGt(row, size)) else {
                return Err("pattern mismatch");
            };
            Ok((inVars.clone(), inEqns.clone(), inTpl))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut vars: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut cexp: metamodelica::Ref<DAE::Exp>;
            let mut elst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
            let mut tpl: (i32, i32);
            let __pa0 = ::match_deref::match_deref! { match &(diagonalEntry(col, &({let __elt = (*metamodelica::index_checked(&matrix.borrow(), row)?).clone(); __elt}))?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            e1 = Expression::expDiv(e.clone(), ce.clone())?;
            (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
            (vars, eqns, cexp, tpl) = makeDummyVar(inTpl, e1.clone(), inVars.clone(), inEqns.clone())?;
            elst = ({
                let __elt = (*metamodelica::index_checked(&matrix.borrow(), col)?).clone();
                __elt
            });
            elst = List::map1(
                elst.clone(),
                &move |__a0: (i32, metamodelica::Ref<DAE::Exp>), __a1: metamodelica::Ref<DAE::Exp>| mulRow(&__a0, __a1),
                cexp.clone(),
            )?;
            (elst, vars, eqns, tpl) = addRows(
                &({
                    let __elt = (*metamodelica::index_checked(&matrix.borrow(), row)?).clone();
                    __elt
                }),
                &elst,
                col,
                &vars,
                eqns.clone(),
                tpl,
                &(metamodelica::nil()),
            )?;
            metamodelica::arrayUpdate(matrix.clone(), row, elst.clone())?;
            (vars, eqns, tpl) = gaussElimination1(col, row + 1, size, ce, matrix.clone(), &vars, eqns.clone(), tpl);
            Ok((vars.clone(), eqns.clone(), tpl))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut vars: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut tpl: (i32, i32);
            (vars, eqns, tpl) =
                gaussElimination1(col, row + 1, size, ce, matrix.clone(), inVars, inEqns.clone(), inTpl);
            Ok((vars.clone(), eqns.clone(), tpl))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVars, outEqns, outTpl)
}

fn gaussElimination(
    mut col: i32,
    mut size: i32,
    mut matrix: metamodelica::Array<metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>>,
    mut inVars: &BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inTpl: (i32, i32),
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
)> {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    (outVars, outEqns) = 'mc: {
        let __mc_input = inTpl;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intGt(col, size)) else {
                return Err("pattern mismatch");
            };
            Ok((inVars.clone(), inEqns.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut vars: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut tpl: (i32, i32);
            let __pa0 = ::match_deref::match_deref! { match &(diagonalEntry(col, &({let __elt = (*metamodelica::index_checked(&matrix.borrow(), col)?).clone(); __elt}))?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            (vars, eqns, tpl) =
                gaussElimination1(col, col + 1, size, &e, matrix.clone(), inVars, inEqns.clone(), inTpl);
            (vars, eqns) = gaussElimination(col + 1, size, matrix.clone(), &vars, eqns.clone(), tpl)?;
            Ok((vars.clone(), eqns.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(diagonalEntry(col, &({let __elt = (*metamodelica::index_checked(&matrix.borrow(), col)?).clone(); __elt}))?) {
                None => (),
                _ => return Err("pattern mismatch"),
            } };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "gaussElimination failt because of non diagonal Entry for col "
                ));
                __mm_s.push_str(&*intString(col));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVars, outEqns))
}

fn diagonalEntry<'__b>(
    mut col: i32,
    mut row: &'__b metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match row {
            Deref @ metamodelica::ListNode::Cons { head: (r, e), tail: rest } => {
                if (intEq(r.clone(), col) && !(Expression::isZero(metamodelica::AsArg::as_arg(&e))?)) {return Ok(Some(e.clone()))} else {if (intGt(r.clone(), col)) {return Ok(None)} else {{ (col, row) = (col, rest); continue '__tco; }}}
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn isConstOneMinusOne(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut b: bool;
    b = Expression::isConstOne(inExp) || Expression::isConstMinusOne(inExp);
    b
}

fn transformJacToAdjacencyMatrix2(
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool>,
) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let () = (::match_deref::match_deref! { match jac {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
            let mut i: i32;
            let mut b: bool;
            let mut b1: bool;
            let mut lst: metamodelica::List<i32>;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            i = ({let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), r.clone())?).clone(); __elt});
            eqn = BackendEquation::get(eqns.clone(), i)?;
            b1 = BackendEquation::isArrayEquation(&eqn);
            b = func(e.clone())?;
            lst = List::consOnTrue(b && b1, c.clone(), ({let __elt = (*metamodelica::index_checked(&m.borrow(), r.clone())?).clone(); __elt}));
            metamodelica::arrayUpdate(m.clone(), r.clone(), lst)?;
            transformJacToAdjacencyMatrix2(rest, m.clone(), mapIncRowEqn.clone(), eqns, ass1.clone(), ass2.clone(), func)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToAdjacencyMatrix1(
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool>,
) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let () = (::match_deref::match_deref! { match jac {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
            let mut b: bool;
            let mut b1: bool;
            let mut lst: metamodelica::List<i32>;
            b1 = intLt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt}), 1);
            b = func(e.clone())?;
            lst = List::consOnTrue(b && b1, c.clone(), ({let __elt = (*metamodelica::index_checked(&m.borrow(), r.clone())?).clone(); __elt}));
            metamodelica::arrayUpdate(m.clone(), r.clone(), lst)?;
            transformJacToAdjacencyMatrix1(rest, m.clone(), ass1.clone(), ass2.clone(), func)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToAdjacencyMatrix(
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool>,
) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let () = (::match_deref::match_deref! { match jac {
        Deref @ metamodelica::ListNode::Nil => {
            transformJacToAdjacencyMatrix(jac, m.clone(), mT.clone(), func)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
            let mut b: bool;
            let mut lst: metamodelica::List<i32>;
            let mut lst1: metamodelica::List<i32>;
            b = func(e.clone())?;
            lst = List::consOnTrue(b, c.clone(), ({let __elt = (*metamodelica::index_checked(&m.borrow(), r.clone())?).clone(); __elt}));
            lst1 = List::consOnTrue(b, r.clone(), ({let __elt = (*metamodelica::index_checked(&mT.borrow(), c.clone())?).clone(); __elt}));
            metamodelica::arrayUpdate(m.clone(), r.clone(), lst)?;
            metamodelica::arrayUpdate(mT.clone(), c.clone(), lst1)?;
            transformJacToAdjacencyMatrix(rest, m.clone(), mT.clone(), func)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToMatrix(
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut row: i32,
    mut col: i32,
    mut size: i32,
    mut b: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut matrix: metamodelica::Array<metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**jac;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(row, size)) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut be: metamodelica::Ref<DAE::Exp>;
                    let mut b1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut lst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
                    let true = (intGt(col, size)) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*b)) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    be = metamodelica::Own::own(__pa0);
                    b1 = metamodelica::Own::own(__pa1);
                    lst = ({let __elt = (*metamodelica::index_checked(&matrix.borrow(), row)?).clone(); __elt});
                    lst = List::consOnTrue(!(Expression::isZero(&be)?), (col, be.clone()), lst.clone());
                    lst = lst.clone().reverse();
                    metamodelica::arrayUpdate(matrix.clone(), row, lst.clone())?;
                    transformJacToMatrix(jac, row + 1, 1, size, &b1, matrix.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    transformJacToMatrix(jac, row, col + 1, size, b, matrix.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
                    let mut lst: metamodelica::List<(i32, metamodelica::Ref<DAE::Exp>)>;
                    let true = (intEq(r.clone(), row)) else { return Err("pattern mismatch") };
                    let true = (intEq(c.clone(), col)) else { return Err("pattern mismatch") };
                    lst = ({let __elt = (*metamodelica::index_checked(&matrix.borrow(), r.clone())?).clone(); __elt});
                    lst = metamodelica::cons((c.clone(), e.clone()), lst.clone());
                    metamodelica::arrayUpdate(matrix.clone(), row, lst.clone())?;
                    transformJacToMatrix(metamodelica::AsArg::as_arg(&rest), row, col + 1, size, b, matrix.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (r, c, _), tail: _ } => {
                    let true = (intEq(r.clone(), row)) else { return Err("pattern mismatch") };
                    let true = (intLt(col, c.clone())) else { return Err("pattern mismatch") };
                    transformJacToMatrix(jac, row, col + 1, size, b, matrix.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (r, _, _), tail: _ } => {
                    let true = (intGe(r.clone(), row)) else { return Err("pattern mismatch") };
                    transformJacToMatrix(jac, row, col + 1, size, b, matrix.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn dumpJacMatrix(
    mut jac: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut row: i32,
    mut col: i32,
    mut size: i32,
    mut vars: &BackendDAE::Variables,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**jac;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(row, size)) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let true = (intGt(col, size)) else { return Err("pattern mismatch") };
                    v = BackendVariable::getVarAt(vars, row)?;
                    cr = BackendVariable::varCref(&v);
                    metamodelica::print(literal!(";... % "));
                    metamodelica::print(intString(row));
                    metamodelica::print(literal!(" "));
                    metamodelica::print(ComponentReferenceBasics::printComponentRefStr(&cr)?);
                    metamodelica::print(literal!("\n"));
                    dumpJacMatrix(jac, row + 1, 1, size, vars)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    metamodelica::print(literal!("0, "));
                    dumpJacMatrix(jac, row, col + 1, size, vars)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest } => {
                    let mut estr: ArcStr;
                    let true = (intEq(r.clone(), row)) else { return Err("pattern mismatch") };
                    let true = (intEq(c.clone(), col)) else { return Err("pattern mismatch") };
                    estr = ExpressionBasics::printExpStr(e.clone())?;
                    metamodelica::print(estr.clone());
                    metamodelica::print(literal!(", "));
                    dumpJacMatrix(metamodelica::AsArg::as_arg(&rest), row, col + 1, size, vars)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (r, c, _), tail: _ } => {
                    let true = (intEq(r.clone(), row)) else { return Err("pattern mismatch") };
                    let true = (intLt(col, c.clone())) else { return Err("pattern mismatch") };
                    metamodelica::print(literal!("0, "));
                    dumpJacMatrix(jac, row, col + 1, size, vars)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (r, _, _), tail: _ } => {
                    let false = (intEq(r.clone(), row)) else { return Err("pattern mismatch") };
                    metamodelica::print(literal!("0, "));
                    dumpJacMatrix(jac, row, col + 1, size, vars)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn getEqnsinOrder(
    mut indx: i32,
    mut inTpl: &(
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        BackendDAE::Variables,
    ),
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
)> {
    let mut outTpl: (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        BackendDAE::Variables,
    );
    let mut e: metamodelica::Ref<BackendDAE::Equation>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqnssort: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut ass2: metamodelica::Array<metamodelica::List<i32>>;
    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut vars: BackendDAE::Variables;
    let mut varssort: BackendDAE::Variables;
    let mut vindxs: metamodelica::List<i32>;
    (eqns, vars, ass2, eqnssort, varssort) = inTpl.clone();
    e = BackendEquation::get(eqns.clone(), indx)?;
    eqnssort = BackendEquation::add(e.clone(), eqnssort)?;
    vindxs = ({
        let __elt = (*metamodelica::index_checked(&ass2.borrow(), indx)?).clone();
        __elt
    });
    vlst = List::map1r(
        vindxs.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        vars.clone(),
    )?;
    vlst = sortVarsforOrder(&e, vlst, &vindxs, &vars)?;
    varssort = BackendVariable::addVars(&vlst, varssort)?;
    outTpl = (eqns, vars, ass2.clone(), eqnssort, varssort);
    Ok(outTpl)
}

fn sortVarsforOrder(
    mut inEqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut inVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut vindxs: &metamodelica::List<i32>,
    mut vars: &BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = &**inEqn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, .. } => {
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    elst = Expression::flattenArrayExpToList(e1.clone());
                    crlst = List::map(elst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCrefNegCref(&__a0))?;
                    vlst = sortVarsforOrder1(&crlst, 1, &inVarLst, vindxs, arrayCreate(((vindxs).len() as i32), None), vars)?;
                    Ok(vlst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e1, .. } => {
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    elst = Expression::flattenArrayExpToList(e1.clone());
                    crlst = List::map(elst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCrefNegCref(&__a0))?;
                    vlst = sortVarsforOrder1(&crlst, 1, &inVarLst, vindxs, arrayCreate(((vindxs).len() as i32), None), vars)?;
                    Ok(vlst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    vlst = List::sort(inVarLst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: metamodelica::Ref<BackendDAE::Var>| BackendVariable::varSortFunc(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>))?;
                    Ok(vlst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarLst)
}

fn sortVarsforOrder1(
    mut crlst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut index: i32,
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut vindxs: &metamodelica::List<i32>,
    mut vararray: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>,
    mut vars: &BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = &**crlst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    vlst = List::sort(inVarLst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: metamodelica::Ref<BackendDAE::Var>| BackendVariable::varSortFunc(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>))?;
                    vlst = sortVarsforOrder2(1, &vlst, vararray.clone(), &(metamodelica::nil()))?;
                    Ok(vlst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest } => {
                    let mut i: i32;
                    let mut p: i32;
                    let mut ilst: metamodelica::List<i32>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (v, i) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), vars)?;
                    p = List::position(i, vindxs)?;
                    ilst = listDelete(vindxs.clone(), p)?;
                    vlst = listDelete(inVarLst.clone(), p)?;
                    metamodelica::arrayUpdate(vararray.clone(), index, Some(v.clone()))?;
                    Ok(sortVarsforOrder1(metamodelica::AsArg::as_arg(&rest), index + 1, &vlst, &ilst, vararray.clone(), vars)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(sortVarsforOrder1(metamodelica::AsArg::as_arg(&rest), index + 1, inVarLst, vindxs, vararray.clone(), vars)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarLst)
}

fn sortVarsforOrder2(
    mut index: i32,
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut vararray: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>,
    mut iAcc: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = &**inVarLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(index, metamodelica::arrayLength(vararray.clone()))) else { return Err("pattern mismatch") };
                    Ok(iAcc.clone().reverse())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let __pa0 = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&vararray.borrow(), index)?).clone(); __elt})) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    v = metamodelica::Own::own(__pa0);
                    Ok(sortVarsforOrder2(index + 1, inVarLst, vararray.clone(), &(metamodelica::cons(v.clone(), iAcc.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: v, tail: vlst } => {
                    Ok(sortVarsforOrder2(index + 1, metamodelica::AsArg::as_arg(&vlst), vararray.clone(), &(metamodelica::cons(v.clone(), iAcc.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarLst)
}

fn getOrphansPairs(
    mut inOrphans: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = &**inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(mark)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } => {
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), o.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    getOrphansPairs1(&(list![o.clone()]), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), o.clone(), &(metamodelica::nil()))?;
                    Ok(getOrphansPairs(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getOrphansPairs(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn getOrphansPairs1(
    mut rows: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut orphan: i32,
    mut nextQueue: &metamodelica::List<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**rows, &**nextQueue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    getOrphansPairs1(nextQueue, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan, &(metamodelica::nil()))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, _) => {
                    let mut o: i32;
                    let mut elst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    elst = List::select1(({let __elt = (*metamodelica::index_checked(&mt.borrow(), r.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    o = hasResidualOrphan(&elst, ass2.clone())?;
                    metamodelica::arrayUpdate(ass1.clone(), orphan, o)?;
                    metamodelica::arrayUpdate(ass2.clone(), o, list![orphan])?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: r, tail: rest }, _) => {
                    let mut next: metamodelica::List<i32>;
                    let mut elst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    elst = List::select1(({let __elt = (*metamodelica::index_checked(&mt.borrow(), r.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = List::select1(List::flatten(List::map1r(elst.clone(), &arrayGet, ass2.clone())?)?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), next.clone());
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), mark)?;
                    getOrphansPairs1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan, &next)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    getOrphansPairs1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), orphan, nextQueue)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn getOrphansPairsConstraints(
    mut inOrphans: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> i32 {
    let mut omark: i32;
    omark = 'mc: {
        let __mc_input = &**inOrphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(mark)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: o, tail: rest } => {
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), o.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(colummarks.clone(), o.clone(), mark)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getOrphansPairsConstraints Process Orphan ")); __mm_s.push_str(&*intString(o.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    getOrphansPairsConstraints1(&(({let __elt = (*metamodelica::index_checked(&mt.borrow(), o.clone())?).clone(); __elt})), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqns.clone(), o.clone(), &(metamodelica::nil()))?;
                    Ok(getOrphansPairsConstraints(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark + 1, rowmarks.clone(), colummarks.clone(), eqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getOrphansPairsConstraints(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    omark
}

fn getOrphansPairsConstraints1(
    mut eqns: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut eqnsarr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut orphan: i32,
    mut nextQueue: &metamodelica::List<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**eqns, &**nextQueue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    getOrphansPairsConstraints1(nextQueue, ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan, &(metamodelica::nil()))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: _ }, _) => {
                    let mut o: i32;
                    let mut next: metamodelica::List<i32>;
                    let mut rlst: metamodelica::List<i32>;
                    let mut ass2lst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    rlst = List::select1(({let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    rlst = List::fold1(&({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone(); __elt}), &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rlst.clone())?;
                    next = List::select1(List::flatten(List::map1r(rlst.clone(), &arrayGet, mt.clone())?)?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    o = hasResidualOrphan1(&next, ass2.clone(), eqnsarr.clone())?;
                    metamodelica::arrayUpdate(ass1.clone(), orphan, o)?;
                    ass2lst = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), o)?).clone(); __elt});
                    ass2lst = metamodelica::cons(orphan, ass2lst.clone());
                    metamodelica::arrayUpdate(ass2.clone(), o, ass2lst.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, _) => {
                    let mut next: metamodelica::List<i32>;
                    let mut rlst: metamodelica::List<i32>;
                    let mut lst: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    rlst = List::select1(({let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    lst = List::select1(List::map1r(rlst.clone(), &arrayGet, ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    rlst = List::fold1(&lst, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rlst.clone())?;
                    next = List::select1(List::map1r(rlst.clone(), &arrayGet, ass1.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    next = listAppend(nextQueue.clone(), next.clone());
                    metamodelica::arrayUpdate(colummarks.clone(), e.clone(), mark)?;
                    getOrphansPairsConstraints1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan, &next)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    getOrphansPairsConstraints1(metamodelica::AsArg::as_arg(&rest), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark, rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan, nextQueue)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn getIndexesForEqnsAdvanced(
    mut orphans: &metamodelica::List<i32>,
    mut index: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut imark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut orowmarks: metamodelica::Array<i32>,
    mut ocolummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut vec1: metamodelica::Array<metamodelica::List<i32>>,
    mut vec2: metamodelica::Array<i32>,
    mut queuemark: metamodelica::Array<bool>,
    mut vars: &BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut size: i32,
) -> i32 {
    let mut outMark: i32;
    outMark = 'mc: {
        let __mc_input = &**orphans;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(imark)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: vorphan, tail: rest } => {
                    let mut eorphan: i32;
                    let mut index1: i32;
                    let mut mark: i32;
                    let mut rows: metamodelica::List<i32>;
                    let mut queue: metamodelica::List<i32>;
                    let mut rqueue: metamodelica::List<i32>;
                    let mut bvars: metamodelica::List<i32>;
                    let mut beqns: metamodelica::List<i32>;
                    let mut lst: metamodelica::List<i32>;
                    let mut vorphans: metamodelica::List<i32>;
                    let mut vorphanseqns: metamodelica::List<i32>;
                    let mut queuelst: metamodelica::List<metamodelica::List<i32>>;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&orowmarks.borrow(), vorphan.clone())?).clone(); __elt}), 1)) else { return Err("pattern mismatch") };
                    eorphan = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), vorphan.clone())?).clone(); __elt});
                    vorphans = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), eorphan)?).clone(); __elt});
                    rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), eorphan)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    rows = List::fold1(&({let __elt = (*metamodelica::index_checked(&ass2.borrow(), eorphan)?).clone(); __elt}), &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rows.clone())?;
                    getIndexSubGraph(&rows, &vorphans, m.clone(), mT.clone(), imark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false);
                    vorphanseqns = List::unique(&(List::flatten(List::map1r(vorphans.clone(), &arrayGet, mT.clone())?)?));
                    queuelst = getIndexQueque(vorphanseqns.clone(), m.clone(), mT.clone(), imark, rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                    queue = List::flatten(queuelst.clone())?;
                    mark = imark + 2;
                    (index1, queue, rqueue) = List::fold1(&queue, &fnptr!(setIndexQueue, i32, (metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<bool>, metamodelica::Array<i32>, i32), (i32, metamodelica::List<i32>, metamodelica::List<i32>)), (vec1.clone(), vec2.clone(), ass2.clone(), queuemark.clone(), colummarks.clone(), mark), (index, metamodelica::nil(), metamodelica::nil()))?;
                    metamodelica::arrayUpdate(vec1.clone(), index1, vorphans.clone())?;
                    metamodelica::arrayUpdate(vec2.clone(), index1, eorphan)?;
                    metamodelica::arrayUpdate(queuemark.clone(), eorphan, true)?;
                    mark = mark + 1;
                    List::map2_0(&rqueue, &doMark, rowmarks.clone(), mark)?;
                    List::map2_0(&queue, &doMark, colummarks.clone(), mark)?;
                    bvars = getBorderElements(&queue, m.clone(), mark, rowmarks.clone(), metamodelica::nil())?;
                    bvars = List::fold1(&vorphans, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), bvars.clone())?;
                    beqns = getBorderElements(&rqueue, mT.clone(), mark, colummarks.clone(), metamodelica::nil())?;
                    beqns = List::removeOnTrue(eorphan, &fnptr!(intEq, i32, i32), beqns.clone())?;
                    lst = List::select2(({let __elt = (*metamodelica::index_checked(&m.borrow(), eorphan)?).clone(); __elt}), (std::sync::Arc::new(unmarked) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark)?;
                    lst = listAppend(vorphans.clone(), listAppend(lst.clone(), bvars.clone()));
                    metamodelica::arrayUpdate(m.clone(), eorphan, lst.clone())?;
                    lst = List::select2(vorphanseqns.clone(), (std::sync::Arc::new(unmarked) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), colummarks.clone(), mark)?;
                    lst = listAppend(metamodelica::cons(eorphan, lst.clone()), beqns.clone());
                    metamodelica::arrayUpdate(mT.clone(), vorphan.clone(), lst.clone())?;
                    setBoarderElemts(&bvars, mT.clone(), mark, colummarks.clone(), eorphan)?;
                    setBoarderElemts(&beqns, m.clone(), mark, rowmarks.clone(), vorphan.clone())?;
                    List::fold1(&vorphans, &markOrphans, -1, orowmarks.clone())?;
                    metamodelica::arrayUpdate(ocolummarks.clone(), eorphan, -1)?;
                    vorphans = List::removeOnTrue(vorphan.clone(), &fnptr!(intEq, i32, i32), vorphans.clone())?;
                    List::fold1(&vorphans, &markOrphans, -1, orowmarks.clone())?;
                    List::fold1r(&vorphans, &*(Arc::new(arrayUpdate.clone())), metamodelica::nil(), mT.clone())?;
                    Ok(getIndexesForEqnsAdvanced(metamodelica::AsArg::as_arg(&rest), index1 + 1, m.clone(), mT.clone(), mark + 2, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone(), queuemark.clone(), vars, eqns.clone(), shared, size))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getIndexesForEqnsAdvanced(metamodelica::AsArg::as_arg(&rest), index, m.clone(), mT.clone(), imark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone(), queuemark.clone(), vars, eqns.clone(), shared, size))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outMark
}

fn getBorderElements<'__b>(
    mut elements: &'__b metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut arr: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match elements {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iAcc)
            },
            Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
                let mut lst: metamodelica::List<i32>;
                let mut lst1: metamodelica::List<i32>;
                (lst, lst1) = List::split2OnTrue(&({let __elt = (*metamodelica::index_checked(&m.borrow(), elem.clone())?).clone(); __elt}), &unmarked, arr.clone(), mark)?;
                metamodelica::arrayUpdate(m.clone(), elem.clone(), lst1)?;
                lst = List::select2(lst, (std::sync::Arc::new(unmarked) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), arr.clone(), mark + 1)?;
                List::map2_0(&lst, &doMark, arr.clone(), mark + 1)?;
                { (elements, m, mark, arr, iAcc) = (rest, m.clone(), mark, arr.clone(), listAppend(lst, iAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn setBoarderElemts(
    mut elements: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut arr: metamodelica::Array<i32>,
    mut orphan: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match elements {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
            let mut lst: metamodelica::List<i32>;
            lst = List::select2(({let __elt = (*metamodelica::index_checked(&m.borrow(), elem.clone())?).clone(); __elt}), (std::sync::Arc::new(unmarked) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), arr.clone(), mark)?;
            metamodelica::arrayUpdate(m.clone(), elem.clone(), metamodelica::cons(orphan, lst))?;
            setBoarderElemts(rest, m.clone(), mark, arr.clone(), orphan)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn setIndexQueue(
    mut col: i32,
    mut tpl: (
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<bool>,
        metamodelica::Array<i32>,
        i32,
    ),
    mut itpl: (i32, metamodelica::List<i32>, metamodelica::List<i32>),
) -> (i32, metamodelica::List<i32>, metamodelica::List<i32>) {
    let mut otpl: (i32, metamodelica::List<i32>, metamodelica::List<i32>);
    otpl = 'mc: {
        let __mc_input = (tpl, &itpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((vec1, vec2, ass2, queuemark, colummark, mark), (index, elst, rlst)) => {
                    let mut r: metamodelica::List<i32>;
                    r = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), col)?).clone(); __elt});
                    let false = (({let __elt = (*metamodelica::index_checked(&queuemark.borrow(), col)?).clone(); __elt})) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(vec1.clone(), index.clone(), r.clone())?;
                    metamodelica::arrayUpdate(vec2.clone(), index.clone(), col)?;
                    metamodelica::arrayUpdate(queuemark.clone(), col, true)?;
                    metamodelica::arrayUpdate(colummark.clone(), col, mark.clone())?;
                    Ok((index.clone() + 1, metamodelica::cons(col, elst.clone()), listAppend(r.clone(), rlst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((_, _, ass2, _, colummark, mark), (index, elst, rlst)) => {
                    let mut r: metamodelica::List<i32>;
                    r = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), col)?).clone(); __elt});
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummark.borrow(), col)?).clone(); __elt}), mark.clone())) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(colummark.clone(), col, mark.clone())?;
                    Ok((index.clone(), metamodelica::cons(col, elst.clone()), listAppend(r.clone(), rlst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(itpl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    otpl
}

fn getIndexQueque(
    mut colums: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut vec2: metamodelica::Array<i32>,
    mut queuemark: metamodelica::Array<bool>,
    mut nextqueue: metamodelica::List<i32>,
    mut iqueue: metamodelica::List<i32>,
    mut iqueue1: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((colums.clone(), nextqueue.clone())) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(iqueue1)
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                let mut queue: metamodelica::List<i32>;
                queue = List::unique(&iqueue);
                { (colums, m, mT, mark, rowmarks, colummarks, ass1, ass2, vec2, queuemark, nextqueue, iqueue, iqueue1) = (nextqueue, m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::cons(queue, iqueue1)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: c, tail: rest }, _) => {
                let mut queue: metamodelica::List<i32>;
                let mut r: metamodelica::List<i32>;
                let mut queue1: metamodelica::List<i32>;
                let mut colums1: metamodelica::List<i32>;
                let mut b1: bool;
                let mut b2: bool;
                r = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), c.clone())?).clone(); __elt});
                (colums1, b2) = getIndexQueque1(&r, c.clone(), mT.clone(), mark, rowmarks.clone())?;
                b1 = !((colums).is_empty());
                queue = if (b1) {List::unionOnTrue(&colums1, &nextqueue, &fnptr!(intEq, i32, i32))?} else {nextqueue};
                queue1 = List::consOnTrue(b2, c.clone(), iqueue);
                { (colums, m, mT, mark, rowmarks, colummarks, ass1, ass2, vec2, queuemark, nextqueue, iqueue, iqueue1) = (rest.clone(), m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), queue, queue1, iqueue1); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getIndexQueque1(
    mut rows: &metamodelica::List<i32>,
    mut c: i32,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
) -> Result<(metamodelica::List<i32>, bool)> {
    let __ab_mT = mT.borrow();
    let __ab_rowmarks = rowmarks.borrow();
    let mut ocolums: metamodelica::List<i32> = metamodelica::nil();
    let mut ob: bool = false;
    let mut colums: metamodelica::List<i32>;
    for mut r in &**rows {
        if intEq((*metamodelica::index_checked(&__ab_rowmarks, r.clone())?).clone(), mark) {
            ob = true;
            colums = List::select(
                (*metamodelica::index_checked(&__ab_mT, r.clone())?).clone(),
                (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
            )?;
            colums = List::removeOnTrue(c, &fnptr!(intEq, i32, i32), colums)?;
            ocolums = listAppend(colums, ocolums);
        }
    }
    ocolums = List::unique(&ocolums);
    Ok((ocolums, ob))
}

fn unmarked(mut indx: i32, mut markarray: metamodelica::Array<i32>, mut mark: i32) -> Result<bool> {
    let __ab_markarray = markarray.borrow();
    let mut b: bool;
    b = intNe((*metamodelica::index_checked(&__ab_markarray, indx)?).clone(), mark);
    Ok(b)
}

fn marked(mut indx: i32, mut markarray: metamodelica::Array<i32>, mut mark: i32) -> Result<bool> {
    let __ab_markarray = markarray.borrow();
    let mut b: bool;
    b = intEq((*metamodelica::index_checked(&__ab_markarray, indx)?).clone(), mark);
    Ok(b)
}

fn isOrphan(mut indx: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    let __ab_ass = ass.borrow();
    let mut b: bool;
    b = intLt((*metamodelica::index_checked(&__ab_ass, indx)?).clone(), 1);
    Ok(b)
}

fn isNoOrphan(mut indx: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    let __ab_ass = ass.borrow();
    let mut b: bool;
    b = intGt((*metamodelica::index_checked(&__ab_ass, indx)?).clone(), 0);
    Ok(b)
}

fn isResOrphan(mut indx: i32, mut ass: metamodelica::Array<metamodelica::List<i32>>) -> Result<bool> {
    let __ab_ass = ass.borrow();
    let mut b: bool;
    b = (*metamodelica::index_checked(&__ab_ass, indx)?).is_empty();
    Ok(b)
}

fn isNoResOrphan(mut indx: i32, mut ass: metamodelica::Array<metamodelica::List<i32>>) -> Result<bool> {
    let __ab_ass = ass.borrow();
    let mut b: bool;
    b = !((*metamodelica::index_checked(&__ab_ass, indx)?).is_empty());
    Ok(b)
}

fn doAssign(
    mut index: i32,
    mut arr: metamodelica::Array<metamodelica::List<i32>>,
    mut assign: metamodelica::List<i32>,
) -> Result<()> {
    metamodelica::arrayUpdate(arr.clone(), index, assign)?;
    Ok(())
}

fn doMark(mut index: i32, mut arr: metamodelica::Array<i32>, mut mark: i32) -> Result<()> {
    metamodelica::arrayUpdate(arr.clone(), index, mark)?;
    Ok(())
}

fn getIndexSubGraph(
    mut rows: &metamodelica::List<i32>,
    mut vorphan: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut colummarks: metamodelica::Array<i32>,
    mut orowmarks: metamodelica::Array<i32>,
    mut ocolummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<metamodelica::List<i32>>,
    mut ifound: bool,
) -> bool {
    let mut found: bool;
    found = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(ifound)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let true = (listMember(r.clone(), vorphan.clone())) else { return Err("pattern mismatch") };
                    getIndexSubGraph(metamodelica::AsArg::as_arg(&rest), vorphan, m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false);
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut e: i32;
                    let false = (listMember(r.clone(), vorphan.clone())) else { return Err("pattern mismatch") };
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&orowmarks.borrow(), r.clone())?).clone(); __elt}), 1)) else { return Err("pattern mismatch") };
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    e = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), r.clone())?).clone(); __elt});
                    List::map2_0(&({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e)?).clone(); __elt}), &doMark, rowmarks.clone(), mark)?;
                    Ok(getIndexSubGraph(metamodelica::AsArg::as_arg(&rest), vorphan, m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut e: i32;
                    let mut nextrows: metamodelica::List<i32>;
                    let mut b: bool;
                    let false = (listMember(r.clone(), vorphan.clone())) else { return Err("pattern mismatch") };
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&orowmarks.borrow(), r.clone())?).clone(); __elt}), 1)) else { return Err("pattern mismatch") };
                    e = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), r.clone())?).clone(); __elt});
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&ocolummarks.borrow(), e)?).clone(); __elt}), 1)) else { return Err("pattern mismatch") };
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e)?).clone(); __elt}), mark)) else { return Err("pattern mismatch") };
                    nextrows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), e)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    nextrows = List::setDifferenceOnTrue(nextrows.clone(), &({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e)?).clone(); __elt}), &fnptr!(intEq, i32, i32))?;
                    metamodelica::arrayUpdate(colummarks.clone(), e, mark)?;
                    b = getIndexSubGraph(&nextrows, vorphan, m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false);
                    markIndexSubgraph(b, &({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e)?).clone(); __elt}), mark, rowmarks.clone())?;
                    Ok(getIndexSubGraph(metamodelica::AsArg::as_arg(&rest), vorphan, m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), b || ifound))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getIndexSubGraph(metamodelica::AsArg::as_arg(&rest), vorphan, m.clone(), mT.clone(), mark, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), ifound))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    found
}

fn markIndexSubgraph(
    mut b: bool,
    mut r: &metamodelica::List<i32>,
    mut mark: i32,
    mut rowmarks: metamodelica::Array<i32>,
) -> Result<()> {
    let () = (match b {
        false => (),
        true => {
            List::map2_0(r, &doMark, rowmarks.clone(), mark)?;
            ()
        }
    });
    Ok(())
}

fn getIndexesForEqnsRest(
    mut i: i32,
    mut size: i32,
    mut id: i32,
    mut mark: i32,
    mut colummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut vec1: metamodelica::Array<i32>,
    mut vec2: metamodelica::Array<i32>,
) -> () {
    let () = 'mc: {
        let __mc_input = vec2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (intGt(i, size)) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(
                mark,
                ({
                    let __elt = (*metamodelica::index_checked(&colummarks.borrow(), i)?).clone();
                    __elt
                }),
            )) else {
                return Err("pattern mismatch");
            };
            getIndexesForEqnsRest(
                i + 1,
                size,
                id,
                mark,
                colummarks.clone(),
                ass1.clone(),
                ass2.clone(),
                vec1.clone(),
                vec2.clone(),
            );
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (intGt(i, size)) else {
                return Err("pattern mismatch");
            };
            metamodelica::arrayUpdate(
                vec1.clone(),
                id,
                ({
                    let __elt = (*metamodelica::index_checked(&ass2.borrow(), i)?).clone();
                    __elt
                }),
            )?;
            metamodelica::arrayUpdate(vec2.clone(), id, i)?;
            getIndexesForEqnsRest(
                i + 1,
                size,
                id + 1,
                mark,
                colummarks.clone(),
                ass1.clone(),
                ass2.clone(),
                vec1.clone(),
                vec2.clone(),
            );
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn markIndexdColums(
    mut i: i32,
    mut size: i32,
    mut mark: i32,
    mut colummarks: metamodelica::Array<i32>,
    mut vec2: metamodelica::Array<i32>,
) -> () {
    let () = 'mc: {
        let __mc_input = vec2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (intGt(i, size)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&vec2.borrow(), i)?).clone();
                    __elt
                }),
                0,
            )) else {
                return Err("pattern mismatch");
            };
            metamodelica::arrayUpdate(
                colummarks.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(&vec2.borrow(), i)?).clone();
                    __elt
                }),
                mark,
            )?;
            markIndexdColums(i + 1, size, mark, colummarks.clone(), vec2.clone());
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (intGt(i, size)) else {
                return Err("pattern mismatch");
            };
            markIndexdColums(i + 1, size, mark, colummarks.clone(), vec2.clone());
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn getOrphans(
    mut indx: i32,
    mut size: i32,
    mut ass: metamodelica::Array<i32>,
    mut inOrphans: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        if intGt(indx, size) {
            return Ok(inOrphans);
        } else {
            {
                (indx, size, ass, inOrphans) = (
                    indx + 1,
                    size,
                    ass.clone(),
                    List::consOnTrue(
                        intLt(
                            ({
                                let __elt = (*metamodelica::index_checked(&ass.borrow(), indx)?).clone();
                                __elt
                            }),
                            1,
                        ),
                        indx,
                        inOrphans,
                    ),
                );
                continue '__tco;
            }
        }
    }
}

fn expHasCref(mut inExp: metamodelica::Ref<DAE::Exp>, mut cr: metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> {
    let mut isthere: bool;
    let mut set: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    set = HashSet::emptyHashSet();
    set = addCrefandParentsToSet(cr, set, None)?;
    let (_, (_, __pa0)) = Expression::traverseExpTopDown(
        inExp,
        &move |__a0: metamodelica::Ref<DAE::Exp>,
               __a1: (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                ),
                i32,
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::ComponentRef>,
                                metamodelica::Ref<DAE::ComponentRef>,
                            ) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                ),
            ),
            bool,
        )|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(expHasCreftraverser(__a0, &__a1)) },
        (set, false),
    )?;
    isthere = metamodelica::Own::own(__pa0);
    Ok(isthere)
}

fn addCrefandParentsToSet(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut ihs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut oprecr: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCref, oprecr)) {
            (cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, None) => {
                let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut set: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                crlst = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
                return Ok(List::fold(&(metamodelica::cons(cr.clone(), crlst)), &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), ihs)?)
            },
            (cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Some(precr)) => {
                let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut set: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                crlst = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
                crlst = List::map1r(metamodelica::cons(cr.clone(), crlst), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::joinCrefs(&__a0, __a1), precr.clone())?;
                return Ok(List::fold(&crlst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), ihs)?)
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: ty, subscriptLst, componentRef: subcr }, None) => {
                let mut idcr: metamodelica::Ref<DAE::ComponentRef>;
                let mut set: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                idcr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                set = BaseHashSet::add(idcr, &ihs)?;
                idcr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), subscriptLst.clone());
                set = BaseHashSet::add(idcr.clone(), &set)?;
                { (inCref, ihs, oprecr) = (subcr.clone(), set, Some(idcr)); continue '__tco; }
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: ty, subscriptLst, componentRef: subcr }, Some(precr)) => {
                let mut idcr: metamodelica::Ref<DAE::ComponentRef>;
                let mut set: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let mut precr = (*precr).clone();
                idcr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                idcr = ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&precr), idcr)?;
                set = BaseHashSet::add(idcr, &ihs)?;
                idcr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), subscriptLst.clone());
                precr = ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&precr), idcr)?;
                set = BaseHashSet::add(precr.clone(), &ihs)?;
                { (inCref, ihs, oprecr) = (subcr.clone(), set, Some(precr.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn expHasCreftraverser(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut inTpl: &(
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
        bool,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
        bool,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
        bool,
    );
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (&*e, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (set, false)) => {
                    let mut b: bool;
                    b = BaseHashSet::has(cr.clone(), &(set.clone()))?;
                    Ok((e.clone(), !(b), (set.clone(), b)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (set, b)) => {
                    Ok((e.clone(), !(b.clone()), (set.clone(), b.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

fn assignLst(
    mut vlst: &metamodelica::List<i32>,
    mut e: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match vlst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: v, tail: rest } => {
            metamodelica::arrayUpdate(ass1.clone(), v.clone(), e)?;
            metamodelica::arrayUpdate(ass2.clone(), e, v.clone())?;
            assignLst(rest, e + 1, ass1.clone(), ass2.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn unassignedLst(mut vlst: &metamodelica::List<i32>, mut ass1: metamodelica::Array<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match vlst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: v, tail: rest } => {
            let false = (intGt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), v.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
            unassignedLst(rest, ass1.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn onefreeMatchingBFS(
    mut queue: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut size: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut columark: metamodelica::Array<i32>,
    mut mark: i32,
    mut nextQeue: &metamodelica::List<i32>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (queue, nextQeue) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            ()
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            onefreeMatchingBFS(nextQeue, m.clone(), mt.clone(), size, ass1.clone(), ass2.clone(), columark.clone(), mark, &(metamodelica::nil()))?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: c, tail: rest }, _) => {
            let mut newqueue: metamodelica::List<i32>;
            let mut rows: metamodelica::List<i32>;
            rows = List::removeOnTrue(ass1.clone(), &isAssignedSaveEnhanced, ({let __elt = (*metamodelica::index_checked(&m.borrow(), c.clone())?).clone(); __elt}))?;
            newqueue = onefreeMatchingBFS1(&rows, c.clone(), mt.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark, nextQeue.clone());
            onefreeMatchingBFS(rest, m.clone(), mt.clone(), size, ass1.clone(), ass2.clone(), columark.clone(), mark, &newqueue)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn isAssignedSaveEnhanced(mut ass: metamodelica::Array<i32>, mut inTpl: i32) -> Result<bool> {
    let __ab_ass = ass.borrow();
    let mut outB: bool;
    outB = if (intGt(inTpl, 0)) {
        intGt((*metamodelica::index_checked(&__ab_ass, inTpl)?).clone(), 0)
    } else {
        true
    };
    Ok(outB)
}

fn onefreeMatchingBFS1(
    mut rows: &metamodelica::List<i32>,
    mut c: i32,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut columark: metamodelica::Array<i32>,
    mut mark: i32,
    mut inNextQeue: metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let __ab_mt = mt.borrow();
    let mut outNextQeue: metamodelica::List<i32>;
    outNextQeue = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut vareqns: metamodelica::List<i32>;
                    metamodelica::arrayUpdate(ass1.clone(), r.clone(), c)?;
                    metamodelica::arrayUpdate(ass2.clone(), c, r.clone())?;
                    vareqns = List::removeOnTrue(ass2.clone(), &isAssignedSaveEnhanced, (*metamodelica::index_checked(&__ab_mt, r.clone())?).clone())?;
                    Ok(listAppend(inNextQeue.clone(), vareqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNextQeue.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outNextQeue
}

fn vectorMatching(
    mut eqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut vars: &BackendDAE::Variables,
    mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (&**eqn, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds, left: e1, right: e2, .. }, _) => {
                    let mut size: i32;
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    size = List::fold(metamodelica::AsArg::as_arg(&ds), &fnptr!(intMul, i32, i32), 1)?;
                    tpl = vectorMatching1(e1.clone(), e2.clone(), size, vars, inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds, left: e2, right: e1, .. }, _) => {
                    let mut size: i32;
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    size = List::fold(metamodelica::AsArg::as_arg(&ds), &fnptr!(intMul, i32, i32), 1)?;
                    tpl = vectorMatching1(e2.clone(), e1.clone(), size, vars, inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, .. }, _) => {
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    tpl = vectorMatching1(e1.clone(), e2.clone(), size.clone(), vars, inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e2, right: e1, .. }, _) => {
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    tpl = vectorMatching1(e2.clone(), e1.clone(), size.clone(), vars, inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    let mut size: i32;
                    size = BackendEquation::equationSize(eqn)?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn vectorMatching1(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut size: i32,
    mut vars: &BackendDAE::Variables,
    mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (&*e1, &*e2, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _, (id, vec1, vec2)) => {
                    let mut ilst: metamodelica::List<i32>;
                    let false = (expHasCref(e2.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars)?;
                    let true = (intEq(size, ((ilst).len() as i32))) else { return Err("pattern mismatch") };
                    unassignedLst(&ilst, vec1.clone())?;
                    assignLst(&ilst, id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (id, vec1, vec2)) => {
                    let mut ilst: metamodelica::List<i32>;
                    let false = (expHasCref(e1.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars)?;
                    let true = (intEq(size, ((ilst).len() as i32))) else { return Err("pattern mismatch") };
                    unassignedLst(&ilst, vec1.clone())?;
                    assignLst(&ilst, id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, _, (id, vec1, vec2)) => {
                    let mut ilst: metamodelica::List<i32>;
                    let false = (expHasCref(e2.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars)?;
                    let true = (intEq(size, ((ilst).len() as i32))) else { return Err("pattern mismatch") };
                    unassignedLst(&ilst, vec1.clone())?;
                    assignLst(&ilst, id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, (id, vec1, vec2)) => {
                    let mut ilst: metamodelica::List<i32>;
                    let false = (expHasCref(e1.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars)?;
                    let true = (intEq(size, ((ilst).len() as i32))) else { return Err("pattern mismatch") };
                    unassignedLst(&ilst, vec1.clone())?;
                    assignLst(&ilst, id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crnosubs: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crlst1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ilst: metamodelica::List<i32>;
                    let mut set: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    elst = Expression::flattenArrayExpToList(e1.clone());
                    crlst = List::map(elst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCrefNegCref(&__a0))?;
                    crlst = List::uniqueOnTrue(&crlst, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1))?;
                    let true = (intEq(size, ((crlst).len() as i32))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crlst.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = metamodelica::Own::own(__pa0);
                    crlst1 = metamodelica::Own::own(__pa1);
                    let true = (List::all(&crlst1, &({ let __pe_b1 = cr.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqualWithoutLastSubs(&__pe_a0, &__pe_b1) }))?) else { return Err("pattern mismatch") };
                    set = HashSet::emptyHashSet();
                    crnosubs = ComponentReferenceBasics::crefStripLastSubs(&cr)?;
                    set = addCrefandParentsToSet(crnosubs.clone(), set.clone(), None)?;
                    set = List::fold(&crlst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), set.clone())?;
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ((metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)| -> metamodelica::Result<_> { ::std::result::Result::Ok(expHasCreftraverser(__a0, &__a1)) }, (set.clone(), false))?) {
                        (_, (_, false)) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (_, ilst) = BackendVariable::getVarLst(&crlst, vars);
                    unassignedLst(&ilst, vec1.clone())?;
                    assignLst(&ilst, id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crnosubs: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crlst1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ilst: metamodelica::List<i32>;
                    let mut set: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    elst = Expression::flattenArrayExpToList(e2.clone());
                    crlst = List::map(elst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::expCrefNegCref(&__a0))?;
                    crlst = List::uniqueOnTrue(&crlst, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1))?;
                    let true = (intEq(size, ((crlst).len() as i32))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crlst.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = metamodelica::Own::own(__pa0);
                    crlst1 = metamodelica::Own::own(__pa1);
                    let true = (List::all(&crlst1, &({ let __pe_b1 = cr.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqualWithoutLastSubs(&__pe_a0, &__pe_b1) }))?) else { return Err("pattern mismatch") };
                    set = HashSet::emptyHashSet();
                    crnosubs = ComponentReferenceBasics::crefStripLastSubs(&cr)?;
                    set = addCrefandParentsToSet(crnosubs.clone(), set.clone(), None)?;
                    set = List::fold(&crlst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), set.clone())?;
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ((metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)| -> metamodelica::Result<_> { ::std::result::Result::Ok(expHasCreftraverser(__a0, &__a1)) }, (set.clone(), false))?) {
                        (_, (_, false)) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (_, ilst) = BackendVariable::getVarLst(&crlst, vars);
                    unassignedLst(&ilst, vec1.clone())?;
                    assignLst(&ilst, id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn aliasMatching(
    mut eqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut vars: &BackendDAE::Variables,
    mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (&**eqn, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, scalar: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, .. }, (id, vec1, vec2)) => {
                    let mut i: i32;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&vec2.borrow(), id.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    (_, i1) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr1), vars)?;
                    (_, i2) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr2), vars)?;
                    i = aliasMatching1(i1, i2, intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), i1)?).clone(); __elt}), 0), intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), i2)?).clone(); __elt}), 0))?;
                    vec1 = metamodelica::arrayUpdate(vec1.clone(), i, id.clone())?;
                    vec2 = metamodelica::arrayUpdate(vec2.clone(), id.clone(), i)?;
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    let mut size: i32;
                    size = BackendEquation::equationSize(eqn)?;
                    Ok((id.clone() + size, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn aliasMatching1(mut i1: i32, mut i2: i32, mut b1: bool, mut b2: bool) -> Result<i32> {
    let mut i: i32;
    i = (match (b1, b2) {
        (false, true) => i1,
        (true, false) => i2,
        _ => return Err("match: no arm matched"),
    });
    Ok(i)
}

fn naturalMatching(
    mut eqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut vars: &BackendDAE::Variables,
    mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> (i32, metamodelica::Array<i32>, metamodelica::Array<i32>) {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (&**eqn, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, (id, vec1, vec2)) => {
                    let mut i: i32;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&vec2.borrow(), id.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars)?) {
                        (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    i = metamodelica::Own::own(__pa0);
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), i)?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayUpdate(vec1.clone(), i, id.clone())?;
                    vec2 = metamodelica::arrayUpdate(vec2.clone(), id.clone(), i)?;
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn naturalMatching1(
    mut eqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut vars: &BackendDAE::Variables,
    mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> (i32, metamodelica::Array<i32>, metamodelica::Array<i32>) {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (&**eqn, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, (id, vec1, vec2)) => {
                    let mut i: i32;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&vec2.borrow(), id.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars)?) {
                        (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    i = metamodelica::Own::own(__pa0);
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), i)?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayUpdate(vec1.clone(), i, id.clone())?;
                    vec2 = metamodelica::arrayUpdate(vec2.clone(), id.clone(), i)?;
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn naturalMatching2(
    mut eqn: metamodelica::Ref<BackendDAE::Equation>,
    mut vars: BackendDAE::Variables,
    mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> (i32, metamodelica::Array<i32>, metamodelica::Array<i32>) {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (&*eqn, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, (id, vec1, vec2)) => {
                    let mut i: i32;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&vec2.borrow(), id.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    e = Expression::expSub(e1.clone(), e2.clone())?;
                    vlst = BackendEquation::equationVars(eqn.clone(), vars.clone())?;
                    (_, i) = getConstOneVariable(&vlst, &e, vec1.clone(), &vars)?;
                    vec1 = metamodelica::arrayUpdate(vec1.clone(), i, id.clone())?;
                    vec2 = metamodelica::arrayUpdate(vec2.clone(), id.clone(), i)?;
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn getConstOneVariable(
    mut vlst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut e: &metamodelica::Ref<DAE::Exp>,
    mut vec1: metamodelica::Array<i32>,
    mut vars: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, i32)> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    let mut i: i32 = 0;
    (outCr, i) = 'mc: {
        let __mc_input = &**vlst;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: v, tail: _ } => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut i: i32 = i.clone();
                    cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars)?) {
                        (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    i = metamodelica::Own::own(__pa0);
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), i)?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    e1 = Differentiate::differentiateExpSolve(e.clone(), cr.clone(), None)?;
                    (e2, _) = ExpressionSimplify::simplify(e1.clone())?;
                    let true = (Expression::isConstOne(&e2) || Expression::isConstMinusOne(&e2)) else { return Err("pattern mismatch") };
                    Ok(((cr.clone(), i), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            i = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut i: i32 = i.clone();
                    (cr, i) = getConstOneVariable(metamodelica::AsArg::as_arg(&rest), e, vec1.clone(), vars)?;
                    Ok(((cr.clone(), i), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            i = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCr, i))
}
