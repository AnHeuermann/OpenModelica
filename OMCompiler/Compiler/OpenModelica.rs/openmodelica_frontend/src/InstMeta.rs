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

use crate::Lookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Flags;

pub(crate) fn fixUniontype(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inState: &ClassInf::State,
    mut inClassDef: &metamodelica::Ref<SCode::ClassDef>,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<DAE::Type>>)> {
    let mut cache: FCore::Cache = inCache;
    let mut outType: Option<metamodelica::Ref<DAE::Type>>;
    outType = (::match_deref::match_deref! { match &((inState, &**inClassDef)) {
        (ClassInf::State::META_UNIONTYPE { typeVars, .. }, Deref @ SCode::ClassDef::PARTS { .. }) => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut p2: metamodelica::Ref<Absyn::Path>;
            let mut utPathOfRestriction: metamodelica::Ref<Absyn::Path>;
            let mut utPath: metamodelica::Ref<Absyn::Path>;
            let mut isSingleton: bool;
            let mut singletonType: metamodelica::Ref<DAE::EvaluateSingletonType>;
            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut typeVarsTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut names: metamodelica::List<ArcStr>;
            utPath = var_field!(inState.path, ClassInf::State::META_UNIONTYPE).clone();
            p = AbsynUtil::makeFullyQualified(var_field!(inState.path, ClassInf::State::META_UNIONTYPE).clone());
            names = SCodeUtil::elementNames(&(({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (var_field!((**inClassDef).elementLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            if !((match &*e.clone() {
        SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { name: __esc_utPathOfRestriction, .. }, .. } => {
            utPathOfRestriction = (*__esc_utPathOfRestriction).clone();
            AbsynUtil::pathSuffixOf(metamodelica::AsArg::as_arg(&utPathOfRestriction), &utPath)
        },
        _ => false,
    })) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })))?;
            paths = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
        for mut n in (names).into_iter().cloned() {
            let __x = AbsynUtil::suffixPath(&p, &(n.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            isSingleton = ((paths).len() as i32) == 1;
            if isSingleton {
                p2 = (paths).get(1)?;
                singletonType = unreachable!("DAE.EvaluateSingletonType.EVAL_SINGLETON_TYPE_FUNCTION is retired and not implemented in the Rust port");
            } else {
                singletonType = openmodelica_frontend_types::DAE::EvaluateSingletonType::interned_NOT_SINGLETON();
            }
            typeVarsTypes = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut tv in (typeVars.clone()).into_iter().cloned() {
            let __x = metamodelica::Ref::new(DAE::Type::T_METAPOLYMORPHIC { name: tv.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Some(metamodelica::Ref::new(DAE::Type::T_METAUNIONTYPE { paths: paths, typeVars: typeVarsTypes, knownSingleton: isSingleton, singletonType: singletonType, path: p }))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, outType))
}

fn fixUniontype2(
    mut arr: metamodelica::Array<(
        FCore::Cache,
        FCore::Graph,
        metamodelica::Ref<Absyn::Path>,
        Option<metamodelica::Ref<DAE::Type>>,
    )>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut singletonType: metamodelica::Ref<DAE::Type>;
    let mut cache: FCore::Cache;
    let mut env: FCore::Graph;
    let mut p: metamodelica::Ref<Absyn::Path>;
    let mut ot: Option<metamodelica::Ref<DAE::Type>>;
    (cache, env, p, ot) = metamodelica::arrayGet(arr.clone(), 1)?;
    if (ot).is_none() {
        (_, singletonType, _) = Lookup::lookupType(
            cache.clone(),
            env.clone(),
            p.clone(),
            Some(metamodelica::sourceInfo!("FrontEnd/InstMeta.mo")),
        )?;
        metamodelica::arrayUpdate(arr.clone(), 1, (cache, env, p, Some(singletonType.clone())))?;
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(ot) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        singletonType = metamodelica::Own::own(__pa0);
    }
    Ok(singletonType)
}

pub(crate) fn checkArrayType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<()> {
    let mut el_ty: metamodelica::Ref<DAE::Type>;
    el_ty = Types::arrayElementType(inType);
    let false = (!(Types::isString(&el_ty)) && Types::isBoxedType(&el_ty) || Flags::isSet(Flags::RML.clone())?) else {
        return Err("pattern mismatch");
    };
    Ok(())
}
