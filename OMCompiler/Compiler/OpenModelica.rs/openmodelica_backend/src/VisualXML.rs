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

use crate::BackendDAEUtil;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::VisualXMLTpl;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_program_util::ProgramUtil;
use openmodelica_tpl::Tpl;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

//----------------------------
//  Visualization types
//----------------------------
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Visualization {
    SHAPE {
        ident: metamodelica::Ref<DAE::ComponentRef>,
        shapeType: metamodelica::Ref<DAE::Exp>,
        T: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
        r: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        r_shape: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        lengthDir: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        widthDir: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        length: metamodelica::Ref<DAE::Exp>,
        width: metamodelica::Ref<DAE::Exp>,
        height: metamodelica::Ref<DAE::Exp>,
        extra: metamodelica::Ref<DAE::Exp>,
        color: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        specularCoeff: metamodelica::Ref<DAE::Exp>,
    },
    VECTOR {
        ident: metamodelica::Ref<DAE::ComponentRef>,
        T: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
        r: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        coordinates: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        color: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        specularCoeff: metamodelica::Ref<DAE::Exp>,
        quantity: metamodelica::Ref<DAE::Exp>,
        headAtOrigin: metamodelica::Ref<DAE::Exp>,
        twoHeadedArrow: metamodelica::Ref<DAE::Exp>,
    },
    SURFACE {
        ident: metamodelica::Ref<DAE::ComponentRef>,
        T: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
        r_0: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        nu: metamodelica::Ref<DAE::Exp>,
        nv: metamodelica::Ref<DAE::Exp>,
        wireframe: metamodelica::Ref<DAE::Exp>,
        multiColored: metamodelica::Ref<DAE::Exp>,
        color: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        specularCoeff: metamodelica::Ref<DAE::Exp>,
        transparency: metamodelica::Ref<DAE::Exp>,
    },
}
impl metamodelica::gc::MMTrace for Visualization {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Visualization::SHAPE {
                ident,
                shapeType,
                T,
                r,
                r_shape,
                lengthDir,
                widthDir,
                length,
                width,
                height,
                extra,
                color,
                specularCoeff,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(shapeType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(T, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r_shape, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lengthDir, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(widthDir, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(length, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(width, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(height, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(extra, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(color, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(specularCoeff, __mmv)?;
                Ok(())
            }
            Visualization::VECTOR {
                ident,
                T,
                r,
                coordinates,
                color,
                specularCoeff,
                quantity,
                headAtOrigin,
                twoHeadedArrow,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(T, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(coordinates, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(color, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(specularCoeff, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(headAtOrigin, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(twoHeadedArrow, __mmv)?;
                Ok(())
            }
            Visualization::SURFACE {
                ident,
                T,
                r_0,
                nu,
                nv,
                wireframe,
                multiColored,
                color,
                specularCoeff,
                transparency,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(T, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r_0, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nu, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nv, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(wireframe, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(multiColored, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(color, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(specularCoeff, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(transparency, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Visualization {
    fn default() -> Self {
        Self::VECTOR {
            ident: Default::default(),
            T: Default::default(),
            r: Default::default(),
            coordinates: Default::default(),
            color: Default::default(),
            specularCoeff: Default::default(),
            quantity: Default::default(),
            headAtOrigin: Default::default(),
            twoHeadedArrow: Default::default(),
        }
    }
}
pub(crate) use self::Visualization::{SHAPE, SURFACE, VECTOR};

//-------------------------
// dump visualization xml
//-------------------------
pub(crate) fn visualizationInfoXML(
    mut daeIn: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut fileName: &ArcStr,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut daeOut: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut eqs0: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut aliasVars: BackendDAE::Variables;
    let mut constVars: BackendDAE::Variables;
    let mut globalKnownVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut allVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut aliasVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut visuals: metamodelica::List<Visualization>;
    let mut allVisuals: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, ArcStr)>;
    let __arc2 = &(*daeIn);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqs0 = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    let __arc5 = shared.clone();
    let BackendDAE::SHARED {
        globalKnownVars: __pa3,
        aliasVars: __pa4,
        ..
    } = &*__arc5;
    globalKnownVars = metamodelica::Own::own(__pa3);
    aliasVars = metamodelica::Own::own(__pa4);
    eqs = List::map(
        eqs0,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendDAEUtil::copyEqSystem(&__a0))
        },
    )?;
    eqs = List::map(
        eqs,
        &fnptr!(setBindingForProtectedVars, metamodelica::Ref<BackendDAE::EqSystem>),
    )?;
    globalKnownVarLst = BackendVariable::varList(&globalKnownVars)?;
    aliasVarLst = BackendVariable::varList(&aliasVars)?;
    allVarLst = List::flatten(List::mapMap(
        eqs.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::daeVars(&__a0))
        },
        &move |__a0: BackendDAE::Variables| BackendVariable::varList(&__a0),
    )?)?;
    (globalKnownVarLst, allVisuals) = List::fold(
        &globalKnownVarLst,
        &fnptr!(
            isVisualizationVarFold,
            metamodelica::Ref<BackendDAE::Var>,
            (
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, ArcStr)>
            )
        ),
        (metamodelica::nil(), metamodelica::nil()),
    )?;
    (allVarLst, allVisuals) = List::fold(
        &allVarLst,
        &fnptr!(
            isVisualizationVarFold,
            metamodelica::Ref<BackendDAE::Var>,
            (
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, ArcStr)>
            )
        ),
        (metamodelica::nil(), allVisuals),
    )?;
    (aliasVarLst, allVisuals) = List::fold(
        &aliasVarLst,
        &fnptr!(
            isVisualizationVarFold,
            metamodelica::Ref<BackendDAE::Var>,
            (
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, ArcStr)>
            )
        ),
        (metamodelica::nil(), allVisuals),
    )?;
    allVarLst = listAppend(globalKnownVarLst, listAppend(allVarLst, aliasVarLst));
    (visuals, _, _) = List::mapFold2(
        &allVisuals,
        &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, ArcStr),
               __a1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
               __a2: Absyn::Program| fillVisualizationObjects(&__a0, __a1, __a2),
        allVarLst,
        program.clone(),
    )?;
    visuals = List::map2(
        visuals,
        &move |__a0: Visualization, __a1: BackendDAE::Variables, __a2: Absyn::Program| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(replaceVisualBinding(__a0, &__a1, &__a2))
        },
        globalKnownVars.clone(),
        program,
    )?;
    constVars = BackendVariable::mergeVariables(globalKnownVars.clone(), aliasVars.clone(), true)?;
    visuals = List::map1(visuals, &inlineConstVisAttributes, constVars)?;
    dumpVis(metamodelica::arrayFromVec(visuals.into_iter().cloned().collect()), {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fileName);
        __mm_s.push_str(&*literal!("_visual.xml"));
        ArcStr::from(__mm_s)
    })?;
    (globalKnownVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        globalKnownVars,
        (std::sync::Arc::new(setVisVarsPublic)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        ArcStr,
                    ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArcStr)>
                    + 'static,
            >),
        literal!(""),
    )?;
    (aliasVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        aliasVars,
        (std::sync::Arc::new(setVisVarsPublic)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        ArcStr,
                    ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArcStr)>
                    + 'static,
            >),
        literal!(""),
    )?;
    daeOut = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: eqs,
        shared: shared,
    });
    Ok(daeOut)
}

fn replaceVisualBinding(
    mut vis: Visualization,
    mut varArray: &BackendDAE::Variables,
    mut program: &Absyn::Program,
) -> Visualization {
    let mut vis: Visualization = vis;
    let () = 'mc: {
        let __mc_input = vis.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Visualization::SHAPE { shapeType: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_shapeType_0 = getConstCrefBinding(cr.clone(), varArray)?;
                    if let Visualization::SHAPE { shapeType, .. } = &mut vis {
                        *shapeType = __owned_variant_shapeType_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Visualization::SHAPE { shapeType: Deref @ DAE::Exp::SCONST { string: s }, .. } => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_shapeType_0 = metamodelica::Ref::new(DAE::Exp::SCONST { string: getFullCADFilePath(s.clone(), program)? });
                    if let Visualization::SHAPE { shapeType, .. } = &mut vis {
                        *shapeType = __owned_variant_shapeType_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
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
    vis
}

fn inlineConstVisAttributes(mut vis: Visualization, mut vars: BackendDAE::Variables) -> Result<Visualization> {
    let mut vis: Visualization = vis;
    let () = (match vis.clone() {
        Visualization::SHAPE { .. } => {
            let __owned_variant_T_0 = Array::map1(
                var_field!(vis.T, Visualization::SHAPE).clone(),
                &inlineConstExpList,
                vars.clone(),
            )?;
            let __owned_variant_r_1 = Array::map1(
                var_field!(vis.r, Visualization::SHAPE).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars.clone(),
            )?;
            let __owned_variant_r_shape_2 = Array::map1(
                var_field!(vis.r_shape, Visualization::SHAPE).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars.clone(),
            )?;
            let __owned_variant_lengthDir_3 = Array::map1(
                var_field!(vis.lengthDir, Visualization::SHAPE).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars.clone(),
            )?;
            let __owned_variant_widthDir_4 = Array::map1(
                var_field!(vis.widthDir, Visualization::SHAPE).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars.clone(),
            )?;
            let __owned_variant_length_5 = inlineConstExp(var_field!(vis.length, Visualization::SHAPE).clone(), &vars);
            let __owned_variant_width_6 = inlineConstExp(var_field!(vis.width, Visualization::SHAPE).clone(), &vars);
            let __owned_variant_height_7 = inlineConstExp(var_field!(vis.height, Visualization::SHAPE).clone(), &vars);
            let __owned_variant_extra_8 = inlineConstExp(var_field!(vis.extra, Visualization::SHAPE).clone(), &vars);
            let __owned_variant_color_9 = Array::map1(
                var_field!(vis.color, Visualization::SHAPE).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars.clone(),
            )?;
            let __owned_variant_specularCoeff_10 =
                inlineConstExp(var_field!(vis.specularCoeff, Visualization::SHAPE).clone(), &vars);
            if let Visualization::SHAPE {
                T,
                r,
                r_shape,
                lengthDir,
                widthDir,
                length,
                width,
                height,
                extra,
                color,
                specularCoeff,
                ..
            } = &mut vis
            {
                *T = __owned_variant_T_0;
                *r = __owned_variant_r_1;
                *r_shape = __owned_variant_r_shape_2;
                *lengthDir = __owned_variant_lengthDir_3;
                *widthDir = __owned_variant_widthDir_4;
                *length = __owned_variant_length_5;
                *width = __owned_variant_width_6;
                *height = __owned_variant_height_7;
                *extra = __owned_variant_extra_8;
                *color = __owned_variant_color_9;
                *specularCoeff = __owned_variant_specularCoeff_10;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE");
            }
            ()
        }
        Visualization::VECTOR { .. } => {
            let __owned_variant_T_0 = Array::map1(
                var_field!(vis.T, Visualization::VECTOR).clone(),
                &inlineConstExpList,
                vars.clone(),
            )?;
            let __owned_variant_r_1 = Array::map1(
                var_field!(vis.r, Visualization::VECTOR).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars.clone(),
            )?;
            let __owned_variant_coordinates_2 = Array::map1(
                var_field!(vis.coordinates, Visualization::VECTOR).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars.clone(),
            )?;
            let __owned_variant_color_3 = Array::map1(
                var_field!(vis.color, Visualization::VECTOR).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars,
            )?;
            if let Visualization::VECTOR {
                T,
                r,
                coordinates,
                color,
                ..
            } = &mut vis
            {
                *T = __owned_variant_T_0;
                *r = __owned_variant_r_1;
                *coordinates = __owned_variant_coordinates_2;
                *color = __owned_variant_color_3;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR");
            }
            ()
        }
        Visualization::SURFACE { .. } => {
            let __owned_variant_T_0 = Array::map1(
                var_field!(vis.T, Visualization::SURFACE).clone(),
                &inlineConstExpList,
                vars.clone(),
            )?;
            let __owned_variant_r_0_1 = Array::map1(
                var_field!(vis.r_0, Visualization::SURFACE).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars.clone(),
            )?;
            let __owned_variant_color_2 = Array::map1(
                var_field!(vis.color, Visualization::SURFACE).clone(),
                &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
                },
                vars,
            )?;
            if let Visualization::SURFACE { T, r_0, color, .. } = &mut vis {
                *T = __owned_variant_T_0;
                *r_0 = __owned_variant_r_0_1;
                *color = __owned_variant_color_2;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE");
            }
            ()
        }
        _ => (),
    });
    Ok(vis)
}

fn inlineConstExpList(
    mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut vars: BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = exps;
    exps = List::map1(
        exps,
        &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(inlineConstExp(__a0, &__a1))
        },
        vars,
    )?;
    Ok(exps)
}

fn inlineConstExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut vars: &BackendDAE::Variables,
) -> metamodelica::Ref<DAE::Exp> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    exp = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    Ok(tryConstCrefValue(cr.clone(), vars)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    exp
}

fn tryConstCrefValue<'__b>(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &'__b BackendDAE::Variables,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        let mut var: metamodelica::Ref<BackendDAE::Var>;
        let mut bind: metamodelica::Ref<DAE::Exp>;
        let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr, vars)?) {
            (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        var = metamodelica::Own::own(__pa0);
        let false = (BackendVariable::isParam(&var)) else {
            return Err("pattern mismatch");
        };
        bind = BackendVariable::varBindExp(&var)?;
        match &*bind {
            _ if (Expression::isConst(bind.clone())?) => return Ok(bind.clone()),
            DAE::Exp::CREF { .. } => {
                (cr, vars) = (Expression::expCref(&bind)?, vars);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

fn getConstCrefBinding(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut eOut: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars), '__try0)) {
            (Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        var = metamodelica::Own::own(__pa1);
        e = unwrap_break_err!(BackendVariable::varBindExp(&var), '__try0);
        eOut = 'mc: {
            let __mc_input = &*e;
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        if !((Expression::isConst(e.clone())?)) { return Err("guard") }
                        Ok(e.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Exp::CREF { componentRef: _, .. } => {
                        Ok(getConstCrefBinding(Expression::expCref(&e)?, vars)?)
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The binding expression ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!(" of the visualization type component ")); __mm_s.push_str(&*ComponentReference::crefStr(&cr)?); __mm_s.push_str(&*literal!("  cannot be evaluated. Please specify a visualization type (CAD files are specified as modelica://packagename/filename.stl)")); ArcStr::from(__mm_s) })?;
                        Ok(e.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            break '__try0 Err::<_, _>("matchcontinue: no arm matched");
        };
        Ok::<_, &'static str>((e.clone(), eOut.clone(), var.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            e = __try0_o0;
            eOut = __try0_o1;
            var = __try0_o2;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("VisualXMl.getConstCrefBinding failed for "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&cr)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/VisualXML.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok(eOut)
}

fn setVisVarsPublic(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut dummyArgIn: ArcStr,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArcStr)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut dummyArgOut: ArcStr = dummyArgIn;
    if isVisualizationVar(&inVar) {
        outVar = makeVarPublicHideResultFalse(inVar)?;
    }
    Ok((outVar, dummyArgOut))
}

fn makeVarPublicHideResultFalse(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut vals: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    vals = inVar.values.clone();
    vals = DAEUtil::setProtectedAttr(vals, false)?;
    outVar = BackendVariable::setVarAttributes(inVar, vals);
    outVar = BackendVariable::setHideResult(outVar, Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })));
    Ok(outVar)
}

fn setBindingForProtectedVars(
    mut eqSysIn: metamodelica::Ref<BackendDAE::EqSystem>,
) -> metamodelica::Ref<BackendDAE::EqSystem> {
    let mut eqSysOut: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut ass1: metamodelica::Array<i32>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    if '__try0: {
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eqSysIn.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa1, orderedVars: __pa2, matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa3, .. }, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        eqs = metamodelica::Own::own(__pa1);
        vars = metamodelica::Own::own(__pa2);
        ass1 = metamodelica::Own::own(__pa3);
        unwrap_break_err!(BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(setBindingForProtectedVars1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (i32, metamodelica::Array<i32>, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (i32, metamodelica::Array<i32>, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>))> + 'static>), (1, ass1.clone(), eqs.clone())), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    eqSysOut = eqSysIn;
    eqSysOut
}

fn setBindingForProtectedVars1(
    mut varIn: metamodelica::Ref<BackendDAE::Var>,
    mut tplIn: (
        i32,
        metamodelica::Array<i32>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        i32,
        metamodelica::Array<i32>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
)> {
    let mut varOut: metamodelica::Ref<BackendDAE::Var>;
    let mut tplOut: (
        i32,
        metamodelica::Array<i32>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    );
    (varOut, tplOut) = 'mc: {
        let __mc_input = (&*varIn, tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { bindExp: None, values: Some(_), .. }, (idx, ass1, eqs)) => {
                    if !((BackendVariable::isProtectedVar(&varIn) && isVisualizationVar(&varIn))) { return Err("guard") }
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    eq = BackendEquation::get(eqs.clone(), metamodelica::arrayGet(ass1.clone(), idx.clone())?)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eq.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp1 = metamodelica::Own::own(__pa0);
                    exp2 = metamodelica::Own::own(__pa1);
                    (exp1, _) = ExpressionSolve::solve(exp1.clone(), exp2.clone(), BackendVariable::varExp(&varIn)?, None)?;
                    var = BackendVariable::setBindExp(varIn.clone(), Some(exp1.clone()));
                    var = makeVarPublicHideResultFalse(var.clone())?;
                    Ok((var.clone(), (idx.clone() + 1, ass1.clone(), eqs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (idx, ass1, eqs)) => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    if BackendVariable::isProtectedVar(&varIn) && isVisualizationVar(&varIn) {
                        var = makeVarPublicHideResultFalse(varIn.clone())?;
                    } else {
                        var = varIn.clone();
                    }
                    Ok((var.clone(), (idx.clone() + 1, ass1.clone(), eqs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((varOut, tplOut))
}

fn fillVisualizationObjects(
    mut visVar: &(metamodelica::Ref<DAE::ComponentRef>, ArcStr),
    mut allVarsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut programIn: Absyn::Program,
) -> Result<(
    Visualization,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    Absyn::Program,
)> {
    let mut visOut: Visualization;
    let mut allVarsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = allVarsIn.clone();
    let mut programOut: Absyn::Program = programIn.clone();
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut vis_name: ArcStr;
    let mut vis: Visualization;
    match '__try0: {
        (cref, vis_name) = visVar.clone();
        vis = unwrap_break_err!(newVisualizer(cref.clone(), &vis_name), '__try0);
        (_, visOut) = unwrap_break_err!(List::fold2(&allVarsIn, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: bool, __a2: Absyn::Program, __a3: (metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, Visualization)| -> metamodelica::Result<_> { ::std::result::Result::Ok(fillVisualizationObjects1(__a0, __a1, &__a2, &__a3)) }, true, programIn.clone(), (metamodelica::nil(), vis.clone())), '__try0);
        Ok::<_, &'static str>((cref.clone(), vis.clone(), visOut.clone(), vis_name.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cref = __try0_o0;
            vis = __try0_o1;
            visOut = __try0_o2;
            vis_name = __try0_o3;
        }
        Err(__try0_err) => {
            metamodelica::print(literal!("fillVisualizationObjects failed! - not yet supported type"));
            return Err(__try0_err);
        }
    }
    Ok((visOut, allVarsOut, programOut))
}

fn newVisualizer(mut cref: metamodelica::Ref<DAE::ComponentRef>, mut visualizerName: &ArcStr) -> Result<Visualization> {
    let mut vis: Visualization;
    vis = (::match_deref::match_deref! { match &(visualizerName.clone()) {
        Deref @ "Shape" => Visualization::SHAPE { ident: cref, shapeType: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("DUMMY") }), T: arrayCreate(3, list![metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })]), r: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), r_shape: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), lengthDir: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), widthDir: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), length: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), width: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), height: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), extra: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), color: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), specularCoeff: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }) },
        Deref @ "Vector" => Visualization::VECTOR { ident: cref, T: arrayCreate(3, list![metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })]), r: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), coordinates: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), color: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), specularCoeff: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), quantity: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), headAtOrigin: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), twoHeadedArrow: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }) },
        Deref @ "Surface" => Visualization::SURFACE { ident: cref, T: arrayCreate(3, list![metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })]), r_0: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), nu: metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 }), nv: metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 }), wireframe: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), multiColored: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), color: arrayCreate(3, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), specularCoeff: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), transparency: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }) },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("VisualXML.newVisualizer")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*visualizerName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/VisualXML.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vis)
}

fn makeCrefQualFromString(mut s: ArcStr) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut crefOut: metamodelica::Ref<DAE::ComponentRef>;
    let mut sLst: metamodelica::List<ArcStr>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    sLst = Util::stringSplitAtChar(s, literal!("."))?;
    crefs = List::map2(
        sLst,
        &fnptr!(
            ComponentReferenceBasics::makeCrefIdent,
            ArcStr,
            metamodelica::Ref<DAE::Type>,
            metamodelica::List<metamodelica::Ref<DAE::Subscript>>
        ),
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
    )?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cref = metamodelica::Own::own(__pa0);
    crefs = metamodelica::Own::own(__pa1);
    crefOut = List::foldr(
        &crefs,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
            ComponentReference::joinCrefs(&__a0, __a1)
        },
        cref,
    )?;
    Ok(crefOut)
}

fn splitCrefAfter(
    mut crefIn: &metamodelica::Ref<DAE::ComponentRef>,
    mut crefCut: &metamodelica::Ref<DAE::ComponentRef>,
) -> (metamodelica::Ref<DAE::ComponentRef>, bool) {
    let mut crefOut: metamodelica::Ref<DAE::ComponentRef>;
    let mut wasCut: bool;
    (crefOut, wasCut) = 'mc: {
        let __mc_input = (&**crefIn, &**crefCut);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: crefIn1, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: crefCut1, .. }) => {
                    let true = (ComponentReferenceBasics::crefFirstCrefEqual(crefIn.clone(), crefCut.clone())?) else { return Err("pattern mismatch") };
                    Ok(splitCrefAfter(metamodelica::AsArg::as_arg(&crefIn1), metamodelica::AsArg::as_arg(&crefCut1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: crefIn1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, .. }) => {
                    let true = (ComponentReferenceBasics::crefFirstCrefEqual(crefIn.clone(), crefCut.clone())?) else { return Err("pattern mismatch") };
                    Ok((crefIn1.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: crefIn1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, .. }) => {
                    let true = (!(ComponentReferenceBasics::crefFirstCrefEqual(crefIn.clone(), crefCut.clone())?)) else { return Err("pattern mismatch") };
                    Ok((crefIn1.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((crefCut.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (crefOut, wasCut)
}

fn fillVisualizationObjects1(
    mut varIn: metamodelica::Ref<BackendDAE::Var>,
    mut storeProtectedCrefs: bool,
    mut program: &Absyn::Program,
    mut tplIn: &(metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, Visualization),
) -> (metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, Visualization) {
    let mut tplOut: (metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, Visualization);
    tplOut = 'mc: {
        let __mc_input = (&*varIn, tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: cref, .. }, (vars, vis @ Visualization::SHAPE { ident, .. })) => {
                    let mut cref1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut filled_vis: Visualization;
                    let __pa0 = ::match_deref::match_deref! { match &(splitCrefAfter(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&ident))) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cref1 = metamodelica::Own::own(__pa0);
                    filled_vis = fillShapeObject(&cref1, &varIn, storeProtectedCrefs, program, vis.clone());
                    Ok((vars.clone(), filled_vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: cref, .. }, (vars, vis @ Visualization::VECTOR { ident, .. })) => {
                    let mut cref1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut filled_vis: Visualization;
                    let __pa0 = ::match_deref::match_deref! { match &(splitCrefAfter(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&ident))) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cref1 = metamodelica::Own::own(__pa0);
                    filled_vis = fillVectorObject(&cref1, &varIn, storeProtectedCrefs, program, vis.clone());
                    Ok((vars.clone(), filled_vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: cref, .. }, (vars, vis @ Visualization::SURFACE { ident, .. })) => {
                    let mut cref1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut filled_vis: Visualization;
                    let __pa0 = ::match_deref::match_deref! { match &(splitCrefAfter(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&ident))) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cref1 = metamodelica::Own::own(__pa0);
                    filled_vis = fillSurfaceObject(&cref1, &varIn, storeProtectedCrefs, program, vis.clone());
                    Ok((vars.clone(), filled_vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut vis: Visualization;
                    (vars, vis) = tplIn.clone();
                    Ok((metamodelica::cons(varIn.clone(), vars.clone()), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    tplOut
}

fn getFullCADFilePath(mut sIn: ArcStr, mut program: &Absyn::Program) -> Result<ArcStr> {
    let mut sOut: ArcStr = sIn.clone();
    let mut chars: metamodelica::List<ArcStr>;
    chars = stringListStringChar(sIn.clone());
    if ((chars).len() as i32) > 11
        && stringEqual(
            &(stringDelimitList(List::firstN(chars, 11)?, literal!(""))),
            &(literal!("modelica://")),
        )
    {
        sOut = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("file://"));
            __mm_s.push_str(&*ProgramUtil::getFullPathFromUri(program, sIn, true)?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(sOut)
}

fn fillShapeObject(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut storeProtectedCrefs: bool,
    mut program: &Absyn::Program,
    mut vis: Visualization,
) -> Visualization {
    let mut vis: Visualization = vis;
    let () = 'mc: {
        let __mc_input = (cref.clone(), vis.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "shapeType", .. }, Visualization::SHAPE { .. }) => {
                    let mut bind: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut vis: Visualization = vis.clone();
                    let __arc1 = &(*var);
                    let BackendDAE::VAR { bindExp: __pa0, .. } = &**__arc1;
                    bind = metamodelica::Own::own(__pa0);
                    if (bind).is_some() {
                        let __owned_variant_shapeType_0 = Util::getOption(bind.clone())?;
                        if let Visualization::SHAPE { shapeType, .. } = &mut vis {
                            *shapeType = __owned_variant_shapeType_0;
                        } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "R", componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "T", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos1 } }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut T0: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    T0 = metamodelica::arrayGet(var_field!(vis.T, Visualization::SHAPE).clone(), pos.clone())?;
                    T0 = List::replaceAt(exp.clone(), pos1.clone(), T0.clone())?;
                    metamodelica::arrayUpdate(var_field!(vis.T, Visualization::SHAPE).clone(), pos.clone(), T0.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "r", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.r, Visualization::SHAPE).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "r_shape", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.r_shape, Visualization::SHAPE).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "lengthDirection", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.lengthDir, Visualization::SHAPE).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "widthDirection", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.widthDir, Visualization::SHAPE).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "length", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_length_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SHAPE { length, .. } = &mut vis {
                        *length = __owned_variant_length_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "width", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_width_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SHAPE { width, .. } = &mut vis {
                        *width = __owned_variant_width_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "height", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_height_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SHAPE { height, .. } = &mut vis {
                        *height = __owned_variant_height_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "extra", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_extra_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SHAPE { extra, .. } = &mut vis {
                        *extra = __owned_variant_extra_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "color", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.color, Visualization::SHAPE).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "specularCoefficient", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_specularCoeff_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SHAPE { specularCoeff, .. } = &mut vis {
                        *specularCoeff = __owned_variant_specularCoeff_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
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
    vis
}

fn fillVectorObject(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut storeProtectedCrefs: bool,
    mut program: &Absyn::Program,
    mut vis: Visualization,
) -> Visualization {
    let mut vis: Visualization = vis;
    let () = 'mc: {
        let __mc_input = (cref.clone(), vis.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "R", componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "T", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos1 } }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }, Visualization::VECTOR { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut T0: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    T0 = metamodelica::arrayGet(var_field!(vis.T, Visualization::VECTOR).clone(), pos.clone())?;
                    T0 = List::replaceAt(exp.clone(), pos1.clone(), T0.clone())?;
                    metamodelica::arrayUpdate(var_field!(vis.T, Visualization::VECTOR).clone(), pos.clone(), T0.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "r", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::VECTOR { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.r, Visualization::VECTOR).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "coordinates", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::VECTOR { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.coordinates, Visualization::VECTOR).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "color", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::VECTOR { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.color, Visualization::VECTOR).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "specularCoefficient", .. }, Visualization::VECTOR { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_specularCoeff_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::VECTOR { specularCoeff, .. } = &mut vis {
                        *specularCoeff = __owned_variant_specularCoeff_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "quantity", .. }, Visualization::VECTOR { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_quantity_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::VECTOR { quantity, .. } = &mut vis {
                        *quantity = __owned_variant_quantity_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "headAtOrigin", .. }, Visualization::VECTOR { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_headAtOrigin_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::VECTOR { headAtOrigin, .. } = &mut vis {
                        *headAtOrigin = __owned_variant_headAtOrigin_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "twoHeadedArrow", .. }, Visualization::VECTOR { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_twoHeadedArrow_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::VECTOR { twoHeadedArrow, .. } = &mut vis {
                        *twoHeadedArrow = __owned_variant_twoHeadedArrow_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
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
    vis
}

fn fillSurfaceObject(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut storeProtectedCrefs: bool,
    mut program: &Absyn::Program,
    mut vis: Visualization,
) -> Visualization {
    let mut vis: Visualization = vis;
    let () = 'mc: {
        let __mc_input = (cref.clone(), vis.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "R", componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "T", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos1 } }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }, Visualization::SURFACE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut T0: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    T0 = metamodelica::arrayGet(var_field!(vis.T, Visualization::SURFACE).clone(), pos.clone())?;
                    T0 = List::replaceAt(exp.clone(), pos1.clone(), T0.clone())?;
                    metamodelica::arrayUpdate(var_field!(vis.T, Visualization::SURFACE).clone(), pos.clone(), T0.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "r_0", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::SURFACE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.r_0, Visualization::SURFACE).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "nu", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_nu_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SURFACE { nu, .. } = &mut vis {
                        *nu = __owned_variant_nu_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "nv", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_nv_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SURFACE { nv, .. } = &mut vis {
                        *nv = __owned_variant_nv_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "wireframe", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_wireframe_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SURFACE { wireframe, .. } = &mut vis {
                        *wireframe = __owned_variant_wireframe_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "multiColored", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_multiColored_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SURFACE { multiColored, .. } = &mut vis {
                        *multiColored = __owned_variant_multiColored_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "color", subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Visualization::SURFACE { .. }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = getVariableBinding(var, storeProtectedCrefs)?;
                    metamodelica::arrayUpdate(var_field!(vis.color, Visualization::SURFACE).clone(), pos.clone(), exp.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "specularCoefficient", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_specularCoeff_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SURFACE { specularCoeff, .. } = &mut vis {
                        *specularCoeff = __owned_variant_specularCoeff_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "transparency", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_transparency_0 = getVariableBinding(var, storeProtectedCrefs)?;
                    if let Visualization::SURFACE { transparency, .. } = &mut vis {
                        *transparency = __owned_variant_transparency_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(((), vis.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vis = __wb0;
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
    vis
}

fn getVariableBinding(
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut storeProtectedCrefs: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut binding: Option<metamodelica::Ref<DAE::Exp>>;
    let __arc1 = &(*var);
    let BackendDAE::VAR { bindExp: __pa0, .. } = &**__arc1;
    binding = metamodelica::Own::own(__pa0);
    if (binding).is_some() {
        let __pa2 = ::match_deref::match_deref! { match &(binding) {
            Some(__pa2) => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa2);
        if !(Expression::isConstValue(&exp)?) && storeProtectedCrefs {
            exp = BackendVariable::varExp(var)?;
        }
    } else {
        exp = BackendVariable::varExp(var)?;
    }
    Ok(exp)
}

fn printVisualization(mut vis: &Visualization) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = (match vis.clone() {
        Visualization::SHAPE {
            ident: mut ident,
            shapeType: mut shapeType,
            color: mut color,
            r: mut r,
            lengthDir: mut lengthDir,
            widthDir: mut widthDir,
            T: mut T,
            length: mut length,
            width: mut width,
            height: mut height,
            extra: mut extra,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("SHAPE "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                metamodelica::AsArg::as_arg(&ident),
            )?);
            __mm_s.push_str(&*literal!(" '"));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(shapeType.clone())?);
            __mm_s.push_str(&*literal!("'\n r{"));
            __mm_s.push_str(&*stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut e in (r.clone()).borrow().iter() {
                        let __x = ExpressionDump::dumpExpStr(e.clone(), 0)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("}"));
            __mm_s.push_str(&*literal!("\nlD{"));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(lengthDir.clone(), &ExpressionBasics::printExpStr)?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("}"));
            __mm_s.push_str(&*literal!(" wD{"));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(widthDir.clone(), &ExpressionBasics::printExpStr)?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("}"));
            __mm_s.push_str(&*literal!("\ncolor("));
            __mm_s.push_str(&*stringDelimitList(
                List::mapArray(color.clone(), &ExpressionBasics::printExpStr)?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(")"));
            __mm_s.push_str(&*literal!(" w: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(width.clone())?);
            __mm_s.push_str(&*literal!(" h: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(height.clone())?);
            __mm_s.push_str(&*literal!(" l: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(length.clone())?);
            __mm_s.push_str(&*literal!("\nT {"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(
                    List::flatten(T.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())?,
                    &ExpressionBasics::printExpStr,
                )?,
                literal!(", "),
            ));
            __mm_s.push_str(&*literal!("}"));
            __mm_s.push_str(&*literal!("\nextra{"));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(extra.clone())?);
            __mm_s.push_str(&*literal!("}"));
            ArcStr::from(__mm_s)
        }
        _ => {
            literal!("-")
        }
    });
    Ok(s)
}

fn isVisualizationVar(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut isVisVar: bool;
    isVisVar = (match &**var {
        BackendDAE::Var { source, .. } => {
            let mut obj: ArcStr;
            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            paths = ElementSource::getElementSourceTypes(source);
            (obj, _) = hasVisPath(paths, 1);
            Util::stringNotEqual(&obj, &(literal!("")))
        }
        _ => false,
    });
    isVisVar
}

fn isVisualizationVarFold(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut tplIn: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, ArcStr)>,
    ),
) -> (
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, ArcStr)>,
) {
    let mut tplOut: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, ArcStr)>,
    );
    tplOut = 'mc: {
        let __mc_input = (&*var, &tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName, source, .. }, (varLst, crefs)) => {
                    let mut idx: i32;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut obj: ArcStr;
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut crefs = (*crefs).clone();
                    paths = ElementSource::getElementSourceTypes(metamodelica::AsArg::as_arg(&source));
                    (obj, idx) = hasVisPath(paths.clone(), 1);
                    let true = (Util::stringNotEqual(&obj, &(literal!("")))) else { return Err("pattern mismatch") };
                    cref = ComponentReference::firstNCrefs(metamodelica::AsArg::as_arg(&varName), idx - 1);
                    crefs = List::unique(&(metamodelica::cons((cref.clone(), obj.clone()), crefs.clone())));
                    Ok((metamodelica::cons(var.clone(), varLst.clone()), crefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tplIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    tplOut
}

fn hasVisPath(mut pathsIn: metamodelica::List<metamodelica::Ref<Absyn::Path>>, mut numIn: i32) -> (ArcStr, i32) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(pathsIn) {
            Deref @ metamodelica::ListNode::Nil => {
                return (literal!(""), -1)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Path::FULLYQUALIFIED { path }, tail: rest } => {
                { (pathsIn, numIn) = (metamodelica::cons(path.clone(), rest.clone()), numIn); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Modelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Mechanics", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "MultiBody", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Visualizers", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Advanced", path: Deref @ Absyn::Path::IDENT { name } } } } } }, tail: _ } if (isVisualizerName(metamodelica::AsArg::as_arg(&name))) => {
                return (name.clone(), numIn)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "ModelicaServices", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Animation", path: Deref @ Absyn::Path::IDENT { name } } }, tail: _ } if (isVisualizerName(metamodelica::AsArg::as_arg(&name))) => {
                return (name.clone(), numIn)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (pathsIn, numIn) = (rest.clone(), numIn + 1); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn isVisualizerName(mut name: &ArcStr) -> bool {
    let mut isVisualizer: bool;
    isVisualizer = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "Shape" => true,
        Deref @ "Vector" => true,
        Deref @ "Surface" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isVisualizer
}

fn dumpVis(mut visIn: metamodelica::Array<Visualization>, mut iFileName: ArcStr) -> Result<()> {
    metamodelica::print(literal!(""));
    Tpl::tplNoret2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Array<Visualization>, __a2: ArcStr| {
                VisualXMLTpl::dumpVisXML(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Array<Visualization>, ArcStr) -> Result<Tpl::Text>
                    + 'static,
            >),
        visIn.clone(),
        iFileName,
    )?;
    Ok(())
}
