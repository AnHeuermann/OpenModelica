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

use crate::ComponentReference;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionSimplify;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type Binding = metamodelica::Ref<DAE::Binding>;

pub type Const = DAE::Const;

pub type EqualityConstraint = Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;

pub type FuncArg = metamodelica::Ref<DAE::FuncArg>;

pub type Properties = DAE::Properties;

pub type TupleConst = metamodelica::Ref<DAE::TupleConst>;

pub type Type = metamodelica::Ref<DAE::Type>;

pub type Var = metamodelica::Ref<DAE::Var>;

pub type EqMod = DAE::EqMod;

pub(crate) fn discreteType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<()> {
    let true = (isDiscreteType(inType)) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

pub fn isDiscreteType<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_INTEGER { .. } => return true,
            DAE::Type::T_STRING { .. } => return true,
            DAE::Type::T_BOOL { .. } => return true,
            DAE::Type::T_CLOCK { .. } => return true,
            DAE::Type::T_ENUMERATION { .. } => return true,
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                inType = var_field!((**inType).complexType, DAE::Type::T_SUBTYPE_BASIC);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn propsAnd(mut inProps: &metamodelica::List<DAE::Properties>) -> Result<DAE::Properties> {
    let mut outProp: DAE::Properties;
    outProp = (::match_deref::match_deref! { match inProps {
        Deref @ metamodelica::ListNode::Cons { head: prop, tail: Deref @ metamodelica::ListNode::Nil } => {
            prop.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: DAE::Properties::PROP { type_: ty, constFlag: c }, tail: props } => {
            let mut c2: Const;
            let mut ty2: Type;
            let mut c = (*c).clone();
            let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (propsAnd(props)?) else { return Err("pattern mismatch") };
            ty2 = metamodelica::Own::own(__pa0);
            c2 = metamodelica::Own::own(__pa1);
            c = constAnd(c.clone(), c2);
            let true = (equivtypes(ty.clone(), ty2)) else { return Err("pattern mismatch") };
            DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outProp)
}

pub(crate) fn makePropsNotConst(mut inProperties: &DAE::Properties) -> Result<DAE::Properties> {
    let mut outProperties: DAE::Properties;
    outProperties = (match inProperties.clone() {
        DAE::Properties::PROP { type_: ref t, .. } => DAE::Properties::PROP {
            type_: t.clone(),
            constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
        },
        _ => return Err("match: no arm matched"),
    });
    Ok(outProperties)
}

// stefan
pub fn getConstList(
    mut inPropertiesList: &metamodelica::List<DAE::Properties>,
) -> Result<metamodelica::List<DAE::Const>> {
    let mut outConstList: metamodelica::List<DAE::Const>;
    outConstList = (::match_deref::match_deref! { match inPropertiesList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: DAE::Properties::PROP { constFlag: c, .. }, tail: pcdr } => {
            let mut ccdr: metamodelica::List<DAE::Const>;
            ccdr = getConstList(pcdr)?;
            metamodelica::cons(c.clone(), ccdr)
        },
        Deref @ metamodelica::ListNode::Cons { head: DAE::Properties::PROP_TUPLE { tupleConst: tc, .. }, tail: pcdr } => {
            let mut c: Const;
            let mut ccdr: metamodelica::List<DAE::Const>;
            c = propertiesListToConst2(metamodelica::AsArg::as_arg(&tc))?;
            ccdr = getConstList(pcdr)?;
            metamodelica::cons(c, ccdr)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outConstList)
}

pub fn propertiesListToConst(mut p: &metamodelica::List<DAE::Properties>) -> Result<DAE::Const> {
    let mut c: DAE::Const;
    c = (::match_deref::match_deref! { match p {
        Deref @ metamodelica::ListNode::Nil => {
            openmodelica_frontend_types::DAE::Const::C_CONST
        },
        Deref @ metamodelica::ListNode::Cons { head: DAE::Properties::PROP { type_: _, constFlag: c1 }, tail: pps } => {
            let mut c2: Const;
            let mut c1 = (*c1).clone();
            c2 = propertiesListToConst(pps)?;
            c1 = constAnd(c1.clone(), c2);
            c1.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: DAE::Properties::PROP_TUPLE { type_: _, tupleConst: tc1 }, tail: pps } => {
            let mut c1: Const;
            let mut c2: Const;
            c1 = propertiesListToConst2(metamodelica::AsArg::as_arg(&tc1))?;
            c2 = propertiesListToConst(pps)?;
            c1 = constAnd(c1, c2);
            c1
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(c)
}

fn propertiesListToConst2(mut t: &metamodelica::Ref<DAE::TupleConst>) -> Result<DAE::Const> {
    let mut c: DAE::Const;
    c = (::match_deref::match_deref! { match t {
        Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c1 } => {
            c1.clone()
        },
        Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: tc1, tail: tcxl } } => {
            let mut c1: Const;
            let mut c2: Const;
            c1 = propertiesListToConst2(metamodelica::AsArg::as_arg(&tc1))?;
            c2 = tupleConstListToConst(metamodelica::AsArg::as_arg(&tcxl))?;
            c1 = constAnd(c1, c2);
            c1
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(c)
}

pub fn tupleConstListToConst(mut t: &metamodelica::List<metamodelica::Ref<DAE::TupleConst>>) -> Result<DAE::Const> {
    let mut c: DAE::Const;
    c = (::match_deref::match_deref! { match t {
        Deref @ metamodelica::ListNode::Nil => {
            openmodelica_frontend_types::DAE::Const::C_CONST
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c1 }, tail: tcxl } => {
            let mut c2: Const;
            let mut c1 = (*c1).clone();
            c2 = tupleConstListToConst(tcxl)?;
            c1 = constAnd(c1.clone(), c2);
            c1.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: p1 @ Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: _ }, tail: tcxl } => {
            let mut c1: Const;
            let mut c2: Const;
            c1 = propertiesListToConst2(metamodelica::AsArg::as_arg(&p1))?;
            c2 = tupleConstListToConst(tcxl)?;
            c1 = constAnd(c1, c2);
            c1
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(c)
}

pub(crate) fn externalObjectType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<()> {
    let () = (match &**inType {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ },
            ..
        } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn varBinding(mut inVar: &metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::Binding> {
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let __arc1 = &(*inVar);
    let DAE::TYPES_VAR { binding: __pa0, .. } = &**__arc1;
    outBinding = metamodelica::Own::own(__pa0);
    outBinding
}

pub(crate) fn varEqualName(mut inVar1: &metamodelica::Ref<DAE::Var>, mut inVar2: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut outEqual: bool;
    let mut name1: ArcStr;
    let mut name2: ArcStr;
    let __arc1 = &(*inVar1);
    let DAE::TYPES_VAR { name: __pa0, .. } = &**__arc1;
    name1 = metamodelica::Own::own(__pa0);
    let __arc3 = &(*inVar2);
    let DAE::TYPES_VAR { name: __pa2, .. } = &**__arc3;
    name2 = metamodelica::Own::own(__pa2);
    outEqual = metamodelica::stringEq(&name1, &name2);
    outEqual
}

pub fn externalObjectConstructorType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<()> {
    let () = (match &**inType {
        DAE::Type::T_FUNCTION { funcResultType: tp, .. } => {
            externalObjectType(tp)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn simpleType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<()> {
    let true = (isSimpleType(inType)) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

pub fn isSimpleType<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_REAL { .. } => return true,
            DAE::Type::T_INTEGER { .. } => return true,
            DAE::Type::T_STRING { .. } => return true,
            DAE::Type::T_BOOL { .. } => return true,
            DAE::Type::T_CLOCK { .. } => return true,
            DAE::Type::T_ENUMERATION { .. } => return true,
            DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                inType = t;
                continue '__tco;
            }
            DAE::Type::T_FUNCTION { funcResultType: t, .. } => {
                inType = t;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn isSimpleNumericType<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_REAL { .. } => return true,
            DAE::Type::T_INTEGER { .. } => return true,
            DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                inType = t;
                continue '__tco;
            }
            DAE::Type::T_FUNCTION { funcResultType: t, .. } => {
                inType = t;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn isNumericType<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_ARRAY { ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            _ => return isSimpleNumericType(inType),
        }
    }
}

pub fn isConnector(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outIsConnector: bool;
    outIsConnector = (match &**inType {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::CONNECTOR { .. },
            ..
        } => true,
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType: ClassInf::State::CONNECTOR { .. },
            ..
        } => true,
        _ => false,
    });
    outIsConnector
}

pub(crate) fn isComplexConnector(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outIsComplexConnector: bool;
    outIsComplexConnector = (match &**inType {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::CONNECTOR { .. },
            ..
        } => true,
        _ => false,
    });
    outIsComplexConnector
}

pub(crate) fn isComplexExpandableConnector(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outResult: bool;
    outResult = (match &**inType {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::CONNECTOR { isExpandable: true, .. },
            ..
        } => true,
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType: ClassInf::State::CONNECTOR { isExpandable: true, .. },
            ..
        } => true,
        _ => false,
    });
    outResult
}

pub fn isComplexType<'__b>(mut ity: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match ity {
            Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                { ity = ty; continue '__tco; }
            },
            Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
                { ity = ty; continue '__tco; }
            },
            Deref @ DAE::Type::T_COMPLEX { varLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => {
                return true
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn isExternalObject(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**tp {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ },
            ..
        } => true,
        _ => false,
    });
    b
}

pub fn expTypetoTypesType(mut inType: &metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    let mut oType: metamodelica::Ref<DAE::Type>;
    oType = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: at, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut ty: Type;
                    let mut tty: Type;
                    ty = expTypetoTypesType(metamodelica::AsArg::as_arg(&at));
                    tty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] });
                    Ok(tty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: at, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: ad } } => {
                    let mut ty: Type;
                    let mut tty: Type;
                    ty = expTypetoTypesType(&(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: at.clone(), dims: ad.clone() })));
                    tty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] });
                    Ok(tty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: CIS, varLst: vars, equalityConstraint: ec, .. } => {
                    let mut vars = (*vars).clone();
                    vars = List::map(vars.clone(), &fnptr!(convertFromExpToTypesVar, metamodelica::Ref<DAE::Var>))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: CIS.clone(), varLst: vars.clone(), equalityConstraint: ec.clone(), usedExternally: var_field!((**inType).usedExternally, DAE::Type::T_COMPLEX).clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: CIS, varLst: vars, complexType: ty, equalityConstraint: ec } => {
                    let mut vars = (*vars).clone();
                    let mut ty = (*ty).clone();
                    vars = List::map(vars.clone(), &fnptr!(convertFromExpToTypesVar, metamodelica::Ref<DAE::Var>))?;
                    ty = expTypetoTypesType(metamodelica::AsArg::as_arg(&ty));
                    Ok(metamodelica::Ref::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: CIS.clone(), varLst: vars.clone(), complexType: ty.clone(), equalityConstraint: ec.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METABOXED { ty } => {
                    let mut ty = (*ty).clone();
                    ty = expTypetoTypesType(metamodelica::AsArg::as_arg(&ty));
                    Ok(metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: ty.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oType
}

fn convertFromExpToTypesVar(mut inVar: metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::Var> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    outVar = inVar.clone();
    assign_field!(outVar.ty = expTypetoTypesType(&inVar.ty));
    outVar
}

pub fn isTuple(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**tp {
        DAE::Type::T_TUPLE { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isMetaTuple(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**tp {
        DAE::Type::T_METATUPLE { .. } => true,
        _ => false,
    });
    b
}

pub fn isRecord(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**tp {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { path: _ },
            ..
        } => true,
        _ => false,
    });
    b
}

pub fn recordHasConstVar(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<bool> {
    let mut hasConstType: bool = false;
    let () = (match &**ty {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { path: _ },
            varLst: __ty_varLst,
            ..
        } => {
            for mut var in &*__ty_varLst.clone() {
                if DAEUtil::isConstVar(metamodelica::AsArg::as_arg(&var)) {
                    hasConstType = true;
                    break;
                }
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Types.recordHasConstVar"));
                    __mm_s.push_str(&*literal!(" failed because input type is not a record."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(hasConstType)
}

pub fn getRecordPath(mut tp: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut p: metamodelica::Ref<Absyn::Path>;
    p = (match &**tp {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { path: __esc_p },
            ..
        } => {
            p = (*__esc_p).clone();
            p.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(p)
}

pub fn isRecordWithOnlyReals(mut tp: &metamodelica::Ref<DAE::Type>) -> Result<bool> {
    let mut b: bool;
    b = (match &**tp {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { path: _ },
            varLst,
            ..
        } => List::all(
            &(List::map(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| {
                getVarType(&__a0)
            })?),
            &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isReal(&__a0))
            },
        )?,
        _ => false,
    });
    Ok(b)
}

pub fn getVarType(mut v: &metamodelica::Ref<DAE::Var>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut tp: metamodelica::Ref<DAE::Type>;
    tp = (match &**v {
        DAE::Var { ty: __esc_tp, .. } => {
            tp = (*__esc_tp).clone();
            tp.clone()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("Types.getVarType failed")],
            )?;
            return Err("fail");
        }
    });
    Ok(tp)
}

pub fn varIsVariable(mut v: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match v {
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { variability: SCode::Variability::VAR { .. }, .. }, .. } => true,
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { variability: SCode::Variability::DISCRETE { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isReal(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut res: bool;
    res = isScalarReal(&(arrayElementType(tp)));
    res
}

pub(crate) fn isScalarReal<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_REAL { .. } => return true,
            DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn isRealOrSubTypeReal(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    let mut lb1: bool;
    let mut lb2: bool;
    lb1 = isReal(&inType);
    lb2 = equivtypes(inType, DAE::T_REAL_DEFAULT().clone());
    b = lb1 || lb2;
    b
}

pub fn isIntegerOrSubTypeInteger(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    let mut lb1: bool;
    let mut lb2: bool;
    lb1 = isInteger(&inType);
    lb2 = equivtypes(inType, DAE::T_INTEGER_DEFAULT().clone());
    b = lb1 || lb2;
    b
}

pub fn isEnumerationOrSubTypeEnumeration(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    let mut lb1: bool;
    let mut lb2: bool;
    lb1 = isEnumeration(&inType);
    lb2 = equivtypes(inType, DAE::T_ENUMERATION_DEFAULT().clone());
    b = lb1 || lb2;
    b
}

fn isClockOrSubTypeClock1(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    let mut lb1: bool;
    let mut lb2: bool;
    let mut lb3: bool;
    lb1 = isClock(&inType);
    lb2 = equivtypes(inType.clone(), DAE::T_CLOCK_DEFAULT().clone());
    lb3 = !(equivtypes(inType, DAE::T_UNKNOWN_DEFAULT().clone()));
    b = lb1 || lb2 && lb3;
    b
}

pub fn isClockOrSubTypeClock(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &*inType {
        DAE::Type::T_FUNCTION { funcResultType: ty, .. } => isClockOrSubTypeClock1(ty.clone()),
        _ => isClockOrSubTypeClock1(inType),
    });
    b
}

pub fn isBooleanOrSubTypeBoolean(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    let mut lb1: bool;
    let mut lb2: bool;
    lb1 = isBoolean(&inType);
    lb2 = equivtypes(inType, DAE::T_BOOL_DEFAULT().clone());
    b = lb1 || lb2;
    b
}

pub fn isStringOrSubTypeString(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    let mut lb1: bool;
    let mut lb2: bool;
    lb1 = isString(&inType);
    lb2 = equivtypes(inType, DAE::T_STRING_DEFAULT().clone());
    b = lb1 || lb2;
    b
}

pub fn isIntegerOrRealOrSubTypeOfEither(mut t: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &*t {
        _ if (isRealOrSubTypeReal(t.clone())) => true,
        _ if (isIntegerOrSubTypeInteger(t.clone())) => true,
        _ => false,
    });
    b
}

pub fn isIntegerOrRealOrBooleanOrSubTypeOfEither(mut t: metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &*t {
        _ if (isRealOrSubTypeReal(t.clone())) => true,
        _ if (isIntegerOrSubTypeInteger(t.clone())) => true,
        _ if (isBooleanOrSubTypeBoolean(t.clone())) => true,
        _ => false,
    });
    b
}

pub(crate) fn isClock(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut res: bool;
    res = isScalarClock(&(arrayElementType(tp)));
    res
}

pub(crate) fn isScalarClock<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_CLOCK { .. } => return true,
            DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn isInteger(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut res: bool;
    res = isScalarInteger(&(arrayElementType(tp)));
    res
}

pub fn isScalarInteger<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_INTEGER { .. } => return true,
            DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn isBoolean(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut res: bool;
    res = isScalarBoolean(&(arrayElementType(tp)));
    res
}

pub fn isScalarBoolean<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_BOOL { .. } => return true,
            DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn integerOrReal(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<()> {
    let () = (match &**inType {
        DAE::Type::T_REAL { .. } => (),
        DAE::Type::T_INTEGER { .. } => (),
        DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. } => {
            integerOrReal(tp)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub fn isNonscalarArray(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (&**inType, &**inDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { .. }, _) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. }, _) => {
                    Ok(isNonscalarArray(metamodelica::AsArg::as_arg(&t), &(metamodelica::nil())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: tys, .. }, _) => {
                    let mut b: bool;
                    b = List::applyAndFold1(metamodelica::AsArg::as_arg(&tys), &fnptr!(boolOr, bool, bool), &move |__a0: metamodelica::Ref<DAE::Type>, __a1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isNonscalarArray(&__a0, &__a1)) }, metamodelica::nil(), false)?;
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBoolean
}

pub fn isArray<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_ARRAY { .. } => return true,
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                inType = var_field!((**inType).complexType, DAE::Type::T_SUBTYPE_BASIC);
                continue '__tco;
            }
            DAE::Type::T_FUNCTION { .. } => {
                inType = var_field!((**inType).funcResultType, DAE::Type::T_FUNCTION);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn isEmptyArray(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inType {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isString(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inType {
        DAE::Type::T_STRING { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isEnumeration(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &*(arrayElementType(inType)) {
        DAE::Type::T_ENUMERATION { .. } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isArrayOrString(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inType) {
        ty if (isArray(metamodelica::AsArg::as_arg(&ty))) => {
            true
        },
        ty if (isString(metamodelica::AsArg::as_arg(&ty))) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn numberOfDimensions<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> i32 {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_ARRAY { ty: t, dims } => {
                let mut n: i32;
                n = numberOfDimensions(t);
                return n + ((dims).len() as i32);
            }
            DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                let mut n: i32;
                {
                    inType = t;
                    continue '__tco;
                }
            }
            _ => return 0,
        }
    }
}

pub fn dimensionsKnown(mut inType: metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inType) {
            Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d, tail: dims }, ty: tp } if (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&d)) && dimensionsKnown(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: dims.clone() }))) => {
                return true
            },
            Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Nil, ty: tp } if (dimensionsKnown(tp.clone())) => {
                return true
            },
            Deref @ DAE::Type::T_ARRAY { .. } => {
                return false
            },
            Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. } => {
                { inType = tp.clone(); continue '__tco; }
            },
            _ => {
                return true
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getDimensionSizes(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::List<i32>> {
    let mut outIntegerLst: metamodelica::List<i32>;
    outIntegerLst = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d, tail: dims }, ty: tp } => {
                    let mut res: metamodelica::List<i32>;
                    let mut i: i32;
                    i = Expression::dimensionSize(metamodelica::AsArg::as_arg(&d))?;
                    res = getDimensionSizes(&(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: dims.clone() })))?;
                    Ok(metamodelica::cons(i, res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: dims }, ty: tp } => {
                    let mut res: metamodelica::List<i32>;
                    res = getDimensionSizes(&(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: dims.clone() })))?;
                    Ok(metamodelica::cons(0, res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Nil, ty: tp } => {
                    let mut res: metamodelica::List<i32>;
                    res = getDimensionSizes(metamodelica::AsArg::as_arg(&tp))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. } => {
                    Ok(getDimensionSizes(metamodelica::AsArg::as_arg(&tp))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (arrayType(inType)) else { return Err("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outIntegerLst)
}

pub fn getDimensionProduct<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> Result<i32> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_ARRAY { dims, ty: tp } => {
                return Ok(({
                    let mut __acc: i32 = 1;
                    for mut d in (dims.clone()).into_iter().cloned() {
                        let __x = Expression::dimensionSize(&(d.clone()))?;
                        __acc *= __x;
                    }
                    __acc
                }) * getDimensionProduct(tp)?);
            }
            DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. } => {
                inType = tp;
                continue '__tco;
            }
            _ => {
                let false = (arrayType(inType)) else {
                    return Err("pattern mismatch");
                };
                return Ok(1);
            }
        }
    }
}

pub fn getDimensionNth(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inDim: i32,
) -> Result<metamodelica::Ref<DAE::Dimension>> {
    let mut outDimension: metamodelica::Ref<DAE::Dimension>;
    outDimension = 'mc: {
        let __mc_input = (&**inType, inDim);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims, .. }, d) => {
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    dim = (dims).get(d.clone())?;
                    Ok(dim.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims }, d) => {
                    let mut dc: i32;
                    dc = ((dims).len() as i32);
                    let true = (d.clone() > dc) else { return Err("pattern mismatch") };
                    Ok(getDimensionNth(metamodelica::AsArg::as_arg(&t), d.clone() - dc)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. }, d) => {
                    Ok(getDimensionNth(metamodelica::AsArg::as_arg(&t), d.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDimension)
}

pub fn setDimensionNth(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inDim: &metamodelica::Ref<DAE::Dimension>,
    mut inDimNth: i32,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &((inType.clone(), inDimNth)) {
        (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, ty }, 1) => {
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![inDim.clone()] })
        },
        (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty }, _) => {
            let mut ty = (*ty).clone();
            let true = (inDimNth > 1) else { return Err("pattern mismatch") };
            ty = setDimensionNth(metamodelica::AsArg::as_arg(&ty), inDim, inDimNth - 1)?;
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outType)
}

pub(crate) fn valuesToVars(
    mut inValuesValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inExpIdentLst: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = (&**inValuesValueLst, &**inExpIdentLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: v, tail: vs }, Deref @ metamodelica::ListNode::Cons { head: id, tail: ids }) => {
                    let mut tp: Type;
                    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    tp = typeOfValue(v.clone())?;
                    rest = valuesToVars(metamodelica::AsArg::as_arg(&vs), metamodelica::AsArg::as_arg(&ids))?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(DAE::Var { name: id.clone(), attributes: DAE::dummyAttrVar().clone(), ty: tp.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("-values_to_vars failed\n"))?;
                    Ok(return Err("fail"))
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

pub fn typeOfValue(mut inValue: metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = inValue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::EMPTY { ty: valType, .. } => {
                    Ok(typeOfValue(valType.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::INTEGER { .. } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::REAL { .. } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::STRING { .. } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::BOOL { .. } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ENUM_LITERAL { name: path, index } => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::pathPrefix(metamodelica::AsArg::as_arg(&path))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(index.clone()), path: path.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: v, tail: vs }, .. } => {
                    let mut tp: Type;
                    let mut dim1: i32;
                    tp = typeOfValue(v.clone())?;
                    dim1 = (((metamodelica::cons(v.clone(), vs.clone()))).len() as i32);
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim1 })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 0 })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::TUPLE { valueLst: vs } => {
                    let mut ts: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    ts = List::map(vs.clone(), &typeOfValue)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_TUPLE { types: ts.clone(), names: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { record_: cname, orderd: vl, comp: ids, index: (-1) } => {
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    vars = valuesToVars(metamodelica::AsArg::as_arg(&vl), metamodelica::AsArg::as_arg(&ids))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: cname.clone() }, varLst: vars.clone(), equalityConstraint: None, usedExternally: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { record_: cname, orderd: vl, comp: ids, index } => {
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut utPath: metamodelica::Ref<Absyn::Path>;
                    let true = (index.clone() >= 0) else { return Err("pattern mismatch") };
                    vars = valuesToVars(metamodelica::AsArg::as_arg(&vl), metamodelica::AsArg::as_arg(&ids))?;
                    utPath = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&cname))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METARECORD { path: cname.clone(), utPath: utPath.clone(), typeVars: metamodelica::nil(), index: index.clone(), fields: vars.clone(), knownSingleton: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::LIST { valueLst: vl } => {
                    let mut tp: Type;
                    let mut ts: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    explist = List::map(vl.clone(), &({ let __pe_b1 = None; move |__pe_a0| ValuesUtil::valueExp(__pe_a0, __pe_b1.clone()) }))?;
                    ts = List::map(vl.clone(), &typeOfValue)?;
                    (_, tp) = listMatchSuperType(&explist, &ts, true)?;
                    tp = boxIfUnboxedType(tp.clone());
                    Ok(metamodelica::Ref::new(DAE::Type::T_METALIST { ty: tp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::OPTION { some: None } => {
                    let mut tp: Type;
                    tp = metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: DAE::T_UNKNOWN_DEFAULT().clone() });
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::OPTION { some: Some(v) } => {
                    let mut tp: Type;
                    tp = boxIfUnboxedType(typeOfValue(v.clone())?);
                    tp = metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: tp.clone() });
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_TUPLE { valueLst: vs } => {
                    let mut ts: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    ts = List::mapMap(vs.clone(), &typeOfValue, &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: ts.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: v, tail: _ } } => {
                    let mut tp: Type;
                    tp = boxIfUnboxedType(typeOfValue(v.clone())?);
                    tp = metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: tp.clone() });
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil } => {
                    let mut tp: Type;
                    tp = metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone() });
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_BOX { value: v } => {
                    let mut tp: Type;
                    tp = typeOfValue(v.clone())?;
                    Ok(boxIfUnboxedType(tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::NORETCALL { .. } => {
                    Ok(DAE::T_NORETCALL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { .. } } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_TYPENAME }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { .. } } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { .. } } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { .. } } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_MODIFICATION }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.typeOfValue failed: ")); __mm_s.push_str(&*ValuesDump::valString(metamodelica::AsArg::as_arg(&v))?); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

pub fn basicType(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inType {
        DAE::Type::T_INTEGER { .. } => true,
        DAE::Type::T_REAL { .. } => true,
        DAE::Type::T_STRING { .. } => true,
        DAE::Type::T_BOOL { .. } => true,
        DAE::Type::T_CLOCK { .. } => true,
        DAE::Type::T_ENUMERATION { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn extendsBasicType(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inType {
        DAE::Type::T_SUBTYPE_BASIC { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn derivedBasicType<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                inType = var_field!((**inType).complexType, DAE::Type::T_SUBTYPE_BASIC);
                continue '__tco;
            }
            _ => return inType.clone(),
        }
    }
}

pub fn arrayType(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inType {
        DAE::Type::T_ARRAY { .. } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn setVarInput(mut var: metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::Var> {
    let mut outV: metamodelica::Ref<DAE::Var>;
    let mut attrs: metamodelica::Ref<DAE::Attributes>;
    outV = var;
    attrs = outV.attributes.clone();
    assign_field!(attrs.direction = openmodelica_ast::Absyn::Direction::INPUT);
    assign_field!(outV.attributes = attrs);
    outV
}

pub fn setVarDefaultInput(mut var: metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::Var> {
    let mut outV: metamodelica::Ref<DAE::Var>;
    let mut attrs: metamodelica::Ref<DAE::Attributes>;
    outV = var;
    attrs = outV.attributes.clone();
    assign_field!(
        attrs.connectorType = openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        attrs.variability = openmodelica_frontend_types::SCode::Variability::VAR,
        attrs.direction = openmodelica_ast::Absyn::Direction::INPUT,
        attrs.innerOuter = openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        attrs.visibility = openmodelica_frontend_types::SCode::Visibility::PUBLIC
    );
    assign_field!(outV.attributes = attrs);
    outV
}

pub fn setVarProtected(mut var: metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::Var> {
    let mut outV: metamodelica::Ref<DAE::Var>;
    let mut attrs: metamodelica::Ref<DAE::Attributes>;
    outV = var;
    attrs = outV.attributes.clone();
    assign_field!(attrs.visibility = openmodelica_frontend_types::SCode::Visibility::PROTECTED);
    assign_field!(outV.attributes = attrs);
    outV
}

fn setVarType(
    mut var: metamodelica::Ref<DAE::Var>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Var> {
    let mut outV: metamodelica::Ref<DAE::Var> = var;
    assign_field!(outV.ty = ty);
    outV
}

pub(crate) fn semiEquivTypes(
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
) -> bool {
    let mut outEquiv: bool;
    let mut ty1: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut dims1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut dims2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    if arrayType(&inType1) && arrayType(&inType2) {
        (ty1, dims1) = TypesDump::flattenArrayType(&inType1);
        (ty2, dims2) = TypesDump::flattenArrayType(&inType2);
        outEquiv = equivtypes(inType1, inType2) && ((dims1).len() as i32) == ((dims2).len() as i32);
    } else if !(arrayType(&inType1)) && !(arrayType(&inType2)) {
        outEquiv = equivtypes(inType1, inType2);
    } else {
        outEquiv = false;
    }
    outEquiv
}

pub fn equivtypes(mut t1: metamodelica::Ref<DAE::Type>, mut t2: metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = subtype(t1.clone(), t2.clone(), true) && subtype(t2, t1, true);
    outBoolean
}

pub fn equivtypesOrRecordSubtypeOf(mut t1: metamodelica::Ref<DAE::Type>, mut t2: metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = subtype(t1.clone(), t2.clone(), false) && subtype(t2, t1, false);
    outBoolean
}

pub fn subtype(
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
    mut requireRecordNamesEqual: bool,
) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (inType1.clone(), inType2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ANYTYPE { .. }, _) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_ANYTYPE { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_STRING { .. }, Deref @ DAE::Type::T_STRING { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_BOOL { .. }, Deref @ DAE::Type::T_BOOL { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_CLOCK { .. }, Deref @ DAE::Type::T_CLOCK { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { names: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { .. }, Deref @ DAE::Type::T_ENUMERATION { names: Deref @ metamodelica::ListNode::Nil, .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { names: names1, .. }, Deref @ DAE::Type::T_ENUMERATION { names: names2, .. }) => {
                    let mut res: bool;
                    res = List::isEqualOnTrue(names1.clone(), names2.clone(), &fnptr!(stringEq, ArcStr, ArcStr))?;
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: dlst1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, ty: t1 }, Deref @ DAE::Type::T_ARRAY { dims: dlst2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, ty: t2 }) => {
                    let true = (Expression::dimsEqual(metamodelica::AsArg::as_arg(&dlst1), metamodelica::AsArg::as_arg(&dlst2))?) else { return Err("pattern mismatch") };
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: dlst2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, ty: t2 }) => {
                    let true = (Expression::dimensionsEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    let true = (subtype(t1.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t2.clone(), dims: dlst2.clone() }), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: dlst1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, ty: t1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }) => {
                    let true = (Expression::dimensionsEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    let true = (subtype(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t1.clone(), dims: dlst1.clone() }), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, Deref @ DAE::Type::T_ARRAY { ty: t2, .. }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, Deref @ DAE::Type::T_ARRAY { ty: t2, .. }) => {
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }) => {
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: p1 }, .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: p2 }, .. }) => {
                    Ok(AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: st1, varLst: els1, .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: st2, varLst: els2, .. }) => {
                    let true = (classTypeEqualIfRecord(metamodelica::AsArg::as_arg(&st1), metamodelica::AsArg::as_arg(&st2)) || !(requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    let true = (((els1).len() as i32) == ((els2).len() as i32)) else { return Err("pattern mismatch") };
                    let true = (subtypeVarlist(els1.clone(), metamodelica::AsArg::as_arg(&els2))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp1, .. }, tp2) => {
                    let mut res: bool;
                    res = subtype(tp1.clone(), tp2.clone(), requireRecordNamesEqual);
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tp1, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp2, .. }) => {
                    let mut res: bool;
                    res = subtype(tp1.clone(), tp2.clone(), requireRecordNamesEqual);
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: type_list1, .. }, Deref @ DAE::Type::T_TUPLE { types: type_list2, .. }) => {
                    let true = (subtypeTypelist(metamodelica::AsArg::as_arg(&type_list1), metamodelica::AsArg::as_arg(&type_list2), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METALIST { ty: t1 }, Deref @ DAE::Type::T_METALIST { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { ty: t1 }, Deref @ DAE::Type::T_METAARRAY { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: tList1 }, Deref @ DAE::Type::T_METATUPLE { types: tList2 }) => {
                    let mut res: bool;
                    res = subtypeTypelist(metamodelica::AsArg::as_arg(&tList1), metamodelica::AsArg::as_arg(&tList2), requireRecordNamesEqual);
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAOPTION { ty: t1 }, Deref @ DAE::Type::T_METAOPTION { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METABOXED { ty: t1 }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2) => {
                    let true = (isBoxedType(metamodelica::AsArg::as_arg(&t2))) else { return Err("pattern mismatch") };
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let true = (isBoxedType(metamodelica::AsArg::as_arg(&t1))) else { return Err("pattern mismatch") };
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAPOLYMORPHIC { name: l1 }, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: l2 }) => {
                    Ok(metamodelica::stringEq(&l1, &l2))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_UNKNOWN { .. }, _) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_UNKNOWN { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_NORETCALL { .. }, Deref @ DAE::Type::T_NORETCALL { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Type::T_FUNCTION { funcArg: farg1, funcResultType: t1, .. }, Deref @ DAE::Type::T_FUNCTION { funcArg: farg2, funcResultType: t2, .. }) => {
                            let mut tList1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut tList2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut t1 = (*t1).clone();
                            let mut t2 = (*t2).clone();
                            tList1 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut t in (farg1.clone()).into_iter().cloned() {
                            let __x = (traverseType(funcArgType(&(t.clone())), 1, &unboxedTypeTraverseHelper)?).0;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            tList2 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut t in (farg2.clone()).into_iter().cloned() {
                            let __x = (traverseType(funcArgType(&(t.clone())), 1, &unboxedTypeTraverseHelper)?).0;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            (t1, _) = traverseType(t1.clone(), 1, &unboxedTypeTraverseHelper)?;
                            (t2, _) = traverseType(t2.clone(), 1, &unboxedTypeTraverseHelper)?;
                            let true = (subtypeTypelist(&tList1, &tList2, requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                            let true = (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) else { return Err("pattern mismatch") };
                            Ok(true)
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: t1 }, Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METARECORD { path: p1, .. }, Deref @ DAE::Type::T_METARECORD { path: p2, .. }) => {
                    Ok(AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAUNIONTYPE { path: p1, .. }, Deref @ DAE::Type::T_METARECORD { utPath: p2, .. }) => {
                    Ok(if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) {subtypeTypelist(var_field!((*inType1).typeVars, DAE::Type::T_METAUNIONTYPE), var_field!((*inType2).typeVars, DAE::Type::T_METARECORD), requireRecordNamesEqual)} else {false})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METARECORD { knownSingleton: b1, utPath: p1, .. }, Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: b2, path: p2, .. }) => {
                    Ok(if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2)) && (b1.clone() || b2.clone())) {subtypeTypelist(var_field!((*inType1).typeVars, DAE::Type::T_METARECORD), var_field!((*inType2).typeVars, DAE::Type::T_METAUNIONTYPE), requireRecordNamesEqual)} else {false})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAUNIONTYPE { path: p1, .. }, Deref @ DAE::Type::T_METAUNIONTYPE { path: p2, .. }) => {
                    Ok(if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) {subtypeTypelist(var_field!((*inType1).typeVars, DAE::Type::T_METAUNIONTYPE), var_field!((*inType2).typeVars, DAE::Type::T_METAUNIONTYPE), requireRecordNamesEqual)} else {false})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_CODE { ty: c1 }, Deref @ DAE::Type::T_CODE { ty: c2 }) => {
                    Ok(c1.clone() == c2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATYPE { ty: t1 }, Deref @ DAE::Type::T_METATYPE { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, Deref @ DAE::Type::T_METATYPE { ty: t2 }) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATYPE { ty: t1 }, t2) => {
                    Ok(subtype(t1.clone(), t2.clone(), requireRecordNamesEqual))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBoolean
}

fn subtypeTypelist<'__b>(
    mut inTypeLst1: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inTypeLst2: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut requireRecordNamesEqual: bool,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match (inTypeLst1, inTypeLst2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return true
            },
            (Deref @ metamodelica::ListNode::Cons { head: t1, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: t2, tail: rest2 }) if (subtype(t1.clone(), t2.clone(), requireRecordNamesEqual)) => {
                { (inTypeLst1, inTypeLst2, requireRecordNamesEqual) = (rest1, rest2, requireRecordNamesEqual); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn subtypeVarlist(
    mut inVarLst1: metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inVarLst2: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (inVarLst1, &**inVarLst2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (l, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: n, ty: t2, .. }, tail: vs }) => {
                    let mut t1: metamodelica::Ref<DAE::Type>;
                    let __arc1 = varlistLookup(metamodelica::AsArg::as_arg(&l), metamodelica::AsArg::as_arg(&n))?;
                    let DAE::TYPES_VAR { ty: __pa0, .. } = &*__arc1;
                    t1 = metamodelica::Own::own(__pa0);
                    let true = (subtype(t1.clone(), t2.clone(), false)) else { return Err("pattern mismatch") };
                    Ok(subtypeVarlist(l.clone(), metamodelica::AsArg::as_arg(&vs)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBoolean
}

pub(crate) fn varlistLookup(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inIdent: &ArcStr,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    let mut name: ArcStr;
    for mut var in &**inVarLst {
        let __arc1 = var.clone();
        let DAE::TYPES_VAR { name: __pa0, .. } = &*__arc1;
        name = metamodelica::Own::own(__pa0);
        if metamodelica::stringEq(&name, &inIdent) {
            outVar = var.clone();
            return Ok(outVar);
        }
    }
    return Err("fail");
    Ok(outVar)
}

pub(crate) fn lookupComponent(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    outVar = 'mc: {
        let __mc_input = (inType, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, n) => {
                    let mut v: metamodelica::Ref<DAE::Var>;
                    let true = (basicType(metamodelica::AsArg::as_arg(&t))) else { return Err("pattern mismatch") };
                    v = lookupInBuiltin(metamodelica::AsArg::as_arg(&t), n.clone())?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { varLst: cs, .. }, id) => {
                    let mut v: metamodelica::Ref<DAE::Var>;
                    v = lookupComponent2(cs.clone(), id.clone())?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst: cs, .. }, id) => {
                    let mut v: metamodelica::Ref<DAE::Var>;
                    v = lookupComponent2(cs.clone(), id.clone())?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty: Deref @ DAE::Type::T_COMPLEX { varLst: cs, .. } }, id) => {
                    let mut v: metamodelica::Ref<DAE::Var>;
                    v = lookupComponent2(cs.clone(), id.clone())?;
                    assign_field!(v.ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: v.ty.clone(), dims: list![dim.clone()] }));
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty: Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst: cs, .. } }, id) => {
                    let mut v: metamodelica::Ref<DAE::Var>;
                    v = lookupComponent2(cs.clone(), id.clone())?;
                    assign_field!(v.ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: v.ty.clone(), dims: list![dim.clone()] }));
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVar)
}

fn lookupInBuiltin(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    outVar = (::match_deref::match_deref! { match &((&**inType, inIdent)) {
        (Deref @ DAE::Type::T_REAL { varLst: cs }, id) => {
            let mut v: metamodelica::Ref<DAE::Var>;
            v = lookupComponent2(cs.clone(), id.clone())?;
            v
        },
        (Deref @ DAE::Type::T_INTEGER { varLst: cs }, id) => {
            let mut v: metamodelica::Ref<DAE::Var>;
            v = lookupComponent2(cs.clone(), id.clone())?;
            v
        },
        (Deref @ DAE::Type::T_STRING { varLst: cs }, id) => {
            let mut v: metamodelica::Ref<DAE::Var>;
            v = lookupComponent2(cs.clone(), id.clone())?;
            v
        },
        (Deref @ DAE::Type::T_BOOL { varLst: cs }, id) => {
            let mut v: metamodelica::Ref<DAE::Var>;
            v = lookupComponent2(cs.clone(), id.clone())?;
            v
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "quantity") => {
            metamodelica::Ref::new(DAE::Var { name: literal!("quantity"), attributes: DAE::dummyAttrParam().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "min") => {
            metamodelica::Ref::new(DAE::Var { name: literal!("min"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(1), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("min,max")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "max") => {
            metamodelica::Ref::new(DAE::Var { name: literal!("max"), attributes: DAE::dummyAttrParam().clone(), ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(2), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: list![literal!("min,max")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "start") => {
            metamodelica::Ref::new(DAE::Var { name: literal!("start"), attributes: DAE::dummyAttrParam().clone(), ty: DAE::T_BOOL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "fixed") => {
            metamodelica::Ref::new(DAE::Var { name: literal!("fixed"), attributes: DAE::dummyAttrParam().clone(), ty: DAE::T_BOOL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })
        },
        (Deref @ DAE::Type::T_ENUMERATION { index: Some(_), .. }, Deref @ "enable") => {
            metamodelica::Ref::new(DAE::Var { name: literal!("enable"), attributes: DAE::dummyAttrParam().clone(), ty: DAE::T_BOOL_DEFAULT().clone(), binding: metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), bind_from_outside: false, constOfForIteratorRange: None })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outVar)
}

fn lookupComponent2(
    mut inVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::Ref<DAE::Var>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inVarLst, inIdent)) {
            (Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ DAE::Var { name: n, .. }, tail: _ }, m) if (stringEq(&n, &m)) => {
                return Ok(v.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: vs }, n) => {
                let mut v: metamodelica::Ref<DAE::Var>;
                { (inVarLst, inIdent) = (vs.clone(), n.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn makeArray(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inArrayDim: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inArrayDim) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut t = inType;
            t
        },
        l => {
            let mut t = inType;
            let mut len: i32;
            len = ((l).len() as i32);
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t, dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: len })] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub(crate) fn makeArraySubscripts(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut lst: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inType, &**lst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(t.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(t.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: _ }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(t.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: _ }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(t.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() })] }), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(t.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: _ }, tail: rest }) => {
                    let mut t = (*t).clone();
                    t = makeArraySubscripts(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(t.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

pub fn liftArray(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: inType,
        dims: list![inDimension],
    });
    outType
}

pub fn liftList(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = metamodelica::Ref::new(DAE::Type::T_METALIST { ty: inType });
    outType
}

pub fn liftArrayListDims(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type> = inType;
    for mut dim in &*inDimensions.reverse() {
        outType = metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: outType,
            dims: list![dim.clone()],
        });
    }
    outType
}

pub(crate) fn liftArrayListDimsReverse(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> metamodelica::Ref<DAE::Type> {
    let mut ty: metamodelica::Ref<DAE::Type> = inType;
    for mut dim in &**dims {
        ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: ty,
            dims: list![dim.clone()],
        });
    }
    ty
}

pub fn liftTypeWithDims(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    if (inDims).is_empty() {
        outType = inType;
        return Ok(outType);
    }
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            metamodelica::print(literal!("Can not handle this yet!!"));
            return Err("fail")
        },
        Deref @ DAE::Type::T_ARRAY { ty, dims } => {
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: listAppend(dims.clone(), inDims) })
        },
        _ => {
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inType, dims: inDims })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub fn liftTypeWithDimExps(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDimExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match inDimExps {
        Deref @ metamodelica::ListNode::Nil => {
            let mut ty = inType;
            ty
        },
        Deref @ metamodelica::ListNode::Cons { head: d, tail: rest } => {
            let mut ty = inType;
            liftArray(liftTypeWithDimExps(ty, rest)?, metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: d.clone() }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub fn liftArrayRight(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inIntegerOption: metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty } => {
            let mut d = inIntegerOption;
            let mut ty_1: Type;
            ty_1 = liftArrayRight(ty.clone(), d);
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty_1, dims: list![dim.clone()] })
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci, varLst: varlst, complexType: ty, equalityConstraint: ec } if (!(((TypesDump::getDimensions(metamodelica::AsArg::as_arg(&ty)))).is_empty())) => {
            let mut d = inIntegerOption;
            let mut ty_1: Type;
            ty_1 = liftArrayRight(ty.clone(), d);
            metamodelica::Ref::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci.clone(), varLst: varlst.clone(), complexType: ty_1, equalityConstraint: ec.clone() })
        },
        tty => {
            let mut d = inIntegerOption;
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tty.clone(), dims: list![d] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn unliftArray<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_ARRAY { ty, .. } => return Ok(ty.clone()),
            DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn unliftArrayOrList(
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<(metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Dimension>)> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    (outType, dim) = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_METALIST { ty } => {
            (boxIfUnboxedType(ty.clone()), openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())
        },
        Deref @ DAE::Type::T_METAARRAY { ty } => {
            (boxIfUnboxedType(ty.clone()), openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())
        },
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __esc_dim, tail: Deref @ metamodelica::ListNode::Nil }, ty } => {
            dim = (*__esc_dim).clone();
            (ty.clone(), dim.clone())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
            let mut ty = (*ty).clone();
            (ty, dim) = unliftArrayOrList(ty.clone())?;
            (ty.clone(), dim)
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: ty, .. } => {
            unliftArrayOrList(ty.clone())?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outType, dim))
}

pub fn arrayElementType<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_ARRAY { .. } => {
                inType = var_field!((**inType).ty, DAE::Type::T_ARRAY);
                continue '__tco;
            }
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                if ((TypesDump::getDimensions(var_field!((**inType).complexType, DAE::Type::T_SUBTYPE_BASIC)))
                    .is_empty())
                {
                    return inType.clone();
                } else {
                    {
                        inType = var_field!((**inType).complexType, DAE::Type::T_SUBTYPE_BASIC);
                        continue '__tco;
                    }
                }
            }
            DAE::Type::T_FUNCTION { .. } => {
                inType = var_field!((**inType).funcResultType, DAE::Type::T_FUNCTION);
                continue '__tco;
            }
            _ => return inType.clone(),
        }
    }
}

pub fn setArrayElementType(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inBaseType: &metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inType {
        DAE::Type::T_ARRAY { ty, dims } => {
            let mut ty = (*ty).clone();
            ty = setArrayElementType(metamodelica::AsArg::as_arg(&ty), inBaseType);
            metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: ty.clone(),
                dims: dims.clone(),
            })
        }
        _ => inBaseType.clone(),
    });
    outType
}

pub fn makeFunctionType(
    mut p: metamodelica::Ref<Absyn::Path>,
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut functionAttributes: DAE::FunctionAttributes,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut invl: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut outvl: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let mut rettype: Type;
    invl = getInputVars(vl.clone())?;
    outvl = getOutputVars(vl)?;
    fargs = makeFargsList(invl)?;
    rettype = makeReturnType(outvl);
    outType = metamodelica::Ref::new(DAE::Type::T_FUNCTION {
        funcArg: fargs,
        funcResultType: rettype,
        functionAttributes: functionAttributes,
        path: p,
    });
    Ok(outType)
}

pub fn extendsFunctionTypeArgs(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inOutputElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inBooltLst: metamodelica::List<bool>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut tysrc: metamodelica::Ref<Absyn::Path>;
    let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let mut fargs1: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let mut newfargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let mut rettype: metamodelica::Ref<DAE::Type>;
    let mut functionAttributes: DAE::FunctionAttributes;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*inType)) {
        Deref @ DAE::Type::T_FUNCTION { funcArg: __pa0, funcResultType: __pa1, functionAttributes: __pa2, path: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fargs = metamodelica::Own::own(__pa0);
    rettype = metamodelica::Own::own(__pa1);
    functionAttributes = metamodelica::Own::own(__pa2);
    tysrc = metamodelica::Own::own(__pa3);
    (fargs1, _) = List::splitOnBoolList(fargs.clone(), inBooltLst)?;
    newfargs = List::threadMap(
        inElementLst,
        fargs1,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::FuncArg>| {
            makeElementFarg(&__a0, &__a1)
        },
    )?;
    newfargs = listAppend(fargs, newfargs);
    outType = metamodelica::Ref::new(DAE::Type::T_FUNCTION {
        funcArg: newfargs,
        funcResultType: rettype,
        functionAttributes: functionAttributes,
        path: tysrc,
    });
    Ok(outType)
}

pub fn setFunctionNoReturn(mut ty: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    let mut ty: metamodelica::Ref<DAE::Type> = ty;
    ty = (match &*ty {
        DAE::Type::T_FUNCTION {
            funcArg: fargs,
            funcResultType: rettype,
            functionAttributes:
                DAE::FunctionAttributes {
                    inline: inl,
                    generateEvents: ge,
                    purity: pu,
                    isFunctionPointer: fp,
                    isBuiltin: bi,
                    functionParallelism: par,
                    noReturn: _,
                },
            path: p,
        } => metamodelica::Ref::new(DAE::Type::T_FUNCTION {
            funcArg: fargs.clone(),
            funcResultType: rettype.clone(),
            functionAttributes: DAE::FunctionAttributes {
                inline: inl.clone(),
                generateEvents: ge.clone(),
                purity: pu.clone(),
                isFunctionPointer: fp.clone(),
                isBuiltin: bi.clone(),
                functionParallelism: par.clone(),
                noReturn: DAE::NoReturn::NORETURN.clone(),
            },
            path: p.clone(),
        }),
        _ => ty,
    });
    ty
}

fn makeElementReturnType(
    mut inElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inElementLst) {
        Deref @ metamodelica::ListNode::Nil => {
            openmodelica_frontend_types::DAE::Type::interned_T_NORETCALL()
        },
        Deref @ metamodelica::ListNode::Cons { head: element, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut ty: Type;
            ty = makeElementReturnTypeSingle(metamodelica::AsArg::as_arg(&element))?;
            ty
        },
        elements => {
            let mut element: metamodelica::Ref<DAE::Element> = <metamodelica::Ref<DAE::Element> as ::std::default::Default>::default();
            let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut names: metamodelica::List<ArcStr>;
            let mut namesOpt: Option<metamodelica::List<ArcStr>>;
            types = metamodelica::nil();
            names = metamodelica::nil();
            for mut element in &*elements.clone() {
                let mut element = element.clone();
                types = metamodelica::cons(makeElementReturnTypeSingle(&element)?, types);
                names = metamodelica::cons(DAEUtil::varName(&element)?, names);
            }
            if (names).is_empty() {
                namesOpt = None;
            } else {
                namesOpt = Some(names.reverse());
            }
            metamodelica::Ref::new(DAE::Type::T_TUPLE { types: types.reverse(), names: namesOpt })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn makeElementReturnTypeSingle(
    mut inElement: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inElement {
        DAE::Element::VAR { ty, .. } => ty.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outType)
}

pub fn getNthEnumLiteral<'__b>(
    mut ty: &'__b metamodelica::Ref<DAE::Type>,
    mut n: i32,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        match &**ty {
            DAE::Type::T_ENUMERATION { .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
                    name: AbsynUtil::joinPaths(
                        var_field!((**ty).path, DAE::Type::T_ENUMERATION).clone(),
                        metamodelica::Ref::new(Absyn::Path::IDENT {
                            name: (var_field!((**ty).names, DAE::Type::T_ENUMERATION)).get(n)?,
                        }),
                    )?,
                    index: n,
                }));
            }
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                (ty, n) = (var_field!((**ty).complexType, DAE::Type::T_SUBTYPE_BASIC), n);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn makeEnumerationType(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { index: None, path: p, names, literalVarLst: vars, attributeLst: attrs } => {
                    let mut attr_names: metamodelica::List<ArcStr>;
                    let mut vars = (*vars).clone();
                    let mut attrs = (*attrs).clone();
                    vars = makeEnumerationType1(p.clone(), metamodelica::AsArg::as_arg(&vars), names.clone(), 1)?;
                    attr_names = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?;
                    attrs = makeEnumerationType1(p.clone(), metamodelica::AsArg::as_arg(&attrs), attr_names.clone(), 1)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: names.clone(), literalVarLst: vars.clone(), attributeLst: attrs.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, .. } => {
                    Ok(makeEnumerationType(inPath, metamodelica::AsArg::as_arg(&ty))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.makeEnumerationType failed on ")); __mm_s.push_str(&*TypesDump::printTypeStr(inType.clone())); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

pub(crate) fn makeEnumerationType1(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inVarLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inNames: metamodelica::List<ArcStr>,
    mut inIdx: i32,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    outVarLst = (::match_deref::match_deref! { match inVarLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name, attributes, ty: _, binding, bind_from_outside: bsrc, constOfForIteratorRange: cnstForRange }, tail: xs } => {
            let mut p = inPath;
            let mut names = inNames;
            let mut idx = inIdx;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            let mut t: metamodelica::Ref<DAE::Type>;
            let mut var: metamodelica::Ref<DAE::Var>;
            vars = makeEnumerationType1(p.clone(), xs, names.clone(), idx + 1)?;
            t = metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(idx), path: p, names: names, literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() });
            var = metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: attributes.clone(), ty: t, binding: binding.clone(), bind_from_outside: bsrc.clone(), constOfForIteratorRange: cnstForRange.clone() });
            metamodelica::cons(var, vars)
        },
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarLst)
}

fn getInputVars(
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut vl_1: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    vl_1 = List::select(
        vl,
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isInputVar(&__a0))
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Var>) -> Result<bool> + 'static>),
    )?;
    Ok(vl_1)
}

fn getOutputVars(
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut vl_1: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    vl_1 = List::select(
        vl,
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isOutputVar(&__a0))
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Var>) -> Result<bool> + 'static>),
    )?;
    Ok(vl_1)
}

pub fn getFixedVarAttributeParameterOrConstant(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut fix: bool;
    match '__try0: {
        fix = unwrap_break_err!(getFixedVarAttribute(tp), '__try0);
        Ok::<_, &'static str>((fix.clone(),))
    } {
        Ok((__try0_o0,)) => {
            fix = __try0_o0;
        }
        Err(_) => {
            fix = true;
        }
    }
    fix
}

pub(crate) fn getFixedVarAttribute(mut tp: &metamodelica::Ref<DAE::Type>) -> Result<bool> {
    let mut fixed: bool = false;
    fixed = 'mc: {
        let __mc_input = &**tp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::VALBOUND { valBound: Deref @ Values::Value::BOOL { boolean: fixed }, .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::BOOL { boolean: fixed }), .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::BCONST { bool: fixed }, .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: vars } } => {
                    let mut fixed: bool = fixed.clone();
                    fixed = getFixedVarAttribute(&(metamodelica::Ref::new(DAE::Type::T_REAL { varLst: vars.clone() })))?;
                    Ok((fixed, fixed.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            fixed = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::VALBOUND { valBound: Deref @ Values::Value::BOOL { boolean: fixed }, .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::BOOL { boolean: fixed }), .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::BCONST { bool: fixed }, .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: vars } } => {
                    let mut fixed: bool = fixed.clone();
                    fixed = getFixedVarAttribute(&(metamodelica::Ref::new(DAE::Type::T_INTEGER { varLst: vars.clone() })))?;
                    Ok((fixed, fixed.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            fixed = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::VALBOUND { valBound: Deref @ Values::Value::BOOL { boolean: fixed }, .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::BOOL { boolean: fixed }), .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: Deref @ "fixed", binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::BCONST { bool: fixed }, .. }, .. }, tail: _ } } => {
                    Ok(fixed.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: vars } } => {
                    let mut fixed: bool = fixed.clone();
                    fixed = getFixedVarAttribute(&(metamodelica::Ref::new(DAE::Type::T_BOOL { varLst: vars.clone() })))?;
                    Ok((fixed, fixed.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            fixed = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, .. } => {
                    let mut result: bool;
                    result = getFixedVarAttribute(metamodelica::AsArg::as_arg(&ty))?;
                    Ok(result)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(fixed)
}

pub fn getConnectorVars(
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    outVars = (match &**inType {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::CONNECTOR { .. },
            varLst: vars,
            ..
        } => vars.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outVars)
}

pub fn isInputVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut b: bool;
    b = (match &**inVar {
        DAE::Var { attributes: attr, .. } => isInputAttr(attr) && isPublicAttr(attr),
    });
    b
}

pub fn isOutputVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut b: bool;
    b = (match &**inVar {
        DAE::Var { attributes: attr, .. } => isOutputAttr(attr) && isPublicAttr(attr),
    });
    b
}

pub(crate) fn isInputAttr(mut inAttributes: &metamodelica::Ref<DAE::Attributes>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inAttributes {
        DAE::Attributes {
            direction: Absyn::Direction::INPUT { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isOutputAttr(mut inAttributes: &metamodelica::Ref<DAE::Attributes>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inAttributes {
        DAE::Attributes {
            direction: Absyn::Direction::OUTPUT { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isBidirAttr(mut inAttributes: &metamodelica::Ref<DAE::Attributes>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inAttributes {
        DAE::Attributes {
            direction: Absyn::Direction::BIDIR { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isPublicAttr(mut inAttributes: &metamodelica::Ref<DAE::Attributes>) -> bool {
    let mut outIsPublic: bool;
    outIsPublic = (match &**inAttributes {
        DAE::Attributes {
            visibility: SCode::Visibility::PUBLIC { .. },
            ..
        } => true,
        _ => false,
    });
    outIsPublic
}

pub(crate) fn isConstAttr(mut inAttributes: &metamodelica::Ref<DAE::Attributes>) -> bool {
    let mut outIsPublic: bool;
    outIsPublic = (match &**inAttributes {
        DAE::Attributes {
            variability: SCode::Variability::CONST { .. },
            ..
        } => true,
        _ => false,
    });
    outIsPublic
}

pub fn isPublicVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut b: bool;
    b = (match &**inVar {
        DAE::Var { .. } => isPublicAttr(&inVar.attributes),
    });
    b
}

pub(crate) fn isConstVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut b: bool;
    b = (match &**inVar {
        DAE::Var { .. } => isConstAttr(&inVar.attributes),
    });
    b
}

// This used in creation of record constructors to decide wether a variable should be
// part of the constructor signature or not. If a var is modifiable from outside then
// it is part of the construvtor signature.
pub fn isModifiableTypesVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<bool> {
    let mut b: bool;
    if !(isPublicVar(inVar)) {
        if (getBindingExpOptional(inVar)).is_none() {
            Error::addSourceMessage(
                &(Error::MISSING_BINDING_PROTECTED_RECORD_VAR.clone()),
                list![TypesDump::getVarName(inVar)],
                &(Absyn::dummyInfo.clone()),
            )?;
        }
        b = false;
        return Ok(b);
    }
    if isConstVar(inVar) && (getBindingExpOptional(inVar)).is_some() {
        b = false;
        return Ok(b);
    }
    b = true;
    Ok(b)
}

pub fn getBindingExpOptional(mut inVar: &metamodelica::Ref<DAE::Var>) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match inVar {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp, .. }, .. } => {
            Some(exp.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

// This should be removed. It is used in cevalScript now. cevalScript should be updated
// and this removed.
pub fn getBindingExp(
    mut inVar: &metamodelica::Ref<DAE::Var>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match inVar {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp, .. }, .. } => {
            exp.clone()
        },
        Deref @ DAE::Var { name, binding: Deref @ DAE::Binding::UNBOUND { .. }, .. } => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Record '")); __mm_s.push_str(&*AbsynUtil::pathString(inPath, literal!("."), true, false)?); __mm_s.push_str(&*literal!("' member '")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("' has no default value and is not modifiable by a constructor function.\n")); ArcStr::from(__mm_s) };
            Error::addCompilerWarning(r#str)?;
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn makeFargsList(
    mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::FuncArg>>> {
    let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    fargs = List::map(vars, &move |__a0: metamodelica::Ref<DAE::Var>| makeFarg(&__a0))?;
    Ok(fargs)
}

fn makeFarg(mut variable: &metamodelica::Ref<DAE::Var>) -> Result<metamodelica::Ref<DAE::FuncArg>> {
    let mut farg: metamodelica::Ref<DAE::FuncArg>;
    farg = (::match_deref::match_deref! { match variable {
        Deref @ DAE::Var { name: n, attributes: Deref @ DAE::Attributes { variability: var, parallelism: par, .. }, ty, binding: bnd, .. } => {
            let mut c: DAE::Const;
            let mut p: DAE::VarParallelism;
            let mut oexp: Option<metamodelica::Ref<DAE::Exp>>;
            c = variabilityToConst(var.clone());
            p = DAEUtil::scodePrlToDaePrl(par.clone());
            oexp = DAEUtil::bindingExp(bnd)?;
            metamodelica::Ref::new(DAE::FuncArg { name: n.clone(), ty: ty.clone(), r#const: c, par: p, defaultBinding: oexp })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(farg)
}

fn makeElementFarg(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut inFarg: &metamodelica::Ref<DAE::FuncArg>,
) -> Result<metamodelica::Ref<DAE::FuncArg>> {
    let mut farg: metamodelica::Ref<DAE::FuncArg>;
    farg = (match &**inElement {
        DAE::Element::VAR { componentRef: cref, .. } => {
            let mut name: ArcStr;
            name = ComponentReferenceBasics::crefLastIdent(cref)?;
            setFuncArgName(inFarg, name)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(farg)
}

fn makeReturnType(mut inVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inVarLst) {
        Deref @ metamodelica::ListNode::Nil => {
            openmodelica_frontend_types::DAE::Type::interned_T_NORETCALL()
        },
        Deref @ metamodelica::ListNode::Cons { head: var, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut ty: Type;
            ty = makeReturnTypeSingle(metamodelica::AsArg::as_arg(&var));
            ty
        },
        vl => {
            metamodelica::Ref::new(DAE::Type::T_TUPLE { types: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut v in (vl.clone()).into_iter().cloned() {
            let __x = makeReturnTypeSingle(&(v.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), names: Some(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut v in (vl.clone()).into_iter().cloned() {
            let __x = TypesDump::getVarName(&(v.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

fn makeReturnTypeSingle(mut inVar: &metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inVar {
        DAE::Var { ty, .. } => ty.clone(),
    });
    outType
}

pub(crate) fn isParameterVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<()> {
    ::match_deref::match_deref! { match &((*inVar)) {
        Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { variability: SCode::Variability::PARAM { .. }, visibility: SCode::Visibility::PUBLIC { .. }, .. }, .. } => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub fn isConstant(mut c: DAE::Const) -> bool {
    let mut b: bool;
    b = (match c {
        DAE::Const::C_CONST { .. } => true,
        _ => false,
    });
    b
}

pub fn isParameter(mut c: DAE::Const) -> bool {
    let mut b: bool;
    b = (match c {
        DAE::Const::C_PARAM { .. } => true,
        _ => false,
    });
    b
}

pub fn isParameterOrConstant(mut c: DAE::Const) -> bool {
    let mut b: bool;
    b = (match c {
        DAE::Const::C_CONST { .. } => true,
        DAE::Const::C_PARAM { .. } => true,
        _ => false,
    });
    b
}

pub fn isVar(mut inConst: DAE::Const) -> bool {
    let mut outIsVar: bool;
    outIsVar = (match inConst {
        DAE::Const::C_VAR { .. } => true,
        _ => false,
    });
    outIsVar
}

pub fn propsContainReal(mut inProperties: &metamodelica::List<DAE::Properties>) -> bool {
    let mut outHasReal: bool = false;
    for mut prop in &**inProperties {
        if isReal(&(getPropType(metamodelica::AsArg::as_arg(&prop)))) {
            outHasReal = true;
            break;
        }
    }
    outHasReal
}

pub fn containReal(mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>) -> bool {
    let mut outHasReal: bool;
    for mut ty in &**inTypes {
        if isReal(metamodelica::AsArg::as_arg(&ty)) {
            outHasReal = true;
            return outHasReal;
        }
    }
    outHasReal = false;
    outHasReal
}

pub fn propAllConst(mut inProperties: DAE::Properties) -> Result<DAE::Const> {
    let mut outConst: DAE::Const;
    outConst = 'mc: {
        let __mc_input = inProperties;
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP { constFlag: mut c, .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(c.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP_TUPLE {
                tupleConst: ref constant_,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut res: DAE::Const;
            res = propTupleAllConst(constant_.clone())?;
            Ok(res)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut prop = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r#str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("- prop_all_const failed: "))?;
            r#str = printPropStr(&(prop.clone()))?;
            Debug::traceln(r#str.clone())?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outConst)
}

pub(crate) fn propAnyConst(mut inProperties: DAE::Properties) -> Result<DAE::Const> {
    let mut outConst: DAE::Const;
    outConst = 'mc: {
        let __mc_input = inProperties;
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP {
                constFlag: mut constant_,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            Ok(constant_.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP_TUPLE {
                tupleConst: ref tconstant_,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut res: DAE::Const;
            res = propTupleAnyConst(tconstant_.clone())?;
            Ok(res)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut prop = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r#str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("- prop_any_const failed: "))?;
            r#str = printPropStr(&(prop.clone()))?;
            Debug::traceln(r#str.clone())?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outConst)
}

fn propTupleAnyConst(mut inTupleConst: metamodelica::Ref<DAE::TupleConst>) -> Result<DAE::Const> {
    let mut outConst: DAE::Const;
    outConst = 'mc: {
        let __mc_input = inTupleConst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c } => {
                    Ok(c.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: _ } } => {
                    let DAE::C_CONST { .. } = (propTupleAnyConst(first.clone())?) else { return Err("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_CONST)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let DAE::C_PARAM { .. } = (propTupleAnyConst(first.clone())?) else { return Err("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_PARAM)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let DAE::C_VAR { .. } = (propTupleAnyConst(first.clone())?) else { return Err("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_VAR)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } } => {
                    let mut res: DAE::Const;
                    let DAE::C_PARAM { .. } = (propTupleAnyConst(first.clone())?) else { return Err("pattern mismatch") };
                    res = propTupleAnyConst(metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: rest.clone() }))?;
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } } => {
                    let mut res: DAE::Const;
                    let DAE::C_VAR { .. } = (propTupleAnyConst(first.clone())?) else { return Err("pattern mismatch") };
                    res = propTupleAnyConst(metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: rest.clone() }))?;
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                r#const => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- prop_tuple_any_const failed: "))?;
                    r#str = TypesDump::printTupleConstStr(metamodelica::AsArg::as_arg(&r#const))?;
                    Debug::traceln(r#str.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outConst)
}

pub fn propTupleAllConst(mut inTupleConst: metamodelica::Ref<DAE::TupleConst>) -> Result<DAE::Const> {
    let mut outConst: DAE::Const;
    outConst = 'mc: {
        let __mc_input = inTupleConst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c } => {
                    Ok(c.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: _ } } => {
                    let DAE::C_PARAM { .. } = (propTupleAllConst(first.clone())?) else { return Err("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_PARAM)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: _ } } => {
                    let DAE::C_VAR { .. } = (propTupleAllConst(first.clone())?) else { return Err("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_VAR)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let DAE::C_CONST { .. } = (propTupleAllConst(first.clone())?) else { return Err("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Const::C_CONST)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } } => {
                    let mut res: DAE::Const;
                    let DAE::C_CONST { .. } = (propTupleAllConst(first.clone())?) else { return Err("pattern mismatch") };
                    res = propTupleAllConst(metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: rest.clone() }))?;
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                r#const => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- prop_tuple_all_const failed: "))?;
                    r#str = TypesDump::printTupleConstStr(metamodelica::AsArg::as_arg(&r#const))?;
                    Debug::traceln(r#str.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outConst)
}

pub fn isPropTupleArray(mut p: &DAE::Properties) -> bool {
    let mut ob: bool;
    let mut b1: bool;
    let mut b2: bool;
    b1 = isPropTuple(p);
    b2 = isPropArray(p);
    ob = boolOr(b1, b2);
    ob
}

pub fn isPropTuple(mut p: &DAE::Properties) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = p.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(getPropType(p)) {
                Deref @ DAE::Type::T_TUPLE { .. } => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    b
}

pub(crate) fn isPropArray(mut p: &DAE::Properties) -> bool {
    let mut b: bool;
    let mut t: Type;
    t = getPropType(p);
    b = isArray(&t);
    b
}

pub fn propTupleFirstProp(mut inTupleProp: DAE::Properties) -> Result<DAE::Properties> {
    let mut outFirstProp: DAE::Properties;
    let mut ty: Type;
    let mut c: DAE::Const;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inTupleProp) {
        DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, .. }, tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: __pa1 }, tail: _ } } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    c = metamodelica::Own::own(__pa1);
    outFirstProp = DAE::Properties::PROP {
        type_: ty,
        constFlag: c,
    };
    Ok(outFirstProp)
}

pub fn propTuplePropList(mut prop_tuple: &DAE::Properties) -> Result<metamodelica::List<DAE::Properties>> {
    let mut prop_list: metamodelica::List<DAE::Properties>;
    prop_list = (::match_deref::match_deref! { match &(prop_tuple) {
        DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: tl, .. }, tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: cl } } => {
            let mut pl: metamodelica::List<DAE::Properties>;
            pl = propTuplePropList2(metamodelica::AsArg::as_arg(&tl), metamodelica::AsArg::as_arg(&cl))?;
            pl
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(prop_list)
}

fn propTuplePropList2(
    mut tl: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut cl: &metamodelica::List<metamodelica::Ref<DAE::TupleConst>>,
) -> Result<metamodelica::List<DAE::Properties>> {
    let mut pl: metamodelica::List<DAE::Properties>;
    pl = (::match_deref::match_deref! { match (tl, cl) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: t, tail: t_rest }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c }, tail: c_rest }) => {
            let mut p_rest: metamodelica::List<DAE::Properties>;
            p_rest = propTuplePropList2(t_rest, c_rest)?;
            metamodelica::cons(DAE::Properties::PROP { type_: t.clone(), constFlag: c.clone() }, p_rest)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(pl)
}

pub fn getPropConst(mut inProperties: DAE::Properties) -> Result<DAE::Const> {
    let mut outConst: DAE::Const;
    let DAE::PROP { constFlag: __pa0, .. } = (inProperties) else {
        return Err("pattern mismatch");
    };
    outConst = metamodelica::Own::own(__pa0);
    Ok(outConst)
}

pub fn getPropType(mut inProperties: &DAE::Properties) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match inProperties.clone() {
        DAE::Properties::PROP { .. } => var_field!(inProperties.type_, DAE::Properties::PROP).clone(),
        DAE::Properties::PROP_TUPLE { .. } => var_field!(inProperties.type_, DAE::Properties::PROP_TUPLE).clone(),
    });
    outType
}

pub fn setPropType(mut inProperties: &DAE::Properties, mut ty: metamodelica::Ref<DAE::Type>) -> DAE::Properties {
    let mut outProperties: DAE::Properties;
    outProperties = (match inProperties.clone() {
        DAE::Properties::PROP { .. } => DAE::Properties::PROP {
            type_: ty,
            constFlag: var_field!(inProperties.constFlag, DAE::Properties::PROP).clone(),
        },
        DAE::Properties::PROP_TUPLE { .. } => DAE::Properties::PROP_TUPLE {
            type_: ty,
            tupleConst: var_field!(inProperties.tupleConst, DAE::Properties::PROP_TUPLE).clone(),
        },
    });
    outProperties
}

pub(crate) fn createEmptyTypeMemory()
-> metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>)>> {
    let mut tyMemory: metamodelica::Array<
        metamodelica::List<(metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>)>,
    >;
    tyMemory = arrayCreate(30, metamodelica::nil());
    tyMemory
}

pub fn simplifyType(mut inType: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outExpType: metamodelica::Ref<DAE::Type>;
    outExpType = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: inType.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METARECORD { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAARRAY { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAOPTION { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATUPLE { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: inType.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN { .. } => {
                    Ok(DAE::T_UNKNOWN_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ANYTYPE { .. } => {
                    Ok(DAE::T_UNKNOWN_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                t @ Deref @ DAE::Type::T_ARRAY { .. } => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut t = (*t).clone();
                    (t, dims) = TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&t));
                    t_1 = simplifyType(t.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: dims.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => {
                    Ok(inType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                    Ok(simplifyType(t.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { .. } => {
                    Ok(DAE::T_CLOCK_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_NORETCALL { .. } => {
                    Ok(DAE::T_NORETCALL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut tys = (*tys).clone();
                    tys = List::map(tys.clone(), &simplifyType)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_TUPLE { types: tys.clone(), names: var_field!((*inType).names, DAE::Type::T_TUPLE).clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { .. } => {
                    Ok(inType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: CIS, varLst, equalityConstraint: ec, .. } => {
                            let mut varLst = (*varLst).clone();
                            let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                            varLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
                for mut v in (varLst.clone()).into_iter().cloned() {
                            let __x = simplifyVar(v.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: CIS.clone(), varLst: varLst.clone(), equalityConstraint: ec.clone(), usedExternally: var_field!((*inType).usedExternally, DAE::Type::T_COMPLEX).clone() }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: CIS @ ClassInf::State::RECORD { .. }, varLst, equalityConstraint: ec, .. } => {
                            let mut varLst = (*varLst).clone();
                            varLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
                for mut v in (varLst.clone()).into_iter().cloned() {
                            let __x = simplifyVar(v.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: CIS.clone(), varLst: varLst.clone(), equalityConstraint: ec.clone(), usedExternally: var_field!((*inType).usedExternally, DAE::Type::T_COMPLEX).clone() }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { .. } => {
                    Ok(inType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METABOXED { ty: t } => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = simplifyType(t.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(DAE::T_UNKNOWN_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.simplifyType failed for: ")); __mm_s.push_str(&*TypesDump::unparseType(inType.clone())?); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpType)
}

fn simplifyVar(mut inVar: metamodelica::Ref<DAE::Var>) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut outVar: metamodelica::Ref<DAE::Var> = inVar;
    outVar = (match &*outVar {
        DAE::Var { .. } => {
            assign_field!(outVar.ty = simplifyType(outVar.ty.clone())?);
            outVar
        }
    });
    Ok(outVar)
}

pub fn complicateType(mut inType: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type> = inType.clone();
    outType = (::match_deref::match_deref! { match &(outType.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            (ty, dims) = TypesDump::flattenArrayType(&outType);
            liftArrayListDims(ty, dims)
        },
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: __outType_functionType } => {
            __outType_functionType.clone()
        },
        Deref @ DAE::Type::T_METATYPE { ty: __outType_ty } => {
            __outType_ty.clone()
        },
        Deref @ DAE::Type::T_TUPLE { types: __outType_types, .. } => {
            assign_variant_field!(outType => DAE::Type::T_TUPLE; types = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut t in (__outType_types.clone()).into_iter().cloned() {
            let __x = complicateType(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            outType
        },
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            if isRecord(&inType) || Config::acceptMetaModelicaGrammar()? {
                assign_variant_field!(outType => DAE::Type::T_COMPLEX; varLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
        for mut v in (var_field!((*outType).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = complicateVar(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            }
            outType
        },
        Deref @ DAE::Type::T_METABOXED { ty: __outType_ty } => {
            assign_variant_field!(outType => DAE::Type::T_METABOXED; ty = complicateType(__outType_ty.clone())?);
            outType
        },
        _ => {
            outType
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

fn complicateVar(mut inVar: metamodelica::Ref<DAE::Var>) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut outVar: metamodelica::Ref<DAE::Var> = inVar;
    outVar = (match &*outVar {
        DAE::Var { .. } => {
            assign_field!(outVar.ty = complicateType(outVar.ty.clone())?);
            outVar
        }
    });
    Ok(outVar)
}

fn typeMemoryEntryEq(
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: &(metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>),
) -> bool {
    let mut outEq: bool;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    (ty2, _) = inType2.clone();
    outEq = typesElabEquivalent(inType1, ty2);
    outEq
}

pub(crate) fn typesElabEquivalent(
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
) -> bool {
    let mut isEqual: bool;
    match '__try0: {
        isEqual = unwrap_break_err!(ttypesElabEquivalent(inType1.clone(), inType2.clone()), '__try0);
        Ok::<_, &'static str>((isEqual.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isEqual = __try0_o0;
        }
        Err(_) => {
            isEqual = false;
        }
    }
    isEqual
}

fn ttypesElabEquivalent(
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match &((inType1.clone(), inType2.clone())) {
        (Deref @ DAE::Type::T_COMPLEX { complexClassType: cty1, varLst: vars1, .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: cty2, varLst: vars2, .. }) => {
            let true = (AbsynUtil::pathEqual(&(ClassInfUtil::getStateName(metamodelica::AsArg::as_arg(&cty1))), &(ClassInfUtil::getStateName(metamodelica::AsArg::as_arg(&cty2))))) else { return Err("pattern mismatch") };
            let true = (List::isEqualOnTrue(vars1.clone(), vars2.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(varsElabEquivalent(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
            true
        },
        (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: ad1, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: ad2, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
            let true = (ad1.clone() == ad2.clone()) else { return Err("pattern mismatch") };
            let true = (typesElabEquivalent(ty1.clone(), ty2.clone())) else { return Err("pattern mismatch") };
            true
        },
        (Deref @ DAE::Type::T_ENUMERATION { path: p1, names: names1, .. }, Deref @ DAE::Type::T_ENUMERATION { path: p2, names: names2, .. }) => {
            let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) else { return Err("pattern mismatch") };
            let true = (List::isEqualOnTrue(names1.clone(), names2.clone(), &fnptr!(stringEqual, ArcStr, ArcStr))?) else { return Err("pattern mismatch") };
            true
        },
        (Deref @ DAE::Type::T_TUPLE { types: types1, .. }, Deref @ DAE::Type::T_TUPLE { types: types2, .. }) => {
            List::isEqualOnTrue(types1.clone(), types2.clone(), &fnptr!(typesElabEquivalent, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>))?
        },
        (Deref @ DAE::Type::T_METABOXED { ty: ty1 }, Deref @ DAE::Type::T_METABOXED { ty: ty2 }) => {
            typesElabEquivalent(ty1.clone(), ty2.clone())
        },
        _ => {
            inType1 == inType2
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

fn varsElabEquivalent(mut inVar1: &metamodelica::Ref<DAE::Var>, mut inVar2: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match (inVar1, inVar2) {
        (Deref @ DAE::Var { name: id1, ty: ty1, .. }, Deref @ DAE::Var { name: id2, ty: ty2, .. }) if (stringEqual(&id1, &id2) && typesElabEquivalent(ty1.clone(), ty2.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEqual
}

pub fn matchProp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inActualType: &DAE::Properties,
    mut inExpectedType: &DAE::Properties,
    mut printFailtrace: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = 'mc: {
        let __mc_input = (inExp, inActualType, inExpectedType, printFailtrace);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP { type_: gt, constFlag: c1 }, DAE::Properties::PROP { type_: et, constFlag: c2 }, _) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: Type;
                    let mut c: Const;
                    (e_1, t_1) = matchType(e.clone(), gt.clone(), et.clone(), printFailtrace)?;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok((e_1.clone(), DAE::Properties::PROP { type_: t_1.clone(), constFlag: c }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP_TUPLE { type_: gt, tupleConst: tc1 }, DAE::Properties::PROP_TUPLE { type_: et, tupleConst: tc2 }, _) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: Type;
                    let mut tc: TupleConst;
                    (e_1, t_1) = matchType(e.clone(), gt.clone(), et.clone(), printFailtrace)?;
                    tc = constTupleAnd(tc1.clone(), metamodelica::AsArg::as_arg(&tc2));
                    Ok((e_1.clone(), DAE::Properties::PROP_TUPLE { type_: t_1.clone(), tupleConst: tc.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP_TUPLE { type_: gt @ Deref @ DAE::Type::T_TUPLE { .. }, tupleConst: tc1 }, DAE::Properties::PROP { type_: et @ Deref @ DAE::Type::T_METATUPLE { .. }, constFlag: c2 }, _) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: Type;
                    let mut c: Const;
                    let mut c_1: Const;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    (e_1, t_1) = matchType(e.clone(), gt.clone(), et.clone(), printFailtrace)?;
                    c_1 = propTupleAllConst(tc1.clone())?;
                    c = constAnd(c_1, c2.clone());
                    Ok((e_1.clone(), DAE::Properties::PROP { type_: t_1.clone(), constFlag: c }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP_TUPLE { type_: gt @ Deref @ DAE::Type::T_TUPLE { .. }, tupleConst: tc1 }, DAE::Properties::PROP { type_: et @ Deref @ DAE::Type::T_METABOXED { .. }, constFlag: c2 }, _) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: Type;
                    let mut c: Const;
                    let mut c_1: Const;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    (e_1, t_1) = matchType(e.clone(), gt.clone(), et.clone(), printFailtrace)?;
                    c_1 = propTupleAllConst(tc1.clone())?;
                    c = constAnd(c_1, c2.clone());
                    Ok((e_1.clone(), DAE::Properties::PROP { type_: t_1.clone(), constFlag: c }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP { type_: gt, .. }, DAE::Properties::PROP_TUPLE { .. }, _) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: Properties;
                    let mut gt = (*gt).clone();
                    prop = propTupleFirstProp(inExpectedType.clone())?;
                    (e_1, prop) = matchProp(e.clone(), inActualType, &prop, printFailtrace)?;
                    gt = simplifyType(gt.clone())?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::TSUB { exp: e_1.clone(), ix: 1, ty: gt.clone() });
                    Ok((e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP_TUPLE { .. }, DAE::Properties::PROP { .. }, _) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut gt: Type;
                    let mut prop: Properties;
                    let ref __pa1 @ DAE::PROP { type_: ref __pa0, .. } = (propTupleFirstProp(inActualType.clone())?) else { return Err("pattern mismatch") };
                    gt = metamodelica::Own::own(__pa0);
                    prop = metamodelica::Own::own(__pa1);
                    (e_1, prop) = matchProp(e.clone(), &prop, inExpectedType, printFailtrace)?;
                    gt = simplifyType(gt.clone())?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::TSUB { exp: e_1.clone(), ix: 1, ty: gt.clone() });
                    Ok((e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _, _, true) => {
                    let true = (Flags::isSet(Flags::TYPES.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.matchProp failed on exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*printPropStr(inActualType)?); __mm_s.push_str(&*literal!(" != ")); ArcStr::from(__mm_s) })?;
                    Debug::traceln(printPropStr(inExpectedType)?)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outProperties))
}

pub(crate) fn matchTypeList(
    mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut expType: metamodelica::Ref<DAE::Type>,
    mut expectedType: metamodelica::Ref<DAE::Type>,
    mut printFailtrace: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outExp: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
    let mut expLstNew: metamodelica::List<metamodelica::Ref<DAE::Exp>> = exps;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut e_1: metamodelica::Ref<DAE::Exp>;
    let mut tp: Type;
    while !((expLstNew).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expLstNew) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
        expLstNew = metamodelica::Own::own(__pa1);
        (e_1, tp) = matchType(exp, expType.clone(), expectedType.clone(), printFailtrace)?;
        outExp = metamodelica::cons(e_1, outExp);
        outTypeLst = metamodelica::cons(tp, outTypeLst);
    }
    outExp = metamodelica::Dangerous::listReverseInPlace(outExp);
    outTypeLst = metamodelica::Dangerous::listReverseInPlace(outTypeLst);
    Ok((outExp, outTypeLst))
}

pub fn matchTypeTuple(
    mut inExp1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inTypeLst2: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inTypeLst3: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut printFailtrace: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outExp, outTypeLst) = 'mc: {
        let __mc_input = (&**inExp1, &**inTypeLst2, &**inTypeLst3, printFailtrace);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, Deref @ metamodelica::ListNode::Cons { head: t1, tail: ts1 }, Deref @ metamodelica::ListNode::Cons { head: t2, tail: ts2 }, _) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tp: Type;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    (e_1, tp) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    (e_2, res) = matchTypeTuple(metamodelica::AsArg::as_arg(&rest), metamodelica::AsArg::as_arg(&ts1), metamodelica::AsArg::as_arg(&ts2), printFailtrace)?;
                    Ok((metamodelica::cons(e_1.clone(), e_2.clone()), metamodelica::cons(tp.clone(), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: t1, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: t2, tail: _ }, true) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.matchTypeTuple failed:")); __mm_s.push_str(&*TypesDump::unparseType(t1.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*TypesDump::unparseType(t2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outTypeLst))
}

pub(crate) fn matchTypeTupleCall(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inTypeLst2: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inTypeLst3: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inExp1, &**inTypeLst2, &**inTypeLst3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ metamodelica::ListNode::Cons { head: t1, tail: ts1 }, Deref @ metamodelica::ListNode::Cons { head: t2, tail: ts2 }) => {
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    matchTypeTupleCall(e.clone(), metamodelica::AsArg::as_arg(&ts1), metamodelica::AsArg::as_arg(&ts2))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- matchTypeTupleCall failed\n"))?;
                    Ok(return Err("fail"))
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

pub fn vectorizableType(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inExpType: metamodelica::Ref<DAE::Type>,
    mut inExpectedType: metamodelica::Ref<DAE::Type>,
    mut fnPath: Option<metamodelica::Ref<Absyn::Path>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outArrayDimLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut outBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    (outExp, outType, outArrayDimLst, outBindings) = vectorizableType2(
        inExp,
        inExpType.clone(),
        &(inExpType),
        metamodelica::nil(),
        inExpectedType,
        fnPath,
    )?;
    Ok((outExp, outType, outArrayDimLst, outBindings))
}

fn vectorizableType2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inExpType: metamodelica::Ref<DAE::Type>,
    mut inCurrentType: &metamodelica::Ref<DAE::Type>,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inExpectedType: metamodelica::Ref<DAE::Type>,
    mut fnPath: Option<metamodelica::Ref<Absyn::Path>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut outBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    let mut vec_type: Type;
    let mut cur_type: Type;
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    match '__try0: {
        vec_type = liftArrayListDimsReverse(inExpectedType.clone(), &inDims);
        (outExp, outType, outBindings) = unwrap_break_err!(matchTypePolymorphic(inExp.clone(), inExpType.clone(), vec_type.clone(), fnPath.clone(), metamodelica::nil(), true), '__try0);
        outDims = inDims.clone().reverse();
        Ok::<_, &'static str>((outBindings.clone(), outDims.clone(), outExp.clone(), outType.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            outBindings = __try0_o0;
            outDims = __try0_o1;
            outExp = __try0_o2;
            outType = __try0_o3;
        }
        Err(_) => {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &((*inCurrentType)) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa1, dims: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cur_type = metamodelica::Own::own(__pa1);
            dim = metamodelica::Own::own(__pa2);
            (outExp, outType, outDims, outBindings) = vectorizableType2(
                inExp.clone(),
                inExpType.clone(),
                &cur_type,
                metamodelica::cons(dim.clone(), inDims.clone()),
                inExpectedType.clone(),
                fnPath.clone(),
            )?;
        }
    }
    Ok((outExp, outType, outDims, outBindings))
}

pub fn unflattenArrayType(mut inTy: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outTy: metamodelica::Ref<DAE::Type>;
    outTy = unflattenArrayType2(inTy, false)?;
    Ok(outTy)
}

fn unflattenArrayType2(mut inTy: metamodelica::Ref<DAE::Type>, mut last: bool) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outTy: metamodelica::Ref<DAE::Type>;
    outTy = 'mc: {
        let __mc_input = (inTy, last);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci, varLst: vl, complexType: ty, equalityConstraint: eqc }, _) => {
                    let mut ty = (*ty).clone();
                    ty = unflattenArrayType(ty.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci.clone(), varLst: vl.clone(), complexType: ty.clone(), equalityConstraint: eqc.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
                    let mut t = (*t).clone();
                    t = unflattenArrayType(t.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::ListNode::Nil }, true) => {
                    Ok(unflattenArrayType(t.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims } }, _) => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = unflattenArrayType2(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: dims.clone() }), true)?;
                    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] });
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, false) => {
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTy)
}

fn typeConvert(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut actual: metamodelica::Ref<DAE::Type>,
    mut expected: metamodelica::Ref<DAE::Type>,
    mut printFailtrace: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outExp, outType) = 'mc: {
        let __mc_input = (inExp1, actual.clone(), expected.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, ty1, ty2) => {
                    let true = (subtype(ty1.clone(), ty2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok((e.clone(), ty2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: ty1, tail: _ }, .. }, ty2) => {
                    let mut ty: Type;
                    let mut e = (*e).clone();
                    let false = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let false = (isTuple(metamodelica::AsArg::as_arg(&ty2))) else { return Err("pattern mismatch") };
                    let true = (subtype(ty1.clone(), ty2.clone(), true)) else { return Err("pattern mismatch") };
                    e = metamodelica::Ref::new(DAE::Exp::TSUB { exp: e.clone(), ix: 1, ty: ty2.clone() });
                    ty = ty2.clone();
                    Ok((e.clone(), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. }, ty2) => {
                    let mut ty1: Type;
                    let mut ty: Type;
                    let mut e = (*e).clone();
                    let mut ty2 = (*ty2).clone();
                    ty1 = unflattenArrayType(actual.clone())?;
                    ty2 = unflattenArrayType(ty2.clone())?;
                    (e, ty) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    Ok((e.clone(), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, ty1, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. }) => {
                    let mut ty2: Type;
                    let mut ty: Type;
                    let mut e = (*e).clone();
                    let mut ty1 = (*ty1).clone();
                    ty1 = unflattenArrayType(ty1.clone())?;
                    ty2 = unflattenArrayType(expected.clone())?;
                    (e, ty) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    Ok((e.clone(), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: elist, .. }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, ty0 @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut sc: bool;
                    let mut a: bool;
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    elist_1 = typeConvertArray(metamodelica::AsArg::as_arg(&elist), ty1.clone(), ty2.clone(), printFailtrace)?;
                    at = simplifyType(ty0.clone())?;
                    a = isArray(metamodelica::AsArg::as_arg(&ty2));
                    sc = boolNot(a);
                    Ok((metamodelica::Ref::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: sc, array: elist_1.clone() }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: elist, .. }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut sc: bool;
                    let mut a: bool;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut ety1: metamodelica::Ref<DAE::Type>;
                    let mut ty2 = (*ty2).clone();
                    let true = (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim1))) else { return Err("pattern mismatch") };
                    elist_1 = typeConvertArray(metamodelica::AsArg::as_arg(&elist), ty1.clone(), ty2.clone(), printFailtrace)?;
                    dims = Expression::arrayDimension(&(simplifyType(ty1.clone())?));
                    a = isArray(metamodelica::AsArg::as_arg(&ty2));
                    sc = boolNot(a);
                    dims = metamodelica::cons(dim1.clone(), dims.clone());
                    ty2 = arrayElementType(metamodelica::AsArg::as_arg(&ty2));
                    ety1 = simplifyType(ty2.clone())?;
                    ty2 = liftArrayListDims(ty2.clone(), dims.clone());
                    Ok((metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety1.clone(), dims: dims.clone() }), scalar: sc, array: elist_1.clone() }), ty2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { start: begin, step: Some(step), stop, .. }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut begin_1: metamodelica::Ref<DAE::Exp>;
                    let mut step_1: metamodelica::Ref<DAE::Exp>;
                    let mut stop_1: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    (begin_1, _) = typeConvert(begin.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    (step_1, _) = typeConvert(step.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    (stop_1, _) = typeConvert(stop.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    at = simplifyType(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] }))?;
                    Ok((metamodelica::Ref::new(DAE::Exp::RANGE { ty: at.clone(), start: begin_1.clone(), step: Some(step_1.clone()), stop: stop_1.clone() }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { start: begin, step: None, stop, .. }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut begin_1: metamodelica::Ref<DAE::Exp>;
                    let mut stop_1: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    (begin_1, _) = typeConvert(begin.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    (stop_1, _) = typeConvert(stop.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    at = simplifyType(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] }))?;
                    Ok((metamodelica::Ref::new(DAE::Exp::RANGE { ty: at.clone(), start: begin_1.clone(), step: None, stop: stop_1.clone() }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty2.clone(), dims: list![dim1.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { integer: nmax, matrix: ell, .. }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim11, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 } }, ty0 @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim22, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 } }) => {
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut ell_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim11), metamodelica::AsArg::as_arg(&dim22))?) else { return Err("pattern mismatch") };
                    ell_1 = typeConvertMatrix(ell.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    at = simplifyType(ty0.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::MATRIX { ty: at.clone(), integer: nmax.clone(), matrix: ell_1.clone() }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t2.clone(), dims: list![dim11.clone()] }), dims: list![dim1.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { integer: nmax, matrix: ell, .. }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim11, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 } }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim22, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 } }) => {
                    if !((!(Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim2))))) { return Err("guard") }
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut ty: Type;
                    let mut ell_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim11), metamodelica::AsArg::as_arg(&dim22))?) else { return Err("pattern mismatch") };
                    ell_1 = typeConvertMatrix(ell.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t2.clone(), dims: list![dim11.clone()] }), dims: list![dim1.clone()] });
                    at = simplifyType(ty.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::MATRIX { ty: at.clone(), integer: nmax.clone(), matrix: ell_1.clone() }), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
                    let mut t_1: Type;
                    let mut t_2: Type;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    (e_1, t_1) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    e_1 = liftExpType(e_1.clone(), dim1.clone());
                    t_2 = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim2.clone()] });
                    Ok((e_1.clone(), t_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
                    let mut t_1: Type;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    (e_1, t_1) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    e_1 = liftExpType(e_1.clone(), openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN());
                    Ok((e_1.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
                    let mut t_1: Type;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    (e_1, t_1) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    e_1 = liftExpType(e_1.clone(), dim1.clone());
                    Ok((e_1.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim1.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty1 }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }) => {
                    let mut t_1: Type;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let false = (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim1))) else { return Err("pattern mismatch") };
                    let false = (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim2))) else { return Err("pattern mismatch") };
                    (e_1, t_1) = typeConvert(e.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
                    e_1 = liftExpType(e_1.clone(), openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN());
                    Ok((e_1.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: elist }, Deref @ DAE::Type::T_TUPLE { types: tys1, .. }, Deref @ DAE::Type::T_TUPLE { types: tys2, .. }) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tys_1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    (elist_1, tys_1) = typeConvertList(metamodelica::AsArg::as_arg(&elist), metamodelica::AsArg::as_arg(&tys1), metamodelica::AsArg::as_arg(&tys2), printFailtrace)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::TUPLE { PR: elist_1.clone() }), metamodelica::Ref::new(DAE::Type::T_TUPLE { types: tys_1.clone(), names: var_field!((*expected).names, DAE::Type::T_TUPLE).clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::ICONST { integer: oi }, Deref @ DAE::Type::T_INTEGER { .. }, t2 @ Deref @ DAE::Type::T_ENUMERATION { path: tp, names: l, .. }) => {
                    let mut name: ArcStr;
                    let mut tp = (*tp).clone();
                    let true = (Config::intEnumConversion()?) else { return Err("pattern mismatch") };
                    let true = (typeConvertIntToEnumCheck(metamodelica::AsArg::as_arg(&exp), metamodelica::AsArg::as_arg(&t2))?) else { return Err("pattern mismatch") };
                    name = (l).get(oi.clone())?;
                    tp = AbsynUtil::joinPaths(tp.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }))?;
                    Ok((metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: tp.clone(), index: oi.clone() }), expected.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
                    Ok((metamodelica::Ref::new(DAE::Exp::CAST { ty: DAE::T_REAL_DEFAULT().clone(), exp: e.clone() }), expected.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t1, .. }, t2) => {
                    let mut t_1: Type;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    (e_1, t_1) = typeConvert(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    Ok((e_1.clone(), t_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t2, .. }) => {
                    let mut t_1: Type;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    (e_1, t_1) = typeConvert(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    Ok((e_1.clone(), t_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p1 }, varLst: els1, .. }, t2 @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, varLst: els2, .. }) => {
                    let mut e = (*e).clone();
                    let false = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) else { return Err("pattern mismatch") };
                    let true = (Flags::isSet(Flags::ALLOW_RECORD_TOO_MANY_FIELDS.clone())? || ((els1).len() as i32) == ((els2).len() as i32)) else { return Err("pattern mismatch") };
                    let true = (subtypeVarlist(els1.clone(), metamodelica::AsArg::as_arg(&els2))) else { return Err("pattern mismatch") };
                    e = metamodelica::Ref::new(DAE::Exp::CAST { ty: t2.clone(), exp: e.clone() });
                    Ok((e.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_OPTION { exp: Some(e) }, Deref @ DAE::Type::T_METAOPTION { ty: t1 }, Deref @ DAE::Type::T_METAOPTION { ty: t2 }) => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { return Err("guard") }
                    let mut t_1: Type;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    (e_1, t_1) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: Some(e_1.clone()) }), metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: t_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_OPTION { exp: None }, _, Deref @ DAE::Type::T_METAOPTION { ty: t2 }) => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { return Err("guard") }
                    Ok((metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: None }), metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: t2.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: elist }, Deref @ DAE::Type::T_TUPLE { types: tys1, .. }, Deref @ DAE::Type::T_METATUPLE { types: tys2 }) => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { return Err("guard") }
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tys_1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tys2 = (*tys2).clone();
                    tys2 = List::map(tys2.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    (elist_1, tys_1) = matchTypeTuple(metamodelica::AsArg::as_arg(&elist), metamodelica::AsArg::as_arg(&tys1), metamodelica::AsArg::as_arg(&tys2), printFailtrace)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::META_TUPLE { listExp: elist_1.clone() }), metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATCHEXPRESSION { matchType: matchTy, inputs, aliases, localDecls, cases, et }, _, _) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut elist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cases = (*cases).clone();
                    let mut et = (*et).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    elist = resultExps(metamodelica::AsArg::as_arg(&cases));
                    (elist_1, _) = matchTypeList(elist.clone(), actual.clone(), expected.clone(), printFailtrace)?;
                    cases = fixCaseReturnTypes2(metamodelica::AsArg::as_arg(&cases), elist_1.clone(), Absyn::dummyInfo.clone())?;
                    et = simplifyType(expected.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::MATCHEXPRESSION { matchType: matchTy.clone(), inputs: inputs.clone(), aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases.clone(), et: et.clone() }), expected.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_TUPLE { listExp: elist }, Deref @ DAE::Type::T_METATUPLE { types: tys1 }, Deref @ DAE::Type::T_METATUPLE { types: tys2 }) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tys_1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tys2 = (*tys2).clone();
                    tys2 = List::map(tys2.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    (elist_1, tys_1) = matchTypeTuple(metamodelica::AsArg::as_arg(&elist), metamodelica::AsArg::as_arg(&tys1), metamodelica::AsArg::as_arg(&tys2), printFailtrace)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::META_TUPLE { listExp: elist_1.clone() }), metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: elist }, Deref @ DAE::Type::T_TUPLE { types: tys1, .. }, ty2 @ Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_UNKNOWN { .. } }) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tys_1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    tys2 = List::fill(ty2.clone(), ((tys1).len() as i32));
                    (elist_1, tys_1) = matchTypeTuple(metamodelica::AsArg::as_arg(&elist), metamodelica::AsArg::as_arg(&tys1), &tys2, printFailtrace)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::META_TUPLE { listExp: elist_1.clone() }), metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, scalar: _, array: elist }, Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, Deref @ DAE::Type::T_METALIST { ty: t2 }) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut t2 = (*t2).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    t2 = boxIfUnboxedType(t2.clone());
                    (elist_1, _) = matchTypeList(elist.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::LIST { valList: elist_1.clone() });
                    t2 = metamodelica::Ref::new(DAE::Type::T_METALIST { ty: t2.clone() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, scalar: _, array: elist }, Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut t2 = (*t2).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    (elist_1, tys1) = matchTypeList(elist.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    (elist_1, t2) = listMatchSuperType(&elist_1, &tys1, printFailtrace)?;
                    t2 = boxIfUnboxedType(t2.clone());
                    (elist_1, _) = matchTypeList(elist_1.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::LIST { valList: elist_1.clone() });
                    t2 = metamodelica::Ref::new(DAE::Type::T_METALIST { ty: t2.clone() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { .. }, integer: _, matrix: elist_big }, t1, t2) => {
                    let mut elist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ty2: Type;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    (elist, ty2) = typeConvertMatrixToList(metamodelica::AsArg::as_arg(&elist_big), metamodelica::AsArg::as_arg(&t1), metamodelica::AsArg::as_arg(&t2), printFailtrace)?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::LIST { valList: elist.clone() });
                    Ok((e_1.clone(), ty2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LIST { valList: elist }, Deref @ DAE::Type::T_METALIST { ty: t1 }, Deref @ DAE::Type::T_METALIST { ty: t2 }) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut t2 = (*t2).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    (elist_1, tys1) = matchTypeList(elist.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    (elist_1, t2) = listMatchSuperType(&elist_1, &tys1, printFailtrace)?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::LIST { valList: elist_1.clone() });
                    t2 = metamodelica::Ref::new(DAE::Type::T_METALIST { ty: t2.clone() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace)?;
                    t2 = metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_BOOL { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace)?;
                    t2 = metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace)?;
                    t2 = metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_ENUMERATION { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace)?;
                    t2 = metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, t1 @ Deref @ DAE::Type::T_ARRAY { .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut e = (*e).clone();
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    (e, t1) = matchType(e.clone(), t1.clone(), unboxedType(t2.clone())?, printFailtrace)?;
                    t2 = metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    e = Expression::boxExp(e.clone());
                    Ok((e.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: path1, expLst: elist, .. }, t1 @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path2 }, varLst: v, .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut l: metamodelica::List<ArcStr>;
                    let mut elist = (*elist).clone();
                    let mut t2 = (*t2).clone();
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2))) else { return Err("pattern mismatch") };
                    t2 = metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    l = List::map(v.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?;
                    tys1 = List::map(v.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| getVarType(&__a0))?;
                    tys2 = List::map(tys1.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    (elist, _) = matchTypeTuple(metamodelica::AsArg::as_arg(&elist), &tys1, &tys2, printFailtrace)?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: path1.clone(), args: elist.clone(), fieldNames: l.clone(), index: -1, typeVars: metamodelica::nil() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { path: path1, exps: elist, .. }, t1 @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path2 }, varLst: v, .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut l: metamodelica::List<ArcStr>;
                    let mut elist = (*elist).clone();
                    let mut t2 = (*t2).clone();
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2))) else { return Err("pattern mismatch") };
                    t2 = metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    l = List::map(v.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?;
                    tys1 = List::map(v.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| getVarType(&__a0))?;
                    tys2 = List::map(tys1.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    (elist, _) = matchTypeTuple(metamodelica::AsArg::as_arg(&elist), &tys1, &tys2, printFailtrace)?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: path1.clone(), args: elist.clone(), fieldNames: l.clone(), index: -1, typeVars: metamodelica::nil() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, ty: _ }, t1 @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: v, .. }, Deref @ DAE::Type::T_METABOXED { ty: t2 }) => {
                    let mut elist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut l: metamodelica::List<ArcStr>;
                    let mut pathList: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut crefList: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut t2 = (*t2).clone();
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    t2 = metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: t1.clone() });
                    l = List::map(v.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?;
                    tys1 = List::map(v.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| getVarType(&__a0))?;
                    tys2 = List::map(tys1.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    expTypes = List::map(tys1.clone(), &simplifyType)?;
                    pathList = List::map(l.clone(), &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr))?;
                    crefList = List::map(pathList.clone(), &move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(ComponentReference::pathToCref(&__a0)) })?;
                    crefList = List::map1r(crefList.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::joinCrefs(&__a0, __a1), cref.clone())?;
                    elist = List::threadMap(crefList.clone(), expTypes.clone(), &Expression::makeCrefExp)?;
                    (elist, _) = matchTypeTuple(&elist, &tys1, &tys2, printFailtrace)?;
                    e_1 = metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: path.clone(), args: elist.clone(), fieldNames: l.clone(), index: -1, typeVars: metamodelica::nil() });
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, Deref @ DAE::Type::T_METABOXED { .. }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Not yet implemented: Converting record into boxed records: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BOX { exp: e }, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut t2 = (*t2).clone();
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    (e_1, t2) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    Ok((e_1.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_INTEGER { .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    t = simplifyType(t2.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::UNBOX { exp: e.clone(), ty: t.clone() }), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_REAL { .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    t = simplifyType(t2.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::UNBOX { exp: e.clone(), ty: t.clone() }), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_BOOL { .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    t = simplifyType(t2.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::UNBOX { exp: e.clone(), ty: t.clone() }), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_ENUMERATION { .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    t = simplifyType(t2.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::UNBOX { exp: e.clone(), ty: t.clone() }), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ DAE::Type::T_METABOXED { ty: t1 }, t2 @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    (e_1, _) = matchType(e.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    t = simplifyType(t2.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("mmc_unbox_record") }), expLst: list![e_1.clone()], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: t.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outType))
}

fn liftExpType(
    mut ie: metamodelica::Ref<DAE::Exp>,
    mut dim: metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    res = (::match_deref::match_deref! { match &(ie) {
        Deref @ DAE::Exp::CAST { ty, exp: e } => {
            let mut ty1: metamodelica::Ref<DAE::Type>;
            ty1 = Expression::liftArrayR(ty.clone(), dim);
            metamodelica::Ref::new(DAE::Exp::CAST { ty: ty1, exp: e.clone() })
        },
        e => {
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn typeConvertArray(
    mut inArray: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inActualType: metamodelica::Ref<DAE::Type>,
    mut inExpectedType: metamodelica::Ref<DAE::Type>,
    mut inPrintFailtrace: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outArray: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outArray = (::match_deref::match_deref! { match inArray {
        Deref @ metamodelica::ListNode::Nil => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = makeDummyExpFromType(&inActualType)?;
            typeConvert(e, inActualType, inExpectedType, inPrintFailtrace)?;
            metamodelica::nil()
        },
        _ => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl, _) = List::map_2(inArray, &({ let __pe_b1 = inActualType; let __pe_b2 = inExpectedType; let __pe_b3 = inPrintFailtrace; move |__pe_a0| typeConvert(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            expl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArray)
}

fn typeConvertMatrix(
    mut inMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inActualType: metamodelica::Ref<DAE::Type>,
    mut inExpectedType: metamodelica::Ref<DAE::Type>,
    mut printFailtrace: bool,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut outMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    outMatrix = List::map3(
        inMatrix,
        &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
               __a1: metamodelica::Ref<DAE::Type>,
               __a2: metamodelica::Ref<DAE::Type>,
               __a3: bool| typeConvertArray(&__a0, __a1, __a2, __a3),
        inActualType,
        inExpectedType,
        printFailtrace,
    )?;
    Ok(outMatrix)
}

fn typeConvertList(
    mut inExpExpLst1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inTypeLst2: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inTypeLst3: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut printFailtrace: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outExpExpLst, outTypeLst) = (::match_deref::match_deref! { match (inExpExpLst1, inTypeLst2, inTypeLst3) {
        (Deref @ metamodelica::ListNode::Nil, _, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }, Deref @ metamodelica::ListNode::Cons { head: ty1, tail: ty1rest }, Deref @ metamodelica::ListNode::Cons { head: ty2, tail: ty2rest }) => {
            let mut rest_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut tyrest_1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut first_1: metamodelica::Ref<DAE::Exp>;
            let mut ty_1: Type;
            (rest_1, tyrest_1) = typeConvertList(rest, ty1rest, ty2rest, printFailtrace)?;
            (first_1, ty_1) = typeConvert(first.clone(), ty1.clone(), ty2.clone(), printFailtrace)?;
            (metamodelica::cons(first_1, rest_1), metamodelica::cons(ty_1, tyrest_1))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outExpExpLst, outTypeLst))
}

fn typeConvertMatrixToList(
    mut melist: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut outType: &metamodelica::Ref<DAE::Type>,
    mut printFailtrace: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut actualOutType: metamodelica::Ref<DAE::Type>;
    (outExp, actualOutType) = 'mc: {
        let __mc_input = (&**melist, &**inType, &**outType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok((metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: expl, tail: rest }, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: t1, .. }, .. }, Deref @ DAE::Type::T_METALIST { ty: Deref @ DAE::Type::T_METALIST { ty: t2 } }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut expl = (*expl).clone();
                    let mut t1 = (*t1).clone();
                    (e, t1) = typeConvertMatrixRowToList(expl.clone(), t1.clone(), t2.clone(), printFailtrace)?;
                    (expl, _) = typeConvertMatrixToList(metamodelica::AsArg::as_arg(&rest), inType, outType, printFailtrace)?;
                    Ok((metamodelica::cons(e.clone(), expl.clone()), metamodelica::Ref::new(DAE::Type::T_METALIST { ty: t1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::TYPES.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- typeConvertMatrixToList failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, actualOutType))
}

fn typeConvertMatrixRowToList(
    mut elist: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut outType: metamodelica::Ref<DAE::Type>,
    mut printFailtrace: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut out: metamodelica::Ref<DAE::Exp>;
    let mut t1: metamodelica::Ref<DAE::Type>;
    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matchTypeList(elist, inType, outType, printFailtrace)?) {
        (__pa0, Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    elist_1 = metamodelica::Own::own(__pa0);
    t1 = metamodelica::Own::own(__pa1);
    out = metamodelica::Ref::new(DAE::Exp::LIST { valList: elist_1 });
    t1 = metamodelica::Ref::new(DAE::Type::T_METALIST { ty: t1 });
    Ok((out, t1))
}

pub fn matchWithPromote(
    mut inProperties1: &DAE::Properties,
    mut inProperties2: &DAE::Properties,
    mut inBoolean3: bool,
) -> Result<DAE::Properties> {
    let mut outProperties: DAE::Properties;
    outProperties = 'mc: {
        let __mc_input = (inProperties1, inProperties2, inBoolean3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t1, .. }, constFlag: c1 }, DAE::Properties::PROP { type_: t2, constFlag: c2 }, havereal) => {
                    Ok(matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t1, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t2, .. }, constFlag: c2 }, havereal) => {
                    Ok(matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }, constFlag: c2 }, havereal) => {
                    let mut t: Type;
                    let mut c: Const;
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?) else { return Err("pattern mismatch") };
                    t = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    dim = dim1.clone();
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t1, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 1 }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }, constFlag: c2 }, havereal) => {
                    let mut t: Type;
                    let mut c: Const;
                    let false = (isArray(metamodelica::AsArg::as_arg(&t1))) else { return Err("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?) else { return Err("pattern mismatch") };
                    t = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t1, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim @ Deref @ DAE::Dimension::DIM_ENUM { size: 1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }, constFlag: c2 }, havereal) => {
                    let mut t: Type;
                    let mut c: Const;
                    let false = (isArray(metamodelica::AsArg::as_arg(&t1))) else { return Err("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?) else { return Err("pattern mismatch") };
                    t = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t1, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim @ Deref @ DAE::Dimension::DIM_BOOLEAN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }, constFlag: c2 }, havereal) => {
                    let mut t: Type;
                    let mut c: Const;
                    let false = (isArray(metamodelica::AsArg::as_arg(&t1))) else { return Err("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?) else { return Err("pattern mismatch") };
                    t = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 1 }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, constFlag: c1 }, DAE::Properties::PROP { type_: t2, constFlag: c2 }, havereal) => {
                    let mut t: Type;
                    let mut c: Const;
                    let false = (isArray(metamodelica::AsArg::as_arg(&t2))) else { return Err("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?) else { return Err("pattern mismatch") };
                    t = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim @ Deref @ DAE::Dimension::DIM_ENUM { size: 1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, constFlag: c1 }, DAE::Properties::PROP { type_: t2, constFlag: c2 }, havereal) => {
                    let mut t: Type;
                    let mut c: Const;
                    let false = (isArray(metamodelica::AsArg::as_arg(&t2))) else { return Err("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?) else { return Err("pattern mismatch") };
                    t = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim @ Deref @ DAE::Dimension::DIM_BOOLEAN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, constFlag: c1 }, DAE::Properties::PROP { type_: t2, constFlag: c2 }, havereal) => {
                    let mut t: Type;
                    let mut c: Const;
                    let false = (isArray(metamodelica::AsArg::as_arg(&t2))) else { return Err("pattern mismatch") };
                    let DAE::PROP { type_: __pa0, constFlag: __pa1 } = (matchWithPromote(&(DAE::Properties::PROP { type_: t1.clone(), constFlag: c1.clone() }), &(DAE::Properties::PROP { type_: t2.clone(), constFlag: c2.clone() }), havereal.clone())?) else { return Err("pattern mismatch") };
                    t = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![dim.clone()] }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t1, constFlag: c1 }, DAE::Properties::PROP { type_: t2, constFlag: c2 }, false) => {
                    let mut c: Const;
                    let false = (isArray(metamodelica::AsArg::as_arg(&t1))) else { return Err("pattern mismatch") };
                    let false = (isArray(metamodelica::AsArg::as_arg(&t2))) else { return Err("pattern mismatch") };
                    let true = (equivtypes(t1.clone(), t2.clone())) else { return Err("pattern mismatch") };
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: t1.clone(), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: t @ Deref @ DAE::Type::T_ENUMERATION { .. }, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ENUMERATION { .. }, constFlag: c2 }, false) => {
                    let mut c: Const;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: t.clone(), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_REAL { varLst: v }, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_REAL { .. }, constFlag: c2 }, true) => {
                    let mut c: Const;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_REAL { varLst: v.clone() }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_REAL { varLst: v }, constFlag: c2 }, true) => {
                    let mut c: Const;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_REAL { varLst: v.clone() }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_REAL { varLst: v }, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: c2 }, true) => {
                    let mut c: Const;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_REAL { varLst: v.clone() }), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: c1 }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: c2 }, true) => {
                    let mut c: Const;
                    c = constAnd(c1.clone(), c2.clone());
                    Ok(DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: c })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.matchWithPromote failed on: ")); __mm_s.push_str(&*literal!("\nprop1: ")); __mm_s.push_str(&*printPropStr(inProperties1)?); __mm_s.push_str(&*literal!("\nprop2: ")); __mm_s.push_str(&*printPropStr(inProperties2)?); __mm_s.push_str(&*literal!("\nhaveReal: ")); __mm_s.push_str(&*boolString(inBoolean3)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outProperties)
}

pub fn constAnd(mut inConst1: DAE::Const, mut inConst2: DAE::Const) -> DAE::Const {
    let mut outConst: DAE::Const;
    outConst = (match (inConst1, inConst2) {
        (DAE::Const::C_CONST { .. }, DAE::Const::C_CONST { .. }) => openmodelica_frontend_types::DAE::Const::C_CONST,
        (DAE::Const::C_CONST { .. }, DAE::Const::C_PARAM { .. }) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (DAE::Const::C_PARAM { .. }, DAE::Const::C_CONST { .. }) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (DAE::Const::C_PARAM { .. }, DAE::Const::C_PARAM { .. }) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (DAE::Const::C_UNKNOWN { .. }, _) => openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
        (_, DAE::Const::C_UNKNOWN { .. }) => openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
        _ => openmodelica_frontend_types::DAE::Const::C_VAR,
    });
    outConst
}

fn constTupleAnd(
    mut inTupleConst1: metamodelica::Ref<DAE::TupleConst>,
    mut inTupleConst2: &metamodelica::Ref<DAE::TupleConst>,
) -> metamodelica::Ref<DAE::TupleConst> {
    let mut outTupleConst: metamodelica::Ref<DAE::TupleConst>;
    outTupleConst = (::match_deref::match_deref! { match &(inTupleConst1) {
        c1 => {
            c1.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTupleConst
}

pub(crate) fn constOr(mut inConst1: DAE::Const, mut inConst2: DAE::Const) -> DAE::Const {
    let mut outConst: DAE::Const;
    outConst = (match (inConst1, inConst2) {
        (DAE::Const::C_CONST { .. }, _) => openmodelica_frontend_types::DAE::Const::C_CONST,
        (_, DAE::Const::C_CONST { .. }) => openmodelica_frontend_types::DAE::Const::C_CONST,
        (DAE::Const::C_PARAM { .. }, _) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (_, DAE::Const::C_PARAM { .. }) => openmodelica_frontend_types::DAE::Const::C_PARAM,
        (DAE::Const::C_UNKNOWN { .. }, _) => openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
        (_, DAE::Const::C_UNKNOWN { .. }) => openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
        _ => openmodelica_frontend_types::DAE::Const::C_VAR,
    });
    outConst
}

pub(crate) fn boolConst(mut inBoolean: bool) -> DAE::Const {
    let mut outConst: DAE::Const;
    outConst = (match inBoolean {
        false => openmodelica_frontend_types::DAE::Const::C_VAR,
        true => openmodelica_frontend_types::DAE::Const::C_CONST,
    });
    outConst
}

pub fn boolConstSize(mut inBoolean: bool) -> DAE::Const {
    let mut outConst: DAE::Const;
    outConst = (match inBoolean {
        false => openmodelica_frontend_types::DAE::Const::C_PARAM,
        true => openmodelica_frontend_types::DAE::Const::C_CONST,
    });
    outConst
}

pub fn constEqualOrHigher(mut c1: DAE::Const, mut c2: DAE::Const) -> bool {
    let mut b: bool;
    b = (match (c1, c2) {
        (DAE::Const::C_CONST { .. }, _) => true,
        (_, DAE::Const::C_CONST { .. }) => false,
        (DAE::Const::C_PARAM { .. }, _) => true,
        (_, DAE::Const::C_PARAM { .. }) => false,
        _ => true,
    });
    b
}

pub(crate) fn constEqual(mut c1: DAE::Const, mut c2: DAE::Const) -> bool {
    let mut b: bool;
    b = c1 == c2;
    b
}

pub fn constIsVariable(mut c: DAE::Const) -> bool {
    let mut b: bool;
    b = constEqual(c, openmodelica_frontend_types::DAE::Const::C_VAR);
    b
}

pub(crate) fn constIsParameter(mut c: DAE::Const) -> bool {
    let mut b: bool;
    b = constEqual(c, openmodelica_frontend_types::DAE::Const::C_PARAM);
    b
}

pub fn constIsConst(mut c: DAE::Const) -> bool {
    let mut b: bool;
    b = constEqual(c, openmodelica_frontend_types::DAE::Const::C_CONST);
    b
}

pub fn printPropStr(mut inProperties: &DAE::Properties) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inProperties.clone() {
        DAE::Properties::PROP {
            type_: ref ty,
            constFlag: mut r#const,
        } => {
            let mut ty_str: ArcStr;
            let mut const_str: ArcStr;
            let mut res: ArcStr;
            ty_str = TypesDump::unparseType(ty.clone())?;
            const_str = TypesDump::printConstStr(r#const.clone())?;
            res = stringAppendList(list![
                literal!("DAE.PROP("),
                ty_str,
                literal!(", "),
                const_str,
                literal!(")")
            ]);
            res
        }
        DAE::Properties::PROP_TUPLE {
            type_: ref ty,
            tupleConst: ref tconst,
        } => {
            let mut ty_str: ArcStr;
            let mut const_str: ArcStr;
            let mut res: ArcStr;
            ty_str = TypesDump::unparseType(ty.clone())?;
            const_str = TypesDump::printTupleConstStr(metamodelica::AsArg::as_arg(&tconst))?;
            res = stringAppendList(list![
                literal!("DAE.PROP_TUPLE("),
                ty_str,
                literal!(", "),
                const_str,
                literal!(")")
            ]);
            res
        }
    });
    Ok(outString)
}

pub(crate) fn printProp(mut p: &DAE::Properties) -> Result<()> {
    let mut r#str: ArcStr;
    r#str = printPropStr(p)?;
    Print::printErrorBuf(r#str)?;
    Ok(())
}

pub(crate) fn flowVariables(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outExpComponentRefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outExpComponentRefLst = 'mc: {
        let __mc_input = (&**inVarLst, inComponentRef);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: id, attributes: Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::FLOW { .. }, .. }, ty, .. }, tail: vs }, cr) => {
                    let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    ty2 = simplifyType(ty.clone())?;
                    cr_1 = ComponentReference::crefPrependIdent(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&id), &(metamodelica::nil()), &ty2)?;
                    res = flowVariables(metamodelica::AsArg::as_arg(&vs), cr.clone())?;
                    Ok(metamodelica::cons(cr_1.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: vs }, cr) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    res = flowVariables(metamodelica::AsArg::as_arg(&vs), cr.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpComponentRefLst)
}

pub(crate) fn streamVariables(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outExpComponentRefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outExpComponentRefLst = 'mc: {
        let __mc_input = (&**inVarLst, inComponentRef);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: id, attributes: Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. }, ty, .. }, tail: vs }, cr) => {
                    let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    ty2 = simplifyType(ty.clone())?;
                    cr_1 = ComponentReference::crefPrependIdent(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&id), &(metamodelica::nil()), &ty2)?;
                    res = streamVariables(metamodelica::AsArg::as_arg(&vs), cr.clone())?;
                    Ok(metamodelica::cons(cr_1.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: vs }, cr) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    res = streamVariables(metamodelica::AsArg::as_arg(&vs), cr.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpComponentRefLst)
}

pub(crate) fn getAllExps(
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpExpLst = getAllExpsTt(inType)?;
    Ok(outExpExpLst)
}

fn getAllExpsTt(mut inType: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpExpLst = 'mc: {
        let __mc_input = inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: vars } => {
                    Ok(getAllExpsVars(vars.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: vars } => {
                    Ok(getAllExpsVars(vars.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { varLst: vars } => {
                    Ok(getAllExpsVars(vars.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: vars } => {
                    Ok(getAllExpsVars(vars.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { literalVarLst: vars, attributeLst: attrs, .. } => {
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tyexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    exps = getAllExpsVars(vars.clone())?;
                    tyexps = getAllExpsVars(attrs.clone())?;
                    exps = listAppend(tyexps.clone(), exps.clone());
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, .. } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. } => {
                    Ok(getAllExpsVars(vars.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { varLst: vars, .. } => {
                    Ok(getAllExpsVars(vars.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { funcArg: fargs, funcResultType: ty, .. } => {
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tyexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut explists: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    explists = List::mapMap(fargs.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(funcArgType(&__a0)) }, &getAllExps)?;
                    tyexps = getAllExps(ty.clone())?;
                    exps = List::flatten(metamodelica::cons(tyexps.clone(), explists.clone()))?;
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut explist: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    explist = List::map(tys.clone(), &getAllExps)?;
                    exps = List::flatten(explist.clone())?;
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATUPLE { types: tys } => {
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    exps = getAllExpsTt(metamodelica::Ref::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }))?;
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAOPTION { ty } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { ty } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAARRAY { ty } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METABOXED { ty } => {
                    Ok(getAllExps(ty.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN { .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_NORETCALL { .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                tty => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    r#str = TypesDump::unparseType(tty.clone())?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Types.getAllExpsTt failed ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpExpLst)
}

fn getAllExpsVars(
    mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut explist: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    explist = List::map(vars, &move |__a0: metamodelica::Ref<DAE::Var>| getAllExpsVar(&__a0))?;
    exps = List::flatten(explist)?;
    Ok(exps)
}

fn getAllExpsVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpExpLst = (match &**inVar {
        DAE::Var { ty, binding: bnd, .. } => {
            let mut tyexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut bndexp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            tyexps = getAllExps(ty.clone())?;
            bndexp = getAllExpsBinding(bnd)?;
            exps = listAppend(tyexps, bndexp);
            exps
        }
    });
    Ok(outExpExpLst)
}

fn getAllExpsBinding(
    mut inBinding: &metamodelica::Ref<DAE::Binding>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpExpLst = (match &**inBinding {
        DAE::Binding::EQBOUND { exp, .. } => {
            list![exp.clone()]
        }
        DAE::Binding::UNBOUND { .. } => metamodelica::nil(),
        DAE::Binding::VALBOUND { .. } => metamodelica::nil(),
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("-- Types.getAllExpsBinding failed\n"))?;
            return Err("fail");
        }
    });
    Ok(outExpExpLst)
}

pub fn isBoxedType(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**ty {
        DAE::Type::T_STRING { .. } => true,
        DAE::Type::T_METAOPTION { .. } => true,
        DAE::Type::T_METALIST { .. } => true,
        DAE::Type::T_METATUPLE { .. } => true,
        DAE::Type::T_METAUNIONTYPE { .. } => true,
        DAE::Type::T_METARECORD { .. } => true,
        DAE::Type::T_METAPOLYMORPHIC { .. } => true,
        DAE::Type::T_METAARRAY { .. } => true,
        DAE::Type::T_FUNCTION { .. } => true,
        DAE::Type::T_METABOXED { .. } => true,
        DAE::Type::T_ANYTYPE { .. } => true,
        DAE::Type::T_UNKNOWN { .. } => true,
        DAE::Type::T_METATYPE { .. } => true,
        DAE::Type::T_NORETCALL { .. } => true,
        DAE::Type::T_CODE { .. } => true,
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::EXTERNAL_OBJ { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

pub fn isMetaBoxedType(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outIsMetaBoxed: bool;
    outIsMetaBoxed = (match &**inType {
        DAE::Type::T_METABOXED { .. } => true,
        _ => false,
    });
    outIsMetaBoxed
}

pub fn boxIfUnboxedType(mut ty: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = &*ty;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { .. } => {
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    tys = List::map(var_field!((*ty).types, DAE::Type::T_TUPLE).clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(if (isBoxedType(&ty)) {ty.clone()} else {metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: ty.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outType
}

pub fn unboxedType(mut ity: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(ity.clone()) {
            Deref @ DAE::Type::T_METABOXED { ty: __ity_ty } => {
                { ity = __ity_ty.clone(); continue '__tco; }
            },
            Deref @ DAE::Type::T_METAOPTION { ty: __ity_ty } => {
                let mut ty: Type;
                ty = unboxedType(__ity_ty.clone())?;
                ty = boxIfUnboxedType(ty);
                return Ok(metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: ty }))
            },
            Deref @ DAE::Type::T_METALIST { ty: __ity_ty } => {
                let mut ty: Type;
                ty = unboxedType(__ity_ty.clone())?;
                ty = boxIfUnboxedType(ty);
                return Ok(metamodelica::Ref::new(DAE::Type::T_METALIST { ty: ty }))
            },
            Deref @ DAE::Type::T_METATUPLE { types: __ity_types } => {
                let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                tys = List::mapMap(__ity_types.clone(), &unboxedType, &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                return Ok(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys }))
            },
            Deref @ DAE::Type::T_METAARRAY { ty: __ity_ty } => {
                let mut ty: Type;
                ty = unboxedType(__ity_ty.clone())?;
                ty = boxIfUnboxedType(ty);
                return Ok(metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: ty }))
            },
            t @ Deref @ DAE::Type::T_ARRAY { .. } => {
                let mut t = (*t).clone();
                assign_variant_field!(t => DAE::Type::T_ARRAY; ty = unboxedType(var_field!((*t).ty, DAE::Type::T_ARRAY).clone())?);
                return Ok(t.clone())
            },
            _ => {
                return Ok(ity)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn listMatchSuperType(
    mut ielist: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut typeList: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut printFailtrace: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut out: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut t: metamodelica::Ref<DAE::Type>;
    (out, t) = 'mc: {
        let __mc_input = (&**ielist, &**typeList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut st: Type;
                    let mut elist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    st = List::reduce(typeList, &superType)?;
                    st = superType(st.clone(), st.clone())?;
                    st = unboxedType(st.clone())?;
                    elist = listMatchSuperType2(ielist, typeList, &st, printFailtrace)?;
                    Ok((elist.clone(), st.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Types.listMatchSuperType failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((out, t))
}

fn listMatchSuperType2(
    mut elist: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut typeList: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut st: &metamodelica::Ref<DAE::Type>,
    mut printFailtrace: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut out: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    out = 'mc: {
        let __mc_input = (&**elist, &**typeList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: erest }, Deref @ metamodelica::ListNode::Cons { head: t, tail: trest }) => {
                    let mut e = (*e).clone();
                    let mut erest = (*erest).clone();
                    let mut t = (*t).clone();
                    (e, t) = matchType(e.clone(), t.clone(), st.clone(), printFailtrace)?;
                    erest = listMatchSuperType2(metamodelica::AsArg::as_arg(&erest), metamodelica::AsArg::as_arg(&trest), st, printFailtrace)?;
                    Ok(metamodelica::cons(e.clone(), erest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: _ }, _) => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    r#str = ExpressionBasics::printExpStr(e.clone())?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Types.listMatchSuperType2 failed: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(out)
}

pub fn superType(
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut out: metamodelica::Ref<DAE::Type>;
    out = 'mc: {
        let __mc_input = (inType1.clone(), inType2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ANYTYPE { .. }, t2) => {
                    Ok(t2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, Deref @ DAE::Type::T_ANYTYPE { .. }) => {
                    Ok(t1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_UNKNOWN { .. }, t2) => {
                    Ok(t2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, Deref @ DAE::Type::T_UNKNOWN { .. }) => {
                    Ok(t1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, t2 @ Deref @ DAE::Type::T_METAPOLYMORPHIC { .. }) => {
                    Ok(t2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: type_list1, .. }, Deref @ DAE::Type::T_TUPLE { types: type_list2, .. }) => {
                    let mut type_list1 = (*type_list1).clone();
                    let mut type_list2 = (*type_list2).clone();
                    type_list1 = List::map(type_list1.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    type_list2 = List::map(type_list2.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    type_list1 = List::threadMap(type_list1.clone(), type_list2.clone(), &superType)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: type_list1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: type_list1, .. }, Deref @ DAE::Type::T_METATUPLE { types: type_list2 }) => {
                    let mut type_list1 = (*type_list1).clone();
                    let mut type_list2 = (*type_list2).clone();
                    type_list1 = List::map(type_list1.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    type_list2 = List::map(type_list2.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    type_list1 = List::threadMap(type_list1.clone(), type_list2.clone(), &superType)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: type_list1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: type_list1 }, Deref @ DAE::Type::T_TUPLE { types: type_list2, .. }) => {
                    let mut type_list1 = (*type_list1).clone();
                    let mut type_list2 = (*type_list2).clone();
                    type_list1 = List::map(type_list1.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    type_list2 = List::map(type_list2.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    type_list1 = List::threadMap(type_list1.clone(), type_list2.clone(), &superType)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: type_list1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: type_list1 }, Deref @ DAE::Type::T_METATUPLE { types: type_list2 }) => {
                    let mut type_list1 = (*type_list1).clone();
                    let mut type_list2 = (*type_list2).clone();
                    type_list1 = List::map(type_list1.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    type_list2 = List::map(type_list2.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    type_list1 = List::threadMap(type_list1.clone(), type_list2.clone(), &superType)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: type_list1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METALIST { ty: t1 }, Deref @ DAE::Type::T_METALIST { ty: t2 }) => {
                    let mut tp: Type;
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    t1 = boxIfUnboxedType(t1.clone());
                    t2 = boxIfUnboxedType(t2.clone());
                    tp = superType(t1.clone(), t2.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METALIST { ty: tp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAOPTION { ty: t1 }, Deref @ DAE::Type::T_METAOPTION { ty: t2 }) => {
                    let mut tp: Type;
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    t1 = boxIfUnboxedType(t1.clone());
                    t2 = boxIfUnboxedType(t2.clone());
                    tp = superType(t1.clone(), t2.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: tp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { ty: t1 }, Deref @ DAE::Type::T_METAARRAY { ty: t2 }) => {
                    let mut tp: Type;
                    let mut t1 = (*t1).clone();
                    let mut t2 = (*t2).clone();
                    t1 = boxIfUnboxedType(t1.clone());
                    t2 = boxIfUnboxedType(t2.clone());
                    tp = superType(t1.clone(), t2.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: tp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1 @ Deref @ DAE::Type::T_METAUNIONTYPE { path: path1, .. }, Deref @ DAE::Type::T_METARECORD { utPath: path2, .. }) => {
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2))) else { return Err("pattern mismatch") };
                    Ok(t1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METARECORD { knownSingleton: false, utPath: path1, .. }, Deref @ DAE::Type::T_METARECORD { knownSingleton: false, utPath: path2, .. }) => {
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2))) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Type::T_METAUNIONTYPE { paths: metamodelica::nil(), typeVars: var_field!((*inType1).typeVars, DAE::Type::T_METARECORD).clone(), knownSingleton: false, singletonType: openmodelica_frontend_types::DAE::EvaluateSingletonType::interned_NOT_SINGLETON(), path: path1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, t2) => {
                    let true = (subtype(t1.clone(), t2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(t2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t1, t2) => {
                    let true = (subtype(t2.clone(), t1.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(t1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(out)
}

pub fn matchTypePolymorphic(
    mut iexp: metamodelica::Ref<DAE::Exp>,
    mut iactual: metamodelica::Ref<DAE::Type>,
    mut expected: metamodelica::Ref<DAE::Type>,
    mut envPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut ipolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut printFailtrace: bool,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = iexp;
    let mut actual: metamodelica::Ref<DAE::Type> = iactual;
    let mut polymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> =
        ipolymorphicBindings;
    let debug: bool = false;
    if (getAllInnerTypesOfType(
        expected.clone(),
        &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isPolymorphic(&__a0))
        },
    )?)
    .is_empty()
    {
        (exp, actual) = matchType(exp, actual, expected, printFailtrace)?;
    } else {
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("match type: "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?);
                __mm_s.push_str(&*literal!(" of "));
                __mm_s.push_str(&*TypesDump::unparseType(actual.clone())?);
                __mm_s.push_str(&*literal!(" with "));
                __mm_s.push_str(&*TypesDump::unparseType(expected.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        (exp, actual) = matchType(
            exp,
            actual,
            metamodelica::Ref::new(DAE::Type::T_METABOXED {
                ty: DAE::T_UNKNOWN_DEFAULT().clone(),
            }),
            printFailtrace,
        )?;
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("matched type: "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?);
                __mm_s.push_str(&*literal!(" of "));
                __mm_s.push_str(&*TypesDump::unparseType(actual.clone())?);
                __mm_s.push_str(&*literal!(" with "));
                __mm_s.push_str(&*TypesDump::unparseType(expected.clone())?);
                __mm_s.push_str(&*literal!(" (boxed)\n"));
                ArcStr::from(__mm_s)
            });
        }
        polymorphicBindings = subtypePolymorphic(
            getUniontypeIfMetarecordReplaceAllSubtypes(actual.clone())?,
            getUniontypeIfMetarecordReplaceAllSubtypes(expected.clone())?,
            envPath,
            &polymorphicBindings,
        )?;
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("match type: "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?);
                __mm_s.push_str(&*literal!(" of "));
                __mm_s.push_str(&*TypesDump::unparseType(actual.clone())?);
                __mm_s.push_str(&*literal!(" with "));
                __mm_s.push_str(&*TypesDump::unparseType(expected)?);
                __mm_s.push_str(&*literal!(" and bindings "));
                __mm_s.push_str(&*polymorphicBindingsStr(polymorphicBindings.clone())?);
                __mm_s.push_str(&*literal!(" (OK)\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok((exp, actual, polymorphicBindings))
}

pub fn matchTypePolymorphicWithError(
    mut iexp: metamodelica::Ref<DAE::Exp>,
    mut iactual: metamodelica::Ref<DAE::Type>,
    mut iexpected: metamodelica::Ref<DAE::Type>,
    mut envPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut ipolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    (outExp, outType, outBindings) = 'mc: {
        let __mc_input = (iexp.clone(), iactual.clone(), iexpected.clone(), ipolymorphicBindings);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, actual, expected, polymorphicBindings) => {
                    let mut exp = (*exp).clone();
                    let mut actual = (*actual).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    (exp, actual, polymorphicBindings) = matchTypePolymorphic(exp.clone(), actual.clone(), expected.clone(), envPath.clone(), polymorphicBindings.clone(), false)?;
                    Ok((exp.clone(), actual.clone(), polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut str3: ArcStr;
                    str1 = ExpressionBasics::printExpStr(iexp.clone())?;
                    str2 = TypesDump::unparseType(iactual.clone())?;
                    str3 = TypesDump::unparseType(iexpected.clone())?;
                    Error::addSourceMessage(&(Error::EXP_TYPE_MISMATCH.clone()), list![str1.clone(), str3.clone(), str2.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outType, outBindings))
}

pub fn matchType(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inActualType: metamodelica::Ref<DAE::Type>,
    mut inExpectedType: metamodelica::Ref<DAE::Type>,
    mut inPrintFailtrace: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    if subtype(inExpectedType.clone(), inActualType.clone(), true) {
        outExp = inExp;
        outType = inActualType;
    } else {
        match '__try0: {
            let false = (subtype(inActualType.clone(), inExpectedType.clone(), true)) else {
                break '__try0 Err::<_, _>("pattern mismatch");
            };
            (outExp, outType) = unwrap_break_err!(typeConvert(inExp.clone(), inActualType.clone(), inExpectedType.clone(), inPrintFailtrace), '__try0);
            (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify1(outExp.clone()), '__try0);
            Ok::<_, &'static str>((outExp.clone(), outType.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                outExp = __try0_o0;
                outType = __try0_o1;
            }
            Err(__try0_err) => {
                printFailure(
                    Flags::TYPES.clone(),
                    &(literal!("matchType")),
                    inExp.clone(),
                    inActualType.clone(),
                    inExpectedType.clone(),
                )?;
                return Err(__try0_err);
            }
        }
    }
    Ok((outExp, outType))
}

pub fn matchTypeNoFail(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inActualType: metamodelica::Ref<DAE::Type>,
    mut inExpectedType: metamodelica::Ref<DAE::Type>,
) -> (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outMatch: bool;
    if subtype(inExpectedType.clone(), inActualType.clone(), true) {
        outExp = inExp;
        outType = inActualType;
        outMatch = true;
    } else {
        match '__try0: {
            (outExp, outType) = unwrap_break_err!(typeConvert(inExp.clone(), inActualType.clone(), inExpectedType.clone(), false), '__try0);
            (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify1(outExp.clone()), '__try0);
            outMatch = true;
            Ok::<_, &'static str>((outExp.clone(), outMatch.clone(), outType.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                outExp = __try0_o0;
                outMatch = __try0_o1;
                outType = __try0_o2;
            }
            Err(_) => {
                outExp = inExp.clone();
                outType = inActualType.clone();
                outMatch = true;
            }
        }
    }
    (outExp, outType, outMatch)
}

pub fn matchTypes(
    mut iexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut itys: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut expected: &metamodelica::Ref<DAE::Type>,
    mut printFailtrace: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outTys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outExps, outTys) = matchTypes_tail(
        iexps,
        itys,
        expected,
        printFailtrace,
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    Ok((outExps, outTys))
}

fn matchTypes_tail<'__b>(
    mut iexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut itys: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut expected: &'__b metamodelica::Ref<DAE::Type>,
    mut printFailtrace: bool,
    mut inAccumExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAccumTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((iexps, itys)) {
            (Deref @ metamodelica::ListNode::Cons { head: e, tail: exps }, Deref @ metamodelica::ListNode::Cons { head: ty, tail: tys }) => {
                let mut e = (*e).clone();
                let mut exps = (*exps).clone();
                let mut ty = (*ty).clone();
                let mut tys = (*tys).clone();
                (e, ty) = matchTypes2(e.clone(), ty.clone(), expected.clone(), printFailtrace)?;
                { (iexps, itys, expected, printFailtrace, inAccumExps, inAccumTypes) = (exps.clone(), tys.clone(), expected, printFailtrace, metamodelica::cons(e.clone(), inAccumExps), metamodelica::cons(ty.clone(), inAccumTypes)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((inAccumExps.reverse(), inAccumTypes.reverse()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn matchTypes2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inExpected: metamodelica::Ref<DAE::Type>,
    mut inPrintFailtrace: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outExp, outType) = 'mc: {
        let __mc_input = inPrintFailtrace;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut expected_ty: metamodelica::Ref<DAE::Type>;
            ty = getUniontypeIfMetarecordReplaceAllSubtypes(inType.clone())?;
            expected_ty = getUniontypeIfMetarecordReplaceAllSubtypes(inExpected.clone())?;
            (e, ty) = matchType(inExp.clone(), ty.clone(), expected_ty.clone(), inPrintFailtrace)?;
            Ok((e.clone(), ty.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- Types.matchTypes failed for "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?);
                __mm_s.push_str(&*literal!(" from "));
                __mm_s.push_str(&*TypesDump::unparseType(inType.clone())?);
                __mm_s.push_str(&*literal!(" to "));
                __mm_s.push_str(&*TypesDump::unparseType(inExpected.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outType))
}

fn printFailure(
    mut flag: Flags::DebugFlag,
    mut source: &ArcStr,
    mut e: metamodelica::Ref<DAE::Exp>,
    mut e_type: metamodelica::Ref<DAE::Type>,
    mut expected_type: metamodelica::Ref<DAE::Type>,
) -> Result<()> {
    if Flags::isSet(flag)? {
        Debug::traceln({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("- Types."));
            __mm_s.push_str(&*source);
            __mm_s.push_str(&*literal!(" failed on:"));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e)?);
            ArcStr::from(__mm_s)
        })?;
        Debug::traceln({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("  type:"));
            __mm_s.push_str(&*TypesDump::unparseType(e_type)?);
            __mm_s.push_str(&*literal!(" differs from expected\n  type:"));
            __mm_s.push_str(&*TypesDump::unparseType(expected_type)?);
            ArcStr::from(__mm_s)
        })?;
    }
    Ok(())
}

fn polymorphicBindingStr(mut binding: &(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (r#str, tys) = binding.clone();
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("    "));
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!(":\n"));
        __mm_s.push_str(&*stringDelimitList(
            List::map1r(
                List::map(tys, &TypesDump::unparseType)?,
                &fnptr!(stringAppend, ArcStr, ArcStr),
                literal!("      "),
            )?,
            literal!("\n"),
        ));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn polymorphicBindingsStr(
    mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        List::map(bindings, &move |__a0: (
            ArcStr,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
        )| polymorphicBindingStr(&__a0))?,
        literal!("\n"),
    );
    Ok(r#str)
}

pub fn fixPolymorphicRestype(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut bindings: &metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut resType: metamodelica::Ref<DAE::Type>;
    resType = fixPolymorphicRestype2(ty, &(literal!("$")), bindings, info)?;
    Ok(resType)
}

fn fixPolymorphicRestype2(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut prefix: &ArcStr,
    mut bindings: &metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut resType: metamodelica::Ref<DAE::Type>;
    resType = 'mc: {
        let __mc_input = ty.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id } => {
                    let mut t1: Type;
                    let __pa0 = ::match_deref::match_deref! { match &(polymorphicBindingsLookup(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefix); __mm_s.push_str(&*id); ArcStr::from(__mm_s) }), bindings)?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    t1 = metamodelica::Own::own(__pa0);
                    t1 = fixPolymorphicRestype2(t1.clone(), &(literal!("")), bindings, info)?;
                    Ok(t1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { ty: t1 } => {
                    let mut t2: Type;
                    t2 = fixPolymorphicRestype2(t1.clone(), prefix, bindings, info)?;
                    t2 = boxIfUnboxedType(t2.clone());
                    Ok(metamodelica::Ref::new(DAE::Type::T_METALIST { ty: t2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAARRAY { ty: t1 } => {
                    let mut t2: Type;
                    t2 = fixPolymorphicRestype2(t1.clone(), prefix, bindings, info)?;
                    t2 = boxIfUnboxedType(t2.clone());
                    Ok(metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: t2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAOPTION { ty: t1 } => {
                    let mut t2: Type;
                    t2 = fixPolymorphicRestype2(t1.clone(), prefix, bindings, info)?;
                    t2 = boxIfUnboxedType(t2.clone());
                    Ok(metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: t2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAUNIONTYPE { typeVars: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAUNIONTYPE { typeVars: tys, .. } => {
                    let mut tys = (*tys).clone();
                    tys = List::map3(tys.clone(), &move |__a0: metamodelica::Ref<DAE::Type>, __a1: ArcStr, __a2: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>, __a3: SourceInfo| fixPolymorphicRestype2(__a0, &__a1, &__a2, &__a3), prefix.clone(), bindings.clone(), info.clone())?;
                    tys = List::map(tys.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METAUNIONTYPE { paths: var_field!((*ty).paths, DAE::Type::T_METAUNIONTYPE).clone(), typeVars: tys.clone(), knownSingleton: var_field!((*ty).knownSingleton, DAE::Type::T_METAUNIONTYPE).clone(), singletonType: var_field!((*ty).singletonType, DAE::Type::T_METAUNIONTYPE).clone(), path: var_field!((*ty).path, DAE::Type::T_METAUNIONTYPE).clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATUPLE { types: tys } => {
                    let mut tys = (*tys).clone();
                    tys = List::map3(tys.clone(), &move |__a0: metamodelica::Ref<DAE::Type>, __a1: ArcStr, __a2: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>, __a3: SourceInfo| fixPolymorphicRestype2(__a0, &__a1, &__a2, &__a3), prefix.clone(), bindings.clone(), info.clone())?;
                    tys = List::map(tys.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                t1 @ Deref @ DAE::Type::T_ARRAY { .. } => {
                    let mut t1 = (*t1).clone();
                    assign_variant_field!(t1 => DAE::Type::T_ARRAY; ty = fixPolymorphicRestype2(var_field!((*t1).ty, DAE::Type::T_ARRAY).clone(), prefix, bindings, info)?);
                    Ok(t1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                t1 @ Deref @ DAE::Type::T_TUPLE { .. } => {
                    let mut t1 = (*t1).clone();
                    assign_variant_field!(t1 => DAE::Type::T_TUPLE; types = List::map3(var_field!((*t1).types, DAE::Type::T_TUPLE).clone(), &move |__a0: metamodelica::Ref<DAE::Type>, __a1: ArcStr, __a2: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>, __a3: SourceInfo| fixPolymorphicRestype2(__a0, &__a1, &__a2, &__a3), prefix.clone(), bindings.clone(), info.clone())?);
                    Ok(t1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { funcArg: args1, funcResultType: ty1, functionAttributes, path } => {
                    let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut args1 = (*args1).clone();
                    let mut ty1 = (*ty1).clone();
                    tys1 = List::map(args1.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(funcArgType(&__a0)) })?;
                    tys1 = List::map3(tys1.clone(), &move |__a0: metamodelica::Ref<DAE::Type>, __a1: ArcStr, __a2: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>, __a3: SourceInfo| fixPolymorphicRestype2(__a0, &__a1, &__a2, &__a3), prefix.clone(), bindings.clone(), info.clone())?;
                    ty1 = fixPolymorphicRestype2(ty1.clone(), prefix, bindings, info)?;
                    args1 = List::threadMap(args1.clone(), tys1.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> { ::std::result::Result::Ok(setFuncArgType(&__a0, __a1)) })?;
                    ty1 = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: args1.clone(), funcResultType: ty1.clone(), functionAttributes: functionAttributes.clone(), path: path.clone() });
                    Ok(ty1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut id: ArcStr;
                    let mut bstr: ArcStr;
                    let mut tstr: ArcStr;
                    tstr = TypesDump::unparseType(ty.clone())?;
                    bstr = polymorphicBindingsStr(bindings.clone())?;
                    id = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.fixPolymorphicRestype failed for type: ")); __mm_s.push_str(&*tstr); __mm_s.push_str(&*literal!(" using bindings: ")); __mm_s.push_str(&*bstr); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![id.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(resType)
}

pub(crate) fn polymorphicBindingsLookup(
    mut id: &ArcStr,
    mut bindings: &metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>> {
    let mut resType: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    resType = 'mc: {
        let __mc_input = &**bindings;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (id2, tys), tail: _ } => {
                    let true = (metamodelica::stringEq(&id, &id2)) else { return Err("pattern mismatch") };
                    Ok(List::map(tys.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    tys = polymorphicBindingsLookup(id, metamodelica::AsArg::as_arg(&rest))?;
                    Ok(tys.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(resType)
}

pub fn getAllInnerTypesOfType(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inFn: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>> {
    pub type TypeFn = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool> + 'static>;

    let mut outTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    outTypes = getAllInnerTypes(&(list![inType]), metamodelica::nil(), inFn)?;
    Ok(outTypes)
}

fn getAllInnerTypes(
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inAccum: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>> {
    pub type MatchFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool> + 'static>;

    let mut outTypes: metamodelica::List<metamodelica::Ref<DAE::Type>> = inAccum;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    for mut t in &**inTypes {
        if inFunc(t.clone())? {
            outTypes = metamodelica::cons(t.clone(), outTypes);
        }
        tys = (match &*t.clone() {
            DAE::Type::T_ARRAY { ty: __esc_ty, .. } => {
                ty = (*__esc_ty).clone();
                list![ty.clone()]
            }
            DAE::Type::T_METALIST { ty: __esc_ty } => {
                ty = (*__esc_ty).clone();
                list![ty.clone()]
            }
            DAE::Type::T_METAARRAY { ty: __esc_ty } => {
                ty = (*__esc_ty).clone();
                list![ty.clone()]
            }
            DAE::Type::T_METABOXED { ty: __esc_ty } => {
                ty = (*__esc_ty).clone();
                list![ty.clone()]
            }
            DAE::Type::T_METAOPTION { ty: __esc_ty } => {
                ty = (*__esc_ty).clone();
                list![ty.clone()]
            }
            DAE::Type::T_TUPLE { types: __esc_tys, .. } => {
                tys = (*__esc_tys).clone();
                tys.clone()
            }
            DAE::Type::T_METATUPLE { types: __esc_tys } => {
                tys = (*__esc_tys).clone();
                tys.clone()
            }
            DAE::Type::T_METAUNIONTYPE {
                typeVars: __esc_tys, ..
            } => {
                tys = (*__esc_tys).clone();
                tys.clone()
            }
            DAE::Type::T_METARECORD {
                typeVars: __esc_tys,
                fields,
                ..
            } => {
                tys = (*__esc_tys).clone();
                listAppend(
                    tys.clone(),
                    List::map(fields.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| {
                        getVarType(&__a0)
                    })?,
                )
            }
            DAE::Type::T_COMPLEX { varLst: fields, .. } => {
                List::map(fields.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| {
                    getVarType(&__a0)
                })?
            }
            DAE::Type::T_SUBTYPE_BASIC { varLst: fields, .. } => {
                List::map(fields.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| {
                    getVarType(&__a0)
                })?
            }
            DAE::Type::T_FUNCTION {
                funcArg: funcArgs,
                funcResultType: __esc_ty,
                ..
            } => {
                ty = (*__esc_ty).clone();
                metamodelica::cons(
                    ty.clone(),
                    List::map(
                        funcArgs.clone(),
                        &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(funcArgType(&__a0))
                        },
                    )?,
                )
            }
            _ => metamodelica::nil(),
        });
        outTypes = getAllInnerTypes(&tys, outTypes, inFunc)?;
    }
    Ok(outTypes)
}

pub fn uniontypeFilter(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**ty {
        DAE::Type::T_METAUNIONTYPE { paths: _, .. } => true,
        _ => false,
    });
    outMatch
}

pub(crate) fn metarecordFilter(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**ty {
        DAE::Type::T_METARECORD { path: _, .. } => true,
        _ => false,
    });
    outMatch
}

pub fn getUniontypePaths(
    mut ty: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outPaths = (match &**ty {
        DAE::Type::T_METAUNIONTYPE { paths, .. } => paths.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outPaths)
}

pub fn makeFunctionPolymorphicReference(
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inType {
        DAE::Type::T_FUNCTION {
            funcArg: funcArgs1,
            funcResultType: resType1,
            functionAttributes,
            path,
        } => {
            let mut funcArgs2: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
            let mut funcArgTypes1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut funcArgTypes2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut dummyBoxedTypeList: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut dummyExpList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut ty2: Type;
            let mut resType2: Type;
            funcArgTypes1 = List::map(
                funcArgs1.clone(),
                &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(funcArgType(&__a0))
                },
            )?;
            (dummyExpList, dummyBoxedTypeList) = makeDummyExpAndTypeLists(&funcArgTypes1)?;
            (_, funcArgTypes2) = matchTypeTuple(&dummyExpList, &funcArgTypes1, &dummyBoxedTypeList, false)?;
            funcArgs2 = List::threadMap(
                funcArgs1.clone(),
                funcArgTypes2,
                &move |__a0: metamodelica::Ref<DAE::FuncArg>,
                       __a1: metamodelica::Ref<DAE::Type>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(setFuncArgType(&__a0, __a1))
                },
            )?;
            resType2 = makeFunctionPolymorphicReferenceResType(resType1.clone())?;
            ty2 = metamodelica::Ref::new(DAE::Type::T_FUNCTION {
                funcArg: funcArgs2,
                funcResultType: resType2,
                functionAttributes: functionAttributes.clone(),
                path: path.clone(),
            });
            ty2
        }
        _ => return Err("fail"),
    });
    Ok(outType)
}

fn makeFunctionPolymorphicReferenceResType(
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ty @ Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut dummyBoxedTypeList: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut dummyExpList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ty = (*ty).clone();
                    let mut tys = (*tys).clone();
                    (dummyExpList, dummyBoxedTypeList) = makeDummyExpAndTypeLists(metamodelica::AsArg::as_arg(&tys))?;
                    (_, tys) = matchTypeTuple(&dummyExpList, metamodelica::AsArg::as_arg(&tys), &dummyBoxedTypeList, false)?;
                    assign_variant_field!(ty => DAE::Type::T_TUPLE; types = tys.clone());
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ty @ Deref @ DAE::Type::T_NORETCALL { .. } => {
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ty1 => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut ty: Type;
                    let mut ty2: Type;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(makeDummyExpAndTypeLists(&(list![ty1.clone()]))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    ty2 = metamodelica::Own::own(__pa1);
                    (_, ty) = matchType(e.clone(), ty1.clone(), ty2.clone(), false)?;
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

fn makeDummyExpAndTypeLists(
    mut lst: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outExps, outTypes) = (::match_deref::match_deref! { match lst {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            let mut restExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut restType: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
            let mut crefExp: metamodelica::Ref<DAE::Exp>;
            (restExp, restType) = makeDummyExpAndTypeLists(rest)?;
            cref_ = ComponentReferenceBasics::makeCrefIdent(literal!("#DummyExp#"), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
            crefExp = Expression::crefExp(cref_)?;
            (metamodelica::cons(crefExp, restExp), metamodelica::cons(metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: DAE::T_UNKNOWN_DEFAULT().clone() }), restType))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExps, outTypes))
}

pub(crate) fn resTypeToListTypes(
    mut inType: metamodelica::Ref<DAE::Type>,
) -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    outType = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
            tys.clone()
        },
        Deref @ DAE::Type::T_NORETCALL { .. } => {
            metamodelica::nil()
        },
        ty => {
            list![ty.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub(crate) fn getRealOrIntegerDimensions<'__b>(
    mut inType: &'__b metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inType {
            Deref @ DAE::Type::T_REAL { .. } => {
                return Ok(metamodelica::nil())
            },
            Deref @ DAE::Type::T_INTEGER { .. } => {
                return Ok(metamodelica::nil())
            },
            Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                { inType = ty; continue '__tco; }
            },
            Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d @ Deref @ DAE::Dimension::DIM_INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Nil }, ty } => {
                let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                dims = getRealOrIntegerDimensions(ty)?;
                return Ok(metamodelica::cons(d.clone(), dims))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isPolymorphic(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**ty {
        DAE::Type::T_METAPOLYMORPHIC { name: _ } => true,
        _ => false,
    });
    outMatch
}

pub(crate) fn polymorphicTypeName(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ DAE::Type::T_METAPOLYMORPHIC { name: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    Ok(name)
}

pub fn addPolymorphicBinding(
    mut id: ArcStr,
    mut ity: metamodelica::Ref<DAE::Type>,
    mut bindings: &metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>> {
    let mut outBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    outBindings = 'mc: {
        let __mc_input = (id.clone(), ity, &**bindings);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ty, Deref @ metamodelica::ListNode::Nil) => {
                    let mut ty = (*ty).clone();
                    ty = unboxedType(ty.clone())?;
                    ty = boxIfUnboxedType(ty.clone());
                    Ok(list![(id.clone(), list![ty.clone()])])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id1, ty, Deref @ metamodelica::ListNode::Cons { head: (id2, tys), tail: rest }) => {
                    let mut ty = (*ty).clone();
                    let true = (metamodelica::stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    ty = unboxedType(ty.clone())?;
                    ty = boxIfUnboxedType(ty.clone());
                    Ok(metamodelica::cons((id2.clone(), metamodelica::cons(ty.clone(), tys.clone())), rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ty, Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }) => {
                    let mut rest = (*rest).clone();
                    rest = addPolymorphicBinding(id.clone(), ty.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(metamodelica::cons(first.clone(), rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outBindings)
}

pub fn solvePolymorphicBindings(
    mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut info: &SourceInfo,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>> {
    let mut solvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    let mut unsolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    (solvedBindings, unsolvedBindings) =
        solvePolymorphicBindingsLoop(&bindings, metamodelica::nil(), metamodelica::nil())?;
    checkValidBindings(bindings, solvedBindings.clone(), unsolvedBindings, info, path)?;
    Ok(solvedBindings)
}

fn checkValidBindings(
    mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut solvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut unsolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut info: &SourceInfo,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<()> {
    let mut bindingsStr: ArcStr;
    let mut solvedBindingsStr: ArcStr;
    let mut unsolvedBindingsStr: ArcStr;
    let mut pathStr: ArcStr;
    if !((unsolvedBindings).is_empty()) {
        pathStr = AbsynUtil::pathString(path, literal!("."), true, false)?;
        bindingsStr = polymorphicBindingsStr(bindings)?;
        solvedBindingsStr = polymorphicBindingsStr(solvedBindings)?;
        unsolvedBindingsStr = polymorphicBindingsStr(unsolvedBindings)?;
        Error::addSourceMessage(
            &(Error::META_UNSOLVED_POLYMORPHIC_BINDINGS.clone()),
            list![pathStr, bindingsStr, solvedBindingsStr, unsolvedBindingsStr],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

fn solvePolymorphicBindingsLoop(
    mut ibindings: &metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut isolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut iunsolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<(
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outSolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    let mut outUnsolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    (outSolvedBindings, outUnsolvedBindings) = 'mc: {
        let __mc_input = (&**ibindings, isolvedBindings, iunsolvedBindings);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, solvedBindings, unsolvedBindings) => {
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (id, Deref @ metamodelica::ListNode::Cons { head: ty, tail: Deref @ metamodelica::ListNode::Nil }), tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut ty = (*ty).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    ty = boxIfUnboxedType(ty.clone());
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(&(listAppend(unsolvedBindings.clone(), rest.clone())), metamodelica::cons((id.clone(), list![ty.clone()]), solvedBindings.clone()), metamodelica::nil())?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (id, tys), tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut tys = (*tys).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    tys = replaceSolvedBindings(metamodelica::AsArg::as_arg(&tys), solvedBindings.clone(), false)?;
                    tys = List::unionOnTrue(metamodelica::AsArg::as_arg(&tys), &(metamodelica::nil()), &fnptr!(equivtypes, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>))?;
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(&(listAppend(metamodelica::cons((id.clone(), tys.clone()), unsolvedBindings.clone()), rest.clone())), solvedBindings.clone(), metamodelica::nil())?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (id, tys), tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut tys = (*tys).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    (tys, solvedBindings) = solveBindings(tys.clone(), metamodelica::AsArg::as_arg(&tys), solvedBindings.clone())?;
                    tys = List::unionOnTrue(metamodelica::AsArg::as_arg(&tys), &(metamodelica::nil()), &fnptr!(equivtypes, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>))?;
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(&(listAppend(metamodelica::cons((id.clone(), tys.clone()), unsolvedBindings.clone()), rest.clone())), solvedBindings.clone(), metamodelica::nil())?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (id, tys), tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut len1: i32;
                    let mut len2: i32;
                    let mut tys = (*tys).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    len1 = ((tys).len() as i32);
                    let true = (len1 > 1) else { return Err("pattern mismatch") };
                    tys = List::unionOnTrue(metamodelica::AsArg::as_arg(&tys), &(metamodelica::nil()), &fnptr!(equivtypes, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>))?;
                    len2 = ((tys).len() as i32);
                    let false = (len1 == len2) else { return Err("pattern mismatch") };
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(&(listAppend(metamodelica::cons((id.clone(), tys.clone()), unsolvedBindings.clone()), rest.clone())), solvedBindings.clone(), metamodelica::nil())?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }, solvedBindings, unsolvedBindings) => {
                    let mut solvedBindings = (*solvedBindings).clone();
                    let mut unsolvedBindings = (*unsolvedBindings).clone();
                    (solvedBindings, unsolvedBindings) = solvePolymorphicBindingsLoop(metamodelica::AsArg::as_arg(&rest), solvedBindings.clone(), metamodelica::cons(first.clone(), unsolvedBindings.clone()))?;
                    Ok((solvedBindings.clone(), unsolvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outSolvedBindings, outUnsolvedBindings))
}

fn solveBindings(
    mut itys1: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut itys2: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut isolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outTys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut outSolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    (outTys, outSolvedBindings) = 'mc: {
        let __mc_input = (itys1, &**itys2, isolvedBindings);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ty1 @ Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id1 }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: ty2 @ Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id2 }, tail: tys2 }, solvedBindings) => {
                    let mut ty: Type;
                    let mut id: ArcStr;
                    let mut fromOtherFunction: bool;
                    let mut solvedBindings = (*solvedBindings).clone();
                    let false = (metamodelica::stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    fromOtherFunction = System::stringFind(id1.clone(), literal!("$"))? != -1;
                    id = if (fromOtherFunction) {id1.clone()} else {id2.clone()};
                    ty = if (fromOtherFunction) {ty2.clone()} else {ty1.clone()};
                    if '__try0: {
                        unwrap_break_err!(polymorphicBindingsLookup(&id, metamodelica::AsArg::as_arg(&solvedBindings)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    solvedBindings = addPolymorphicBinding(id.clone(), ty.clone(), metamodelica::AsArg::as_arg(&solvedBindings))?;
                    Ok((metamodelica::cons(ty.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: ty2, tail: tys2 }, solvedBindings) => {
                    let mut solvedBindings = (*solvedBindings).clone();
                    let false = (isPolymorphic(metamodelica::AsArg::as_arg(&ty2))) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(polymorphicBindingsLookup(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&solvedBindings)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    solvedBindings = addPolymorphicBinding(id.clone(), ty2.clone(), metamodelica::AsArg::as_arg(&solvedBindings))?;
                    Ok((metamodelica::cons(ty2.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ty1, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }, tail: tys2 }, solvedBindings) => {
                    let mut solvedBindings = (*solvedBindings).clone();
                    let false = (isPolymorphic(metamodelica::AsArg::as_arg(&ty1))) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(polymorphicBindingsLookup(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&solvedBindings)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    solvedBindings = addPolymorphicBinding(id.clone(), ty1.clone(), metamodelica::AsArg::as_arg(&solvedBindings))?;
                    Ok((metamodelica::cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METAOPTION { ty: ty1 }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METAOPTION { ty: ty2 }, tail: tys2 }, solvedBindings) => {
                    let mut ty1 = (*ty1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(solveBindings(list![ty1.clone()], &(list![ty2.clone()]), solvedBindings.clone())?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ty1 = metamodelica::Own::own(__pa0);
                    solvedBindings = metamodelica::Own::own(__pa1);
                    ty1 = metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: ty1.clone() });
                    Ok((metamodelica::cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METALIST { ty: ty1 }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METALIST { ty: ty2 }, tail: tys2 }, solvedBindings) => {
                    let mut ty1 = (*ty1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(solveBindings(list![ty1.clone()], &(list![ty2.clone()]), solvedBindings.clone())?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ty1 = metamodelica::Own::own(__pa0);
                    solvedBindings = metamodelica::Own::own(__pa1);
                    ty1 = metamodelica::Ref::new(DAE::Type::T_METALIST { ty: ty1.clone() });
                    Ok((metamodelica::cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METAARRAY { ty: ty1 }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METAARRAY { ty: ty2 }, tail: tys2 }, solvedBindings) => {
                    let mut ty1 = (*ty1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(solveBindings(list![ty1.clone()], &(list![ty2.clone()]), solvedBindings.clone())?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ty1 = metamodelica::Own::own(__pa0);
                    solvedBindings = metamodelica::Own::own(__pa1);
                    ty1 = metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: ty1.clone() });
                    Ok((metamodelica::cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METATUPLE { types: tys1 }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_METATUPLE { types: tys2 }, tail: rest }, solvedBindings) => {
                    let mut ty1: Type;
                    let mut tys1 = (*tys1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    (tys1, solvedBindings) = solveBindingsThread(metamodelica::AsArg::as_arg(&tys1), metamodelica::AsArg::as_arg(&tys2), false, solvedBindings.clone())?;
                    ty1 = metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys1.clone() });
                    Ok((metamodelica::cons(ty1.clone(), rest.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: args1, funcResultType: ty1, functionAttributes: functionAttributes1, path }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: args2, funcResultType: ty2, functionAttributes: _, path: _ }, tail: rest }, solvedBindings) => {
                    let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut args1 = (*args1).clone();
                    let mut ty1 = (*ty1).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    tys1 = List::map(args1.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(funcArgType(&__a0)) })?;
                    tys2 = List::map(args2.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(funcArgType(&__a0)) })?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(solveBindingsThread(&(metamodelica::cons(ty1.clone(), tys1.clone())), &(metamodelica::cons(ty2.clone(), tys2.clone())), false, solvedBindings.clone())?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ty1 = metamodelica::Own::own(__pa0);
                    tys1 = metamodelica::Own::own(__pa1);
                    solvedBindings = metamodelica::Own::own(__pa2);
                    tys1 = List::map(tys1.clone(), &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    args1 = List::threadMap(args1.clone(), tys1.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> { ::std::result::Result::Ok(setFuncArgType(&__a0, __a1)) })?;
                    args1 = List::map(args1.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(clearDefaultBinding(&__a0)) })?;
                    ty1 = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: args1.clone(), funcResultType: ty1.clone(), functionAttributes: functionAttributes1.clone(), path: path.clone() });
                    Ok((metamodelica::cons(ty1.clone(), rest.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tys1, Deref @ metamodelica::ListNode::Cons { head: ty, tail: tys2 }, solvedBindings) => {
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut solvedBindings = (*solvedBindings).clone();
                    (tys, solvedBindings) = solveBindings(tys1.clone(), metamodelica::AsArg::as_arg(&tys2), solvedBindings.clone())?;
                    Ok((metamodelica::cons(ty.clone(), tys.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outTys, outSolvedBindings))
}

fn solveBindingsThread(
    mut itys1: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut itys2: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut changed: bool,
    mut isolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outTys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut outSolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    (outTys, outSolvedBindings) = 'mc: {
        let __mc_input = (&**itys1, &**itys2, changed, isolvedBindings);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ty1, tail: tys1 }, Deref @ metamodelica::ListNode::Cons { head: ty2, tail: tys2 }, _, solvedBindings) => {
                    let mut ty1 = (*ty1).clone();
                    let mut tys2 = (*tys2).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(solveBindings(list![ty1.clone()], &(list![ty2.clone()]), solvedBindings.clone())?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ty1 = metamodelica::Own::own(__pa0);
                    solvedBindings = metamodelica::Own::own(__pa1);
                    (tys2, solvedBindings) = solveBindingsThread(metamodelica::AsArg::as_arg(&tys1), metamodelica::AsArg::as_arg(&tys2), true, solvedBindings.clone())?;
                    Ok((metamodelica::cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ty1, tail: tys1 }, Deref @ metamodelica::ListNode::Cons { head: _, tail: tys2 }, _, solvedBindings) => {
                    let mut tys2 = (*tys2).clone();
                    let mut solvedBindings = (*solvedBindings).clone();
                    (tys2, solvedBindings) = solveBindingsThread(metamodelica::AsArg::as_arg(&tys1), metamodelica::AsArg::as_arg(&tys2), changed, solvedBindings.clone())?;
                    Ok((metamodelica::cons(ty1.clone(), tys2.clone()), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, true, solvedBindings) => {
                    Ok((metamodelica::nil(), solvedBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outTys, outSolvedBindings))
}

fn replaceSolvedBindings(
    mut itys: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut isolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut changed: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>> {
    let mut outTys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    outTys = 'mc: {
        let __mc_input = (&**itys, isolvedBindings, changed);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, true) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ty, tail: tys }, solvedBindings, _) => {
                    let mut ty = (*ty).clone();
                    let mut tys = (*tys).clone();
                    ty = replaceSolvedBinding(metamodelica::AsArg::as_arg(&ty), solvedBindings.clone())?;
                    tys = replaceSolvedBindings(metamodelica::AsArg::as_arg(&tys), solvedBindings.clone(), true)?;
                    Ok(metamodelica::cons(ty.clone(), tys.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ty, tail: tys }, solvedBindings, _) => {
                    let mut tys = (*tys).clone();
                    tys = replaceSolvedBindings(metamodelica::AsArg::as_arg(&tys), solvedBindings.clone(), changed)?;
                    Ok(metamodelica::cons(ty.clone(), tys.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTys)
}

fn replaceSolvedBinding(
    mut ity: &metamodelica::Ref<DAE::Type>,
    mut isolvedBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outTy: metamodelica::Ref<DAE::Type>;
    outTy = (match &**ity {
        DAE::Type::T_METALIST { ty } => {
            let mut solvedBindings = isolvedBindings;
            let mut ty = (*ty).clone();
            ty = replaceSolvedBinding(metamodelica::AsArg::as_arg(&ty), solvedBindings)?;
            ty = metamodelica::Ref::new(DAE::Type::T_METALIST { ty: ty.clone() });
            ty.clone()
        }
        DAE::Type::T_METAARRAY { ty } => {
            let mut solvedBindings = isolvedBindings;
            let mut ty = (*ty).clone();
            ty = replaceSolvedBinding(metamodelica::AsArg::as_arg(&ty), solvedBindings)?;
            ty = metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: ty.clone() });
            ty.clone()
        }
        DAE::Type::T_METAOPTION { ty } => {
            let mut solvedBindings = isolvedBindings;
            let mut ty = (*ty).clone();
            ty = replaceSolvedBinding(metamodelica::AsArg::as_arg(&ty), solvedBindings)?;
            ty = metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: ty.clone() });
            ty.clone()
        }
        DAE::Type::T_METATUPLE { types: tys } => {
            let mut solvedBindings = isolvedBindings;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut tys = (*tys).clone();
            tys = replaceSolvedBindings(metamodelica::AsArg::as_arg(&tys), solvedBindings, false)?;
            ty = metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys.clone() });
            ty
        }
        DAE::Type::T_TUPLE { types: tys, .. } => {
            let mut solvedBindings = isolvedBindings;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut tys = (*tys).clone();
            tys = replaceSolvedBindings(metamodelica::AsArg::as_arg(&tys), solvedBindings, false)?;
            ty = metamodelica::Ref::new(DAE::Type::T_TUPLE {
                types: tys.clone(),
                names: var_field!((**ity).names, DAE::Type::T_TUPLE).clone(),
            });
            ty
        }
        DAE::Type::T_FUNCTION {
            funcArg: args,
            funcResultType: resType,
            functionAttributes,
            path,
        } => {
            let mut solvedBindings = isolvedBindings;
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut args = (*args).clone();
            tys = List::map(
                args.clone(),
                &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(funcArgType(&__a0))
                },
            )?;
            tys = replaceSolvedBindings(&(metamodelica::cons(resType.clone(), tys)), solvedBindings, false)?;
            tys = List::map(tys, &unboxedType)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::map(tys, &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            tys = metamodelica::Own::own(__pa1);
            args = List::threadMap(args.clone(), tys, &move |__a0: metamodelica::Ref<DAE::FuncArg>,
                                                             __a1: metamodelica::Ref<DAE::Type>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(setFuncArgType(&__a0, __a1))
            })?;
            ty = makeRegularTupleFromMetaTupleOnTrue(isTuple(resType), ty)?;
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION {
                funcArg: args.clone(),
                funcResultType: ty,
                functionAttributes: functionAttributes.clone(),
                path: path.clone(),
            });
            ty
        }
        DAE::Type::T_METAPOLYMORPHIC { name: id } => {
            let mut solvedBindings = isolvedBindings;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let __pa0 = ::match_deref::match_deref! { match &(polymorphicBindingsLookup(id, &solvedBindings)?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            ty
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outTy)
}

fn subtypePolymorphic(
    mut actual: metamodelica::Ref<DAE::Type>,
    mut expected: metamodelica::Ref<DAE::Type>,
    mut envPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inBindings: &metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>> {
    let mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> =
        metamodelica::nil();
    bindings = 'mc: {
        let __mc_input = (actual.clone(), expected.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }) => {
                    Ok(addPolymorphicBinding({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*id); ArcStr::from(__mm_s) }, actual.clone(), inBindings)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAPOLYMORPHIC { name: id }, _) => {
                    if stringGet(&id,1)? != stringCharInt(literal!("$"))? {
                        return Err("fail");
                    }
                    Ok(addPolymorphicBinding({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$$")); __mm_s.push_str(&*id); ArcStr::from(__mm_s) }, expected.clone(), inBindings)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METABOXED { ty: ty1 }, ty2) => {
                    let mut ty1 = (*ty1).clone();
                    ty1 = unboxedType(ty1.clone())?;
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, Deref @ DAE::Type::T_METABOXED { ty: ty2 }) => {
                    let mut ty2 = (*ty2).clone();
                    ty2 = unboxedType(ty2.clone())?;
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_NORETCALL { .. }, Deref @ DAE::Type::T_NORETCALL { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_STRING { .. }, Deref @ DAE::Type::T_STRING { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_BOOL { .. }, Deref @ DAE::Type::T_BOOL { .. }) => {
                    Ok(inBindings.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { names: names1, .. }, Deref @ DAE::Type::T_ENUMERATION { names: names2, .. }) => {
                    let true = (List::isEqualOnTrue(names1.clone(), names2.clone(), &fnptr!(stringEq, ArcStr, ArcStr))?) else { return Err("pattern mismatch") };
                    Ok(inBindings.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: ty1, .. }, Deref @ DAE::Type::T_ARRAY { ty: ty2, .. }) => {
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { ty: ty1 }, Deref @ DAE::Type::T_METAARRAY { ty: ty2 }) => {
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METALIST { ty: ty1 }, Deref @ DAE::Type::T_METALIST { ty: ty2 }) => {
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAOPTION { ty: ty1 }, Deref @ DAE::Type::T_METAOPTION { ty: ty2 }) => {
                    Ok(subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), inBindings)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METATUPLE { types: tList1 }, Deref @ DAE::Type::T_METATUPLE { types: tList2 }) => {
                    Ok(subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: tList1, .. }, Deref @ DAE::Type::T_TUPLE { types: tList2, .. }) => {
                    Ok(subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAUNIONTYPE { .. }, Deref @ DAE::Type::T_METAUNIONTYPE { .. }) => {
                    let true = (AbsynUtil::pathEqual(var_field!((*actual).path, DAE::Type::T_METAUNIONTYPE), var_field!((*expected).path, DAE::Type::T_METAUNIONTYPE))) else { return Err("pattern mismatch") };
                    Ok(subtypePolymorphicList(var_field!((*actual).typeVars, DAE::Type::T_METAUNIONTYPE).clone(), var_field!((*expected).typeVars, DAE::Type::T_METAUNIONTYPE).clone(), envPath.clone(), inBindings.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: path1 }, .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: path2 }, .. }) => {
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2))) else { return Err("pattern mismatch") };
                    Ok(inBindings.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: farg1, funcResultType: ty1, functionAttributes: _, path: path1 }, Deref @ DAE::Type::T_FUNCTION { funcArg: farg2, funcResultType: ty2, functionAttributes: _, path: _ }) => {
                    let mut prefix: ArcStr;
                    let mut tList1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tList2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut farg1 = (*farg1).clone();
                    let mut ty1 = (*ty1).clone();
                    let mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> = bindings.clone();
                    if AbsynUtil::pathPrefixOf(envPath.clone().unwrap_or(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("$TOP$") })), path1.clone()) {
                        tList1 = List::map(farg1.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(funcArgType(&__a0)) })?;
                        tList2 = List::map(farg2.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(funcArgType(&__a0)) })?;
                        bindings = subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), inBindings.clone())?;
                        bindings = subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), &bindings)?;
                    } else {
                        prefix = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*AbsynUtil::pathString(path1.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) };
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseType(actual.clone(), prefix.clone(), &fnptr!(prefixTraversedPolymorphicType, metamodelica::Ref<DAE::Type>, ArcStr))?) {
                            (Deref @ DAE::Type::T_FUNCTION { funcArg: __pa0, funcResultType: __pa1, functionAttributes: _, path: _ }, _) => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        farg1 = metamodelica::Own::own(__pa0);
                        ty1 = metamodelica::Own::own(__pa1);
                        tList1 = List::map(farg1.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(funcArgType(&__a0)) })?;
                        tList2 = List::map(farg2.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(funcArgType(&__a0)) })?;
                        bindings = subtypePolymorphicList(tList1.clone(), tList2.clone(), envPath.clone(), inBindings.clone())?;
                        bindings = subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), &bindings)?;
                    }
                    Ok((bindings.clone(), bindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            bindings = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_UNKNOWN { .. }, ty2) => {
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut ids: metamodelica::List<ArcStr>;
                    let mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> = bindings.clone();
                    tys = getAllInnerTypesOfType(ty2.clone(), &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isPolymorphic(&__a0)) })?;
                    ids = List::map(tys.clone(), &move |__a0: metamodelica::Ref<DAE::Type>| polymorphicTypeName(&__a0))?;
                    bindings = List::fold1(&ids, &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::Type>, __a2: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>| addPolymorphicBinding(__a0, __a1, &__a2), actual.clone(), inBindings.clone())?;
                    Ok((bindings.clone(), bindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            bindings = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ANYTYPE { .. }, ty2) => {
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut ids: metamodelica::List<ArcStr>;
                    let mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> = bindings.clone();
                    tys = getAllInnerTypesOfType(ty2.clone(), &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isPolymorphic(&__a0)) })?;
                    ids = List::map(tys.clone(), &move |__a0: metamodelica::Ref<DAE::Type>| polymorphicTypeName(&__a0))?;
                    bindings = List::fold1(&ids, &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::Type>, __a2: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>| addPolymorphicBinding(__a0, __a1, &__a2), actual.clone(), inBindings.clone())?;
                    Ok((bindings.clone(), bindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            bindings = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(bindings)
}

fn subtypePolymorphicList(
    mut actual: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut expected: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut envPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut ibindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) -> Result<metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((actual, expected, ibindings)) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, bindings) => {
                return Ok(bindings.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: ty1, tail: tList1 }, Deref @ metamodelica::ListNode::Cons { head: ty2, tail: tList2 }, bindings) => {
                let mut bindings = (*bindings).clone();
                bindings = subtypePolymorphic(ty1.clone(), ty2.clone(), envPath.clone(), metamodelica::AsArg::as_arg(&bindings))?;
                { (actual, expected, envPath, ibindings) = (tList1.clone(), tList2.clone(), envPath, bindings.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn boxVarLst(
    mut vars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut ovars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    ovars = (::match_deref::match_deref! { match vars {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name, attributes, ty: type_, binding, bind_from_outside: bdsrc, constOfForIteratorRange }, tail: rest } => {
            let mut type_ = (*type_).clone();
            let mut rest = (*rest).clone();
            type_ = boxIfUnboxedType(type_.clone());
            rest = boxVarLst(metamodelica::AsArg::as_arg(&rest))?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: attributes.clone(), ty: type_.clone(), binding: binding.clone(), bind_from_outside: bdsrc.clone(), constOfForIteratorRange: constOfForIteratorRange.clone() }), rest.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ovars)
}

pub(crate) fn liftArraySubscript(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inSubscript: &metamodelica::Ref<DAE::Subscript>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match inSubscript {
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            let mut ty = inType;
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty, dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() })] })
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e } => {
            let mut ty = inType;
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty, dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: e.clone() })] })
        },
        _ => {
            let mut ty = inType;
            ty
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub(crate) fn liftArraySubscriptList(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inSubscriptLst: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match inSubscriptLst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut ty = inType;
            ty
        },
        Deref @ metamodelica::ListNode::Cons { head: sub, tail: rest } => {
            let mut ty = inType;
            liftArraySubscript(liftArraySubscriptList(ty, rest)?, metamodelica::AsArg::as_arg(&sub))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub fn convertTupleToMetaTuple(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut oexp: metamodelica::Ref<DAE::Exp>;
    let mut oty: metamodelica::Ref<DAE::Type>;
    (oexp, oty) = (match &*exp {
        DAE::Exp::TUPLE { PR: _ } => {
            (oexp, oty) = matchType(exp, ty, DAE::T_METABOXED_DEFAULT().clone(), false)?;
            (oexp, oty)
        }
        _ => (exp, ty),
    });
    Ok((oexp, oty))
}

pub fn isFunctionType(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**ty {
        DAE::Type::T_FUNCTION { .. } => true,
        _ => false,
    });
    b
}

fn prefixTraversedPolymorphicType(mut ty: Type, mut prefix: ArcStr) -> (Type, ArcStr) {
    let mut oty: Type = ty.clone();
    let mut r#str: ArcStr;
    (oty, r#str) = (match &*oty {
        DAE::Type::T_METAPOLYMORPHIC { name: __oty_name } => {
            assign_variant_field!(oty => DAE::Type::T_METAPOLYMORPHIC; name = { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix); __mm_s.push_str(&*__oty_name); ArcStr::from(__mm_s) });
            (oty, prefix)
        }
        _ => (ty, prefix),
    });
    (oty, r#str)
}

pub fn makeExpDimensionsUnknown(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut dummy: i32,
) -> (metamodelica::Ref<DAE::Type>, i32) {
    let mut oty: metamodelica::Ref<DAE::Type> = ty;
    let mut odummy: i32 = dummy;
    oty = (::match_deref::match_deref! { match &(oty.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()]);
            oty
        },
        _ => oty,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, odummy)
}

pub(crate) fn makeKnownDimensionsInteger(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut dummy: i32,
) -> (metamodelica::Ref<DAE::Type>, i32) {
    let mut oty: metamodelica::Ref<DAE::Type> = ty;
    let mut odummy: i32 = dummy;
    oty = (::match_deref::match_deref! { match &(oty.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_BOOLEAN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 2 })]);
            oty
        },
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_ENUM { size, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: size.clone() })]);
            oty
        },
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ICONST { integer: size } }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: size.clone() })]);
            oty
        },
        _ => {
            oty
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, odummy)
}

pub fn traverseType<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut arg: A,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, A) -> Result<(metamodelica::Ref<DAE::Type>, A)>,
) -> Result<(metamodelica::Ref<DAE::Type>, A)> {
    pub type Func<A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, A) -> Result<(metamodelica::Ref<DAE::Type>, A)> + 'static,
    >;

    let mut oty: metamodelica::Ref<DAE::Type>;
    let mut a: A = arg;
    (oty, a) = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_NORETCALL { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_UNKNOWN { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_METAPOLYMORPHIC { .. } => {
            (ty, a)
        },
        Deref @ DAE::Type::T_CODE { .. } => {
            (ty, a)
        },
        __esc_oty @ Deref @ DAE::Type::T_METABOXED { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METABOXED).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_METABOXED; ty = tyInner);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_ARRAY { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_ARRAY).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_ARRAY; ty = tyInner);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_METATYPE { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METATYPE).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_METATYPE; ty = tyInner);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_METALIST { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METALIST).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_METALIST; ty = tyInner);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_METAOPTION { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METAOPTION).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_METAOPTION; ty = tyInner);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_METAARRAY { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            (tyInner, a) = traverseType(var_field!((*oty).ty, DAE::Type::T_METAARRAY).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_METAARRAY; ty = tyInner);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            (tyInner, a) = traverseType(var_field!((*oty).functionType, DAE::Type::T_FUNCTION_REFERENCE_VAR).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_FUNCTION_REFERENCE_VAR; functionType = tyInner);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            (tyInner, a) = traverseType(var_field!((*oty).functionType, DAE::Type::T_FUNCTION_REFERENCE_FUNC).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_FUNCTION_REFERENCE_FUNC; functionType = tyInner);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_METATUPLE { .. } => {
            oty = (*__esc_oty).clone();
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            (tys, a) = traverseTupleType(var_field!((*oty).types, DAE::Type::T_METATUPLE), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_METATUPLE; types = tys);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_TUPLE { .. } => {
            oty = (*__esc_oty).clone();
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            (tys, a) = traverseTupleType(var_field!((*oty).types, DAE::Type::T_TUPLE), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_TUPLE; types = tys);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_METARECORD { .. } => {
            oty = (*__esc_oty).clone();
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            (vars, a) = traverseVarTypes(var_field!((*oty).fields, DAE::Type::T_METARECORD), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_METARECORD; fields = vars);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_COMPLEX { .. } => {
            oty = (*__esc_oty).clone();
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            (vars, a) = traverseVarTypes(var_field!((*oty).varLst, DAE::Type::T_COMPLEX), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_COMPLEX; varLst = vars);
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            (vars, a) = traverseVarTypes(var_field!((*oty).varLst, DAE::Type::T_SUBTYPE_BASIC), a, r#fn)?;
            (tyInner, a) = traverseType(var_field!((*oty).complexType, DAE::Type::T_SUBTYPE_BASIC).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_SUBTYPE_BASIC;
                varLst = vars,
                complexType = tyInner
            );
            (oty.clone(), a)
        },
        __esc_oty @ Deref @ DAE::Type::T_FUNCTION { .. } => {
            oty = (*__esc_oty).clone();
            let mut tyInner: Type;
            let mut farg: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
            (farg, a) = traverseFuncArg(var_field!((*oty).funcArg, DAE::Type::T_FUNCTION), a, r#fn)?;
            (tyInner, a) = traverseType(var_field!((*oty).funcResultType, DAE::Type::T_FUNCTION).clone(), a, r#fn)?;
            assign_variant_field!(oty => DAE::Type::T_FUNCTION;
                funcArg = farg,
                funcResultType = tyInner
            );
            (oty.clone(), a)
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.traverseType not implemented correctly: ")); __mm_s.push_str(&*TypesDump::unparseType(ty)?); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, a) = r#fn(oty, a)?;
    Ok((oty, a))
}

fn traverseTupleType<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut itys: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut ia: A,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, A) -> Result<(metamodelica::Ref<DAE::Type>, A)>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Type>>, A)> {
    pub type Func<A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, A) -> Result<(metamodelica::Ref<DAE::Type>, A)> + 'static,
    >;

    let mut otys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut oa: A;
    (otys, oa) = (::match_deref::match_deref! { match itys {
        Deref @ metamodelica::ListNode::Nil => {
            let mut a = ia;
            (metamodelica::nil(), a)
        },
        Deref @ metamodelica::ListNode::Cons { head: ty, tail: tys } => {
            let mut a = ia;
            let mut ty = (*ty).clone();
            let mut tys = (*tys).clone();
            (ty, a) = traverseType(ty.clone(), a, r#fn)?;
            (tys, a) = traverseTupleType(metamodelica::AsArg::as_arg(&tys), a, r#fn)?;
            (metamodelica::cons(ty.clone(), tys.clone()), a)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((otys, oa))
}

fn traverseVarTypes<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ivars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut ia: A,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, A) -> Result<(metamodelica::Ref<DAE::Type>, A)>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Var>>, A)> {
    pub type Func<A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, A) -> Result<(metamodelica::Ref<DAE::Type>, A)> + 'static,
    >;

    let mut ovars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut oa: A;
    (ovars, oa) = (::match_deref::match_deref! { match ivars {
        Deref @ metamodelica::ListNode::Nil => {
            let mut a = ia;
            (metamodelica::nil(), a)
        },
        Deref @ metamodelica::ListNode::Cons { head: var, tail: vars } => {
            let mut a = ia;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut var = (*var).clone();
            let mut vars = (*vars).clone();
            ty = getVarType(metamodelica::AsArg::as_arg(&var))?;
            (ty, a) = traverseType(ty, a, r#fn)?;
            var = setVarType(var.clone(), ty);
            (vars, a) = traverseVarTypes(metamodelica::AsArg::as_arg(&vars), a, r#fn)?;
            (metamodelica::cons(var.clone(), vars.clone()), a)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ovars, oa))
}

fn traverseFuncArg<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iargs: &metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    mut ia: A,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, A) -> Result<(metamodelica::Ref<DAE::Type>, A)>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::FuncArg>>, A)> {
    pub type Func<A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, A) -> Result<(metamodelica::Ref<DAE::Type>, A)> + 'static,
    >;

    let mut oargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let mut oa: A;
    (oargs, oa) = (::match_deref::match_deref! { match iargs {
        Deref @ metamodelica::ListNode::Nil => {
            let mut a = ia;
            (metamodelica::nil(), a)
        },
        Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ DAE::FuncArg { .. }, tail: args } => {
            let mut a = ia;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut arg = (*arg).clone();
            let mut args = (*args).clone();
            (ty, a) = traverseType(arg.ty.clone(), a, r#fn)?;
            assign_field!(arg.ty = ty);
            (args, a) = traverseFuncArg(metamodelica::AsArg::as_arg(&args), a, r#fn)?;
            (metamodelica::cons(arg.clone(), args.clone()), a)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oargs, oa))
}

pub fn makeRegularTupleFromMetaTupleOnTrue(
    mut b: bool,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut out: metamodelica::Ref<DAE::Type>;
    out = (::match_deref::match_deref! { match &((b, ty.clone())) {
        (true, Deref @ DAE::Type::T_METATUPLE { types: tys }) => {
            let mut tys = (*tys).clone();
            tys = List::mapMap(tys.clone(), &unboxedType, &fnptr!(boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
            tys = List::map(tys.clone(), &unboxedType)?;
            metamodelica::Ref::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None })
        },
        (false, _) => {
            ty
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(out)
}

pub fn allTuple<'__b>(mut itys: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match itys {
            Deref @ metamodelica::ListNode::Nil => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_TUPLE { .. }, tail: tys } => {
                { itys = tys; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn unboxedFunctionType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inType {
        DAE::Type::T_FUNCTION {
            funcArg: args1,
            funcResultType: ty1,
            functionAttributes,
            path,
        } => {
            let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut args1 = (*args1).clone();
            let mut ty1 = (*ty1).clone();
            tys1 = List::mapMap(
                args1.clone(),
                &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(funcArgType(&__a0))
                },
                &unboxedType,
            )?;
            ty1 = unboxedType(ty1.clone())?;
            args1 = List::threadMap(args1.clone(), tys1, &move |__a0: metamodelica::Ref<DAE::FuncArg>,
                                                                __a1: metamodelica::Ref<DAE::Type>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(setFuncArgType(&__a0, __a1))
            })?;
            metamodelica::Ref::new(DAE::Type::T_FUNCTION {
                funcArg: args1.clone(),
                funcResultType: ty1.clone(),
                functionAttributes: functionAttributes.clone(),
                path: path.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outType)
}

pub fn varHasMetaRecordType(mut var: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match var {
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_METARECORD { .. } }, .. } => true,
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_METARECORD { .. }, .. } => true,
        Deref @ DAE::Var { ty: Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::META_RECORD { path: _ }, .. } }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn optInteger(mut inInt: Option<i32>) -> i32 {
    let mut outInt: i32;
    outInt = (match inInt {
        Some(mut i) => i,
        _ => -1,
    });
    outInt
}

pub fn typeToValue(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut defaultValue: metamodelica::Ref<Values::Value>;
    defaultValue = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("<EMPTY>") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { index: iOpt, path, .. } => {
                    let mut i: i32;
                    i = optInteger(iOpt.clone());
                    Ok(metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: path.clone(), index: i }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: st, varLst: vars, .. } => {
                    let mut comp: metamodelica::List<ArcStr>;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut ordered: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    (ordered, comp) = varsToValues(metamodelica::AsArg::as_arg(&vars))?;
                    path = ClassInfUtil::getStateName(metamodelica::AsArg::as_arg(&st));
                    Ok(metamodelica::Ref::new(Values::Value::RECORD { record_: path.clone(), orderd: ordered.clone(), comp: comp.clone(), index: -1 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    v = typeToValue(metamodelica::AsArg::as_arg(&t))?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t } => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut valueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    v = typeToValue(metamodelica::AsArg::as_arg(&t))?;
                    valueLst = List::fill(v.clone(), i.clone());
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: valueLst.clone(), dimLst: list![i.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut valueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    valueLst = List::map(tys.clone(), &move |__a0: metamodelica::Ref<DAE::Type>| typeToValue(&__a0))?;
                    v = metamodelica::Ref::new(Values::Value::TUPLE { valueLst: valueLst.clone() });
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN { .. } => {
                    Ok(openmodelica_frontend_types::Values::Value::interned_META_FAIL())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Types.typeToValue failed on unhandled Type "))?;
                    s1 = TypesDump::printTypeStr(inType.clone());
                    Debug::traceln(s1.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(defaultValue)
}

pub(crate) fn varsToValues(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Values::Value>>,
    metamodelica::List<ArcStr>,
)> {
    let mut outValuesValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut outExpIdentLst: metamodelica::List<ArcStr>;
    (outValuesValueLst, outExpIdentLst) = 'mc: {
        let __mc_input = &**inVarLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: id, ty: tp, .. }, tail: rest } => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut restVals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut restIds: metamodelica::List<ArcStr>;
                    v = typeToValue(metamodelica::AsArg::as_arg(&tp))?;
                    (restVals, restIds) = varsToValues(metamodelica::AsArg::as_arg(&rest))?;
                    Ok((metamodelica::cons(v.clone(), restVals.clone()), metamodelica::cons(id.clone(), restIds.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Types.varsToValues failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outValuesValueLst, outExpIdentLst))
}

pub fn makeNthDimUnknown(mut ty: &metamodelica::Ref<DAE::Type>, mut dim: i32) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut oty: metamodelica::Ref<DAE::Type>;
    oty = (::match_deref::match_deref! { match &((ty.clone(), dim)) {
        (Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, 1) => {
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty1.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })
        },
        (Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: Deref @ metamodelica::ListNode::Cons { head: ad, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
            let mut ty1 = (*ty1).clone();
            ty1 = makeNthDimUnknown(metamodelica::AsArg::as_arg(&ty1), dim - 1)?;
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty1.clone(), dims: list![ad.clone()] })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(oty)
}

pub fn arraySuperType(
    mut ity1: metamodelica::Ref<DAE::Type>,
    mut info: &SourceInfo,
    mut ity2: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = 'mc: {
        let __mc_input = (ity1, ity2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, ty2) => {
                    let mut ty1 = (*ty1).clone();
                    let true = (isInteger(&(arrayElementType(metamodelica::AsArg::as_arg(&ty1))))) else { return Err("pattern mismatch") };
                    let true = (isReal(&(arrayElementType(metamodelica::AsArg::as_arg(&ty2))))) else { return Err("pattern mismatch") };
                    (ty1, _) = traverseType(ty1.clone(), -1, &fnptr!(replaceIntegerTypeWithReal, metamodelica::Ref<DAE::Type>, i32))?;
                    let true = (subtype(ty1.clone(), ty2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(ty1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, ty2) => {
                    let mut ty2 = (*ty2).clone();
                    let true = (isInteger(&(arrayElementType(metamodelica::AsArg::as_arg(&ty2))))) else { return Err("pattern mismatch") };
                    let true = (isReal(&(arrayElementType(metamodelica::AsArg::as_arg(&ty1))))) else { return Err("pattern mismatch") };
                    (ty2, _) = traverseType(ty2.clone(), -1, &fnptr!(replaceIntegerTypeWithReal, metamodelica::Ref<DAE::Type>, i32))?;
                    let true = (subtype(ty1.clone(), ty2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(ty1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, ty2) => {
                    let true = (subtype(ty1.clone(), ty2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(ty1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, ty2) => {
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    str1 = TypesDump::unparseType(ty1.clone())?;
                    str2 = TypesDump::unparseType(ty2.clone())?;
                    typeErrorSanityCheck(str1.clone(), &str2, info)?;
                    Error::addSourceMessage(&(Error::ARRAY_TYPE_MISMATCH.clone()), list![str1.clone(), str2.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(ty)
}

fn replaceIntegerTypeWithReal(mut ty: Type, mut dummy: i32) -> (Type, i32) {
    let mut oty: Type;
    let mut odummy: i32 = dummy;
    oty = (match &*ty {
        DAE::Type::T_INTEGER { .. } => DAE::T_REAL_DEFAULT().clone(),
        _ => ty,
    });
    (oty, odummy)
}

pub fn isZeroLengthArray(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<bool> {
    let mut res: bool;
    res = (match &**ty {
        DAE::Type::T_ARRAY { dims, .. } => {
            res = List::fold(
                dims,
                &move |__a0: metamodelica::Ref<DAE::Dimension>, __a1: bool| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isZeroDim(&__a0, __a1))
                },
                false,
            )?;
            res
        }
        _ => false,
    });
    Ok(res)
}

fn isZeroDim(mut dim: &metamodelica::Ref<DAE::Dimension>, mut acc: bool) -> bool {
    let mut res: bool;
    res = (match &**dim {
        DAE::Dimension::DIM_INTEGER { integer: 0 } => true,
        DAE::Dimension::DIM_ENUM { size: 0, .. } => true,
        _ => acc,
    });
    res
}

pub fn variabilityToConst(mut variability: SCode::Variability) -> DAE::Const {
    let mut r#const: DAE::Const;
    r#const = (match variability {
        SCode::Variability::VAR { .. } => openmodelica_frontend_types::DAE::Const::C_VAR,
        SCode::Variability::DISCRETE { .. } => openmodelica_frontend_types::DAE::Const::C_VAR,
        SCode::Variability::PARAM { .. } => openmodelica_frontend_types::DAE::Const::C_PARAM,
        SCode::Variability::CONST { .. } => openmodelica_frontend_types::DAE::Const::C_CONST,
    });
    r#const
}

pub(crate) fn varKindToConst(mut varKind: DAE::VarKind) -> DAE::Const {
    let mut r#const: DAE::Const;
    r#const = (match varKind {
        DAE::VarKind::VARIABLE { .. } => openmodelica_frontend_types::DAE::Const::C_VAR,
        DAE::VarKind::DISCRETE { .. } => openmodelica_frontend_types::DAE::Const::C_VAR,
        DAE::VarKind::PARAM { .. } => openmodelica_frontend_types::DAE::Const::C_PARAM,
        DAE::VarKind::CONST { .. } => openmodelica_frontend_types::DAE::Const::C_CONST,
    });
    r#const
}

pub fn isValidFunctionVarType<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_COMPLEX {
                complexClassType: state,
                ..
            } => return isValidFunctionVarState(state),
            DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. } => {
                inType = ty;
                continue '__tco;
            }
            _ => return true,
        }
    }
}

fn isValidFunctionVarState(mut inState: &ClassInf::State) -> bool {
    let mut outIsValid: bool;
    outIsValid = (match inState.clone() {
        ClassInf::State::MODEL { .. } => false,
        ClassInf::State::BLOCK { .. } => false,
        ClassInf::State::CONNECTOR { .. } => false,
        ClassInf::State::OPTIMIZATION { .. } => false,
        ClassInf::State::PACKAGE { .. } => false,
        _ => true,
    });
    outIsValid
}

fn makeDummyExpFromType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match inType {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            getNthEnumLiteral(inType, 1)?
        },
        Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut idim: i32;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut ety: metamodelica::Ref<DAE::Type>;
            idim = Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?;
            exp = makeDummyExpFromType(ty)?;
            ety = Expression::r#typeof(exp.clone())?;
            ety = Expression::liftArrayLeft(ety, dim.clone());
            expl = List::fill(exp, idim);
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ety, scalar: true, array: expl })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn printExpTypeStr(mut iet: &metamodelica::Ref<DAE::Type>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = TypesDump::printTypeStr(expTypetoTypesType(iet));
    r#str
}

pub fn isUnknownType(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**inType {
        DAE::Type::T_UNKNOWN { .. } => true,
        DAE::Type::T_ANYTYPE { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isOverdeterminedType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<bool> {
    let mut outIsOverdetermined: bool;
    outIsOverdetermined = (::match_deref::match_deref! { match inType {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: cct, equalityConstraint: Some(_), .. } => {
            ClassInfUtil::isTypeOrRecord(cct)
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => {
            true
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outIsOverdetermined)
}

pub fn hasMetaArray(mut ty: metamodelica::Ref<DAE::Type>) -> Result<bool> {
    let mut b: bool;
    (_, b) = traverseType(ty, false, &fnptr!(hasMetaArrayWork, metamodelica::Ref<DAE::Type>, bool))?;
    Ok(b)
}

fn hasMetaArrayWork(mut ty: Type, mut b: bool) -> (Type, bool) {
    let mut oty: Type = ty.clone();
    let mut ob: bool = b;
    if !(b) {
        ob = (match &*ty {
            DAE::Type::T_METAARRAY { .. } => true,
            _ => false,
        });
    }
    (oty, ob)
}

fn classTypeEqualIfRecord(mut st1: &ClassInf::State, mut st2: &ClassInf::State) -> bool {
    let mut b: bool;
    b = (match (st1.clone(), st2.clone()) {
        (ClassInf::State::RECORD { path: ref p1 }, ClassInf::State::RECORD { path: ref p2 }) => {
            AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))
        }
        _ => true,
    });
    b
}

pub(crate) fn ifExpMakeDimsUnknown(
    mut ty1: &metamodelica::Ref<DAE::Type>,
    mut ty2: &metamodelica::Ref<DAE::Type>,
) -> (metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>) {
    let mut oty1: metamodelica::Ref<DAE::Type>;
    let mut oty2: metamodelica::Ref<DAE::Type>;
    (oty1, oty2) = (::match_deref::match_deref! { match (ty1, ty2) {
        (Deref @ DAE::Type::T_ARRAY { ty: inner1, dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: inner2, dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            (oty1, oty2) = ifExpMakeDimsUnknown(inner1, inner2);
            (metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inner1.clone(), dims: metamodelica::cons(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(), metamodelica::nil()) }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inner2.clone(), dims: metamodelica::cons(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(), metamodelica::nil()) }))
        },
        (Deref @ DAE::Type::T_ARRAY { ty: inner1, dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: inner2, dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            (oty1, oty2) = ifExpMakeDimsUnknown(inner1, inner2);
            (metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inner1.clone(), dims: metamodelica::cons(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(), metamodelica::nil()) }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inner2.clone(), dims: metamodelica::cons(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(), metamodelica::nil()) }))
        },
        (Deref @ DAE::Type::T_ARRAY { ty: inner1, dims: Deref @ metamodelica::ListNode::Cons { head: d1, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::Type::T_ARRAY { ty: inner2, dims: Deref @ metamodelica::ListNode::Cons { head: d2, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            (oty1, oty2) = ifExpMakeDimsUnknown(inner1, inner2);
            (metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inner1.clone(), dims: list![d1.clone()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inner2.clone(), dims: list![d2.clone()] }))
        },
        _ => {
            (ty1.clone(), ty2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty1, oty2)
}

pub(crate) fn isFixedWithNoBinding(
    mut inTy: &metamodelica::Ref<DAE::Type>,
    mut inVariability: SCode::Variability,
) -> bool {
    let mut outFixed: bool;
    outFixed = 'mc: {
        let __mc_input = &**inTy;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: bool;
                    b = getFixedVarAttribute(inTy)?;
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { varLst: vl, .. } => {
                    let true = (allHaveBindings(metamodelica::AsArg::as_arg(&vl))?) else { return Err("pattern mismatch") };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: bool;
                    b = listMember(inVariability, list![openmodelica_frontend_types::SCode::Variability::PARAM, openmodelica_frontend_types::SCode::Variability::CONST]);
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outFixed
}

pub fn allHaveBindings(mut inVars: &metamodelica::List<metamodelica::Ref<DAE::Var>>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inVars {
        Deref @ metamodelica::ListNode::Nil => {
            true
        },
        Deref @ metamodelica::ListNode::Cons { head: v, tail: _ } if (!(hasBinding(metamodelica::AsArg::as_arg(&v)))) => {
            false
        },
        Deref @ metamodelica::ListNode::Cons { head: v, tail: rest } => {
            let true = (hasBinding(metamodelica::AsArg::as_arg(&v))) else { return Err("pattern mismatch") };
            let true = (allHaveBindings(rest)?) else { return Err("pattern mismatch") };
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn hasBinding(mut inVar: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inVar {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::UNBOUND { .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn typeErrorSanityCheck(mut inType1: ArcStr, mut inType2: &ArcStr, mut inInfo: &SourceInfo) -> Result<()> {
    if stringEq(&inType1, &inType2) {
        Error::addSourceMessage(&(Error::ERRONEOUS_TYPE_ERROR.clone()), list![inType1], inInfo)?;
        return Err("fail");
    }
    Ok(())
}

pub fn dimNotFixed(mut dim: &metamodelica::Ref<DAE::Dimension>) -> bool {
    let mut b: bool;
    b = (match &**dim {
        DAE::Dimension::DIM_UNKNOWN { .. } => true,
        DAE::Dimension::DIM_EXP { .. } => true,
        _ => false,
    });
    b
}

pub fn isArrayWithUnknownDimension(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**ty {
        DAE::Type::T_ARRAY { .. } => {
            ({
                let mut __acc: Option<bool> = None;
                for mut d in (TypesDump::getDimensions(ty)).into_iter().cloned() {
                    let __x = (match &*d.clone() {
                        DAE::Dimension::DIM_UNKNOWN { .. } => true,
                        _ => false,
                    });
                    __acc = Some(match __acc {
                        None => __x,
                        Some(__cur) => {
                            if __x > __cur {
                                __x
                            } else {
                                __cur
                            }
                        }
                    });
                }
                __acc.unwrap_or(false)
            })
        }
        _ => false,
    });
    b
}

pub fn setTypeVars(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut inVars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut ty: metamodelica::Ref<DAE::Type> = ty;
    ty = (match &*ty {
        DAE::Type::T_REAL { .. } => {
            assign_variant_field!(ty => DAE::Type::T_REAL; varLst = inVars.clone());
            ty
        }
        DAE::Type::T_INTEGER { .. } => {
            assign_variant_field!(ty => DAE::Type::T_INTEGER; varLst = inVars.clone());
            ty
        }
        DAE::Type::T_STRING { .. } => {
            assign_variant_field!(ty => DAE::Type::T_STRING; varLst = inVars.clone());
            ty
        }
        DAE::Type::T_BOOL { .. } => {
            assign_variant_field!(ty => DAE::Type::T_BOOL; varLst = inVars.clone());
            ty
        }
        DAE::Type::T_CLOCK { .. } => {
            assign_variant_field!(ty => DAE::Type::T_CLOCK; varLst = inVars.clone());
            ty
        }
        DAE::Type::T_ENUMERATION { .. } => {
            assign_variant_field!(ty => DAE::Type::T_ENUMERATION; attributeLst = inVars.clone());
            ty
        }
        DAE::Type::T_ARRAY { ty: __ty_ty, .. } => {
            assign_variant_field!(ty => DAE::Type::T_ARRAY; ty = setTypeVars(__ty_ty.clone(), inVars)?);
            ty
        }
        DAE::Type::T_SUBTYPE_BASIC {
            complexType: __ty_complexType,
            ..
        } => {
            assign_variant_field!(ty => DAE::Type::T_SUBTYPE_BASIC; complexType = setTypeVars(__ty_complexType.clone(), inVars)?);
            ty
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(ty)
}

pub fn isEmptyOrNoRetcall(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match ty {
        Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Nil, .. } => true,
        Deref @ DAE::Type::T_METATUPLE { types: Deref @ metamodelica::ListNode::Nil } => true,
        Deref @ DAE::Type::T_NORETCALL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn typeConvertIntToEnumCheck(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut expected: &metamodelica::Ref<DAE::Type>,
) -> Result<bool> {
    let mut conversionOK: bool;
    conversionOK = 'mc: {
        let __mc_input = (&**exp, &**expected);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: oi }, Deref @ DAE::Type::T_ENUMERATION { path: tp, names: l, .. }) => {
                    let mut pathStr: ArcStr;
                    let mut intStr: ArcStr;
                    let mut enumConst: ArcStr;
                    let true = (1 <= oi.clone() && oi.clone() <= ((l).len() as i32)) else { return Err("pattern mismatch") };
                    pathStr = AbsynUtil::pathString(tp.clone(), literal!("."), true, false)?;
                    intStr = intString(oi.clone());
                    enumConst = (l).get(oi.clone())?;
                    Error::addMessage(Error::INTEGER_ENUMERATION_CONVERSION_WARNING.clone(), list![intStr.clone(), pathStr.clone(), enumConst.clone()])?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: oi }, Deref @ DAE::Type::T_ENUMERATION { path: tp, names: l, .. }) => {
                    let mut pathStr: ArcStr;
                    let mut intStr: ArcStr;
                    let mut lengthStr: ArcStr;
                    pathStr = AbsynUtil::pathString(tp.clone(), literal!("."), true, false)?;
                    let false = (stringEq(&pathStr, &(literal!("")))) else { return Err("pattern mismatch") };
                    intStr = intString(oi.clone());
                    lengthStr = intString(((l).len() as i32));
                    Error::addMessage(Error::INTEGER_ENUMERATION_OUT_OF_RANGE.clone(), list![pathStr.clone(), intStr.clone(), lengthStr.clone()])?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: oi }, Deref @ DAE::Type::T_ENUMERATION { path: tp, .. }) => {
                    let mut pathStr: ArcStr;
                    let mut intStr: ArcStr;
                    pathStr = AbsynUtil::pathString(tp.clone(), literal!("."), true, false)?;
                    let true = (stringEq(&pathStr, &(literal!("")))) else { return Err("pattern mismatch") };
                    intStr = intString(oi.clone());
                    Error::addMessage(Error::INTEGER_TO_UNKNOWN_ENUMERATION.clone(), list![intStr.clone()])?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(conversionOK)
}

pub fn findVarIndex(mut id: ArcStr, mut vars: &metamodelica::List<metamodelica::Ref<DAE::Var>>) -> Result<i32> {
    let mut index: i32;
    index = List::position1OnTrue(
        vars,
        &move |__a0: metamodelica::Ref<DAE::Var>, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(selectVar(&__a0, &__a1))
        },
        id,
    )? - 1;
    Ok(index)
}

fn selectVar(mut var: &metamodelica::Ref<DAE::Var>, mut id: &ArcStr) -> bool {
    let mut b: bool;
    b = (match &**var {
        DAE::Var { name: id1, .. } => stringEq(&id, &id1),
        _ => false,
    });
    b
}

pub(crate) fn getUniontypeIfMetarecord(mut inTy: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = (match &*inTy.clone() {
        DAE::Type::T_METARECORD {
            utPath: p,
            knownSingleton: b,
            typeVars: __inTy_typeVars,
            ..
        } => metamodelica::Ref::new(DAE::Type::T_METAUNIONTYPE {
            paths: metamodelica::nil(),
            typeVars: __inTy_typeVars.clone(),
            knownSingleton: b.clone(),
            singletonType: if (b.clone()) {
                metamodelica::Ref::new(DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: inTy })
            } else {
                openmodelica_frontend_types::DAE::EvaluateSingletonType::interned_NOT_SINGLETON()
            },
            path: p.clone(),
        }),
        _ => inTy,
    });
    ty
}

pub fn getUniontypeIfMetarecordReplaceAllSubtypes(
    mut inTy: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    (ty, _) = traverseType(
        inTy,
        1,
        &fnptr!(getUniontypeIfMetarecordTraverse, metamodelica::Ref<DAE::Type>, i32),
    )?;
    Ok(ty)
}

fn getUniontypeIfMetarecordTraverse(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut dummy: i32,
) -> (metamodelica::Ref<DAE::Type>, i32) {
    let mut oty: metamodelica::Ref<DAE::Type>;
    let mut odummy: i32 = dummy;
    oty = (match &*ty.clone() {
        DAE::Type::T_METARECORD {
            knownSingleton: __ty_knownSingleton,
            typeVars: __ty_typeVars,
            utPath: __ty_utPath,
            ..
        } => metamodelica::Ref::new(DAE::Type::T_METAUNIONTYPE {
            paths: metamodelica::nil(),
            typeVars: __ty_typeVars.clone(),
            knownSingleton: __ty_knownSingleton.clone(),
            singletonType: if (__ty_knownSingleton.clone()) {
                metamodelica::Ref::new(DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: ty })
            } else {
                openmodelica_frontend_types::DAE::EvaluateSingletonType::interned_NOT_SINGLETON()
            },
            path: __ty_utPath.clone(),
        }),
        _ => ty,
    });
    (oty, odummy)
}

fn isBuiltin(mut a: &DAE::FunctionBuiltin) -> bool {
    let mut b: bool;
    b = (match a.clone() {
        DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN { .. } => false,
        _ => true,
    });
    b
}

pub fn makeCallAttr(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut attr: &DAE::FunctionAttributes,
) -> metamodelica::Ref<DAE::CallAttributes> {
    let mut callAttr: metamodelica::Ref<DAE::CallAttributes>;
    let mut isImpure: bool;
    let mut isT: bool;
    let mut isB: bool;
    isT = isTuple(&ty);
    isB = isBuiltin(&attr.isBuiltin);
    isImpure = attr.purity.clone() == DAE::Purity::IMPURE.clone();
    callAttr = metamodelica::Ref::new(DAE::CallAttributes {
        ty: ty,
        tuple_: isT,
        builtin: isB,
        isImpure: isImpure,
        isFunctionPointerCall: false,
        inlineType: attr.inline.clone(),
        tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
        noReturn: DAE::NoReturn::RETURNS.clone(),
    });
    callAttr
}

pub(crate) fn builtinName(mut isbuiltin: &DAE::FunctionBuiltin) -> Option<ArcStr> {
    let mut name: Option<ArcStr>;
    name = (match isbuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_BUILTIN { .. } => {
            var_field!(isbuiltin.name, DAE::FunctionBuiltin::FUNCTION_BUILTIN).clone()
        }
        _ => None,
    });
    name
}

pub fn getFuncArg(
    mut ty: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::FuncArg>>> {
    let mut args: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let __pa0 = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ DAE::Type::T_FUNCTION { funcArg: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    args = metamodelica::Own::own(__pa0);
    Ok(args)
}

pub fn isArray1D(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**inType {
        DAE::Type::T_ARRAY { ty, .. } => !(arrayType(ty)),
        _ => false,
    });
    b
}

pub fn isArray2D(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inType {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, .. }, .. } => {
            !(arrayType(metamodelica::AsArg::as_arg(&ty)))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn funcArgName(mut arg: &metamodelica::Ref<DAE::FuncArg>) -> ArcStr {
    let mut name: ArcStr;
    let __arc1 = &(*arg);
    let DAE::FUNCARG { name: __pa0, .. } = &**__arc1;
    name = metamodelica::Own::own(__pa0);
    name
}

pub fn funcArgType(mut arg: &metamodelica::Ref<DAE::FuncArg>) -> metamodelica::Ref<DAE::Type> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __arc1 = &(*arg);
    let DAE::FUNCARG { ty: __pa0, .. } = &**__arc1;
    ty = metamodelica::Own::own(__pa0);
    ty
}

pub fn funcArgDefaultBinding(mut arg: &metamodelica::Ref<DAE::FuncArg>) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut defaultBinding: Option<metamodelica::Ref<DAE::Exp>>;
    let __arc1 = &(*arg);
    let DAE::FUNCARG {
        defaultBinding: __pa0, ..
    } = &**__arc1;
    defaultBinding = metamodelica::Own::own(__pa0);
    defaultBinding
}

pub fn setFuncArgType(
    mut arg: &metamodelica::Ref<DAE::FuncArg>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::FuncArg> {
    let mut outArg: metamodelica::Ref<DAE::FuncArg>;
    let mut name: ArcStr;
    let mut r#const: DAE::Const;
    let mut par: DAE::VarParallelism;
    let mut defaultBinding: Option<metamodelica::Ref<DAE::Exp>>;
    let __arc4 = &(*arg);
    let DAE::FUNCARG {
        name: __pa0,
        ty: _,
        r#const: __pa1,
        par: __pa2,
        defaultBinding: __pa3,
    } = &**__arc4;
    name = metamodelica::Own::own(__pa0);
    r#const = metamodelica::Own::own(__pa1);
    par = metamodelica::Own::own(__pa2);
    defaultBinding = metamodelica::Own::own(__pa3);
    outArg = metamodelica::Ref::new(DAE::FuncArg {
        name: name,
        ty: ty,
        r#const: r#const,
        par: par,
        defaultBinding: defaultBinding,
    });
    outArg
}

pub(crate) fn setFuncArgName(
    mut arg: &metamodelica::Ref<DAE::FuncArg>,
    mut name: ArcStr,
) -> metamodelica::Ref<DAE::FuncArg> {
    let mut outArg: metamodelica::Ref<DAE::FuncArg>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut r#const: DAE::Const;
    let mut par: DAE::VarParallelism;
    let mut defaultBinding: Option<metamodelica::Ref<DAE::Exp>>;
    let __arc4 = &(*arg);
    let DAE::FUNCARG {
        name: _,
        ty: __pa0,
        r#const: __pa1,
        par: __pa2,
        defaultBinding: __pa3,
    } = &**__arc4;
    ty = metamodelica::Own::own(__pa0);
    r#const = metamodelica::Own::own(__pa1);
    par = metamodelica::Own::own(__pa2);
    defaultBinding = metamodelica::Own::own(__pa3);
    outArg = metamodelica::Ref::new(DAE::FuncArg {
        name: name,
        ty: ty,
        r#const: r#const,
        par: par,
        defaultBinding: defaultBinding,
    });
    outArg
}

pub(crate) fn clearDefaultBinding(mut arg: &metamodelica::Ref<DAE::FuncArg>) -> metamodelica::Ref<DAE::FuncArg> {
    let mut outArg: metamodelica::Ref<DAE::FuncArg>;
    let mut name: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut r#const: DAE::Const;
    let mut par: DAE::VarParallelism;
    let __arc4 = &(*arg);
    let DAE::FUNCARG {
        name: __pa0,
        ty: __pa1,
        r#const: __pa2,
        par: __pa3,
        defaultBinding: _,
    } = &**__arc4;
    name = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    r#const = metamodelica::Own::own(__pa2);
    par = metamodelica::Own::own(__pa3);
    outArg = metamodelica::Ref::new(DAE::FuncArg {
        name: name,
        ty: ty,
        r#const: r#const,
        par: par,
        defaultBinding: None,
    });
    outArg
}

pub fn makeDefaultFuncArg(mut name: ArcStr, mut ty: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::FuncArg> {
    let mut arg: metamodelica::Ref<DAE::FuncArg>;
    arg = metamodelica::Ref::new(DAE::FuncArg {
        name: name,
        ty: ty,
        r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        defaultBinding: None,
    });
    arg
}

pub fn setIsFunctionPointer(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut dummy: i32,
) -> (metamodelica::Ref<DAE::Type>, i32) {
    let mut oty: metamodelica::Ref<DAE::Type> = ty;
    let mut odummy: i32 = dummy;
    oty = (match &*oty {
        DAE::Type::T_FUNCTION {
            functionAttributes:
                attr @ DAE::FunctionAttributes {
                    isFunctionPointer: false,
                    ..
                },
            ..
        } => {
            let mut attr = (*attr).clone();
            attr.isFunctionPointer = true;
            assign_variant_field!(oty => DAE::Type::T_FUNCTION; functionAttributes = attr.clone());
            oty
        }
        _ => oty,
    });
    (oty, odummy)
}

pub(crate) fn isFunctionReferenceVar(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**ty {
        DAE::Type::T_FUNCTION_REFERENCE_VAR { .. } => true,
        _ => false,
    });
    b
}

pub fn isFunctionPointer(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outIsFunPtr: bool;
    outIsFunPtr = (match &**inType {
        DAE::Type::T_FUNCTION {
            functionAttributes:
                DAE::FunctionAttributes {
                    isFunctionPointer: true,
                    ..
                },
            ..
        } => true,
        _ => false,
    });
    outIsFunPtr
}

pub fn filterRecordComponents(
    mut inRecordVars: metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut outRecordVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    outRecordVars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
        for mut v in (inRecordVars).into_iter().cloned() {
            let __x = (match &*v.clone() {
                DAE::Var { .. } => {
                    if !(allowedInRecord(&(v.ty.clone()))) {
                        Error::addSourceMessage(
                            &(Error::ILLEGAL_RECORD_COMPONENT.clone()),
                            list![TypesDump::unparseVar(&(v.clone()))?],
                            inInfo,
                        )?;
                        return Err("fail");
                    }
                    v.clone()
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outRecordVars)
}

pub(crate) fn allowedInRecord(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut yes: bool;
    yes = 'mc: {
        let __mc_input = &**ty;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    t = arrayElementType(ty);
                    let true = (basicType(&t) || isRecord(&t) || extendsBasicType(&t)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    yes
}

pub fn lookupIndexInMetaRecord(
    mut vars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut name: ArcStr,
) -> Result<i32> {
    let mut index: i32;
    index = List::position1OnTrue(
        vars,
        &move |__a0: metamodelica::Ref<DAE::Var>, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(DAEUtil::typeVarIdentEqual(&__a0, &__a1))
        },
        name,
    )?;
    Ok(index)
}

pub fn checkEnumDuplicateLiterals(mut names: metamodelica::List<ArcStr>, mut info: &SourceInfo) -> Result<()> {
    let mut sortedNames: metamodelica::List<ArcStr>;
    sortedNames = List::sort(
        names.clone(),
        (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
    )?;
    if !(List::sortedListAllUnique(sortedNames.clone(), &fnptr!(stringEq, ArcStr, ArcStr))?) {
        Error::addSourceMessage(
            &(Error::ENUM_DUPLICATES.clone()),
            list![
                stringDelimitList(
                    List::sortedUniqueOnlyDuplicates(sortedNames, &fnptr!(stringEq, ArcStr, ArcStr))?,
                    literal!(",")
                ),
                stringDelimitList(names, literal!(","))
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

pub fn checkTypeCompat(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inType2: &metamodelica::Ref<DAE::Type>,
    mut inAllowUnknown: bool,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    bool,
)> {
    let mut outExp1: metamodelica::Ref<DAE::Exp> = inExp1.clone();
    let mut outExp2: metamodelica::Ref<DAE::Exp> = inExp2.clone();
    let mut outCompatType: metamodelica::Ref<DAE::Type>;
    let mut outCompatible: bool = true;
    let mut ty1: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    if referenceEq(&*(&*inType1), &*(&**inType2)) {
        outCompatType = inType1;
        return Ok((outExp1, outExp2, outCompatType, outCompatible));
    }
    if metamodelica::valueConstructor((&*&*inType1))? != metamodelica::valueConstructor((&*&**inType2))? {
        if extendsBasicType(&inType1) || extendsBasicType(inType2) {
            ty1 = derivedBasicType(&inType1);
            ty2 = derivedBasicType(inType2);
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1, ty1, inExp2, &ty2, false)?;
        } else {
            (outExp1, outExp2, outCompatType, outCompatible) =
                checkTypeCompat_cast(inExp1, &inType1, inExp2, inType2, inAllowUnknown)?;
        }
        return Ok((outExp1, outExp2, outCompatType, outCompatible));
    }
    outCompatType = (match &*inType1 {
        DAE::Type::T_INTEGER { .. } => DAE::T_INTEGER_DEFAULT().clone(),
        DAE::Type::T_REAL { .. } => DAE::T_REAL_DEFAULT().clone(),
        DAE::Type::T_STRING { .. } => DAE::T_STRING_DEFAULT().clone(),
        DAE::Type::T_BOOL { .. } => DAE::T_BOOL_DEFAULT().clone(),
        DAE::Type::T_CLOCK { .. } => DAE::T_CLOCK_DEFAULT().clone(),
        DAE::Type::T_SUBTYPE_BASIC {
            complexType: __inType1_complexType,
            ..
        } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            (outExp1, outExp2, outCompatType, outCompatible) =
                checkTypeCompat(inExp1, __inType1_complexType.clone(), inExp2, &ty, false)?;
            outCompatType
        }
        DAE::Type::T_ENUMERATION {
            names: __inType1_names, ..
        } => {
            let mut names: metamodelica::List<ArcStr>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_ENUMERATION { names: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            names = metamodelica::Own::own(__pa0);
            outCompatible = List::isEqualOnTrue(__inType1_names.clone(), names, &fnptr!(stringEq, ArcStr, ArcStr))?;
            inType1
        }
        DAE::Type::T_ARRAY { .. } => {
            let mut dims1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut dims2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut ety1: metamodelica::Ref<DAE::Type>;
            let mut ety2: metamodelica::Ref<DAE::Type>;
            ety1 = arrayElementType(&inType1);
            ety2 = arrayElementType(inType2);
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1, ety1, inExp2, &ety2, false)?;
            if outCompatible {
                dims1 = TypesDump::getDimensions(&inType1);
                dims2 = TypesDump::getDimensions(inType2);
                if ((dims1).len() as i32) == ((dims2).len() as i32) {
                    dims1 = ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
                        let __thr_src0 = dims1;
                        let mut __thr_it0 = (&__thr_src0).into_iter();
                        let __thr_src1 = dims2;
                        let mut __thr_it1 = (&__thr_src1).into_iter();
                        loop {
                            match (__thr_it0.next(), __thr_it1.next()) {
                                (Some(dim1), Some(dim2)) => {
                                    let __x =
                                        if (Expression::dimensionsKnownAndEqual(&(dim1.clone()), &(dim2.clone()))?) {
                                            dim1.clone()
                                        } else {
                                            openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()
                                        };
                                    __acc = cons(__x, __acc);
                                }
                                (None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                        }
                        __acc.reverse()
                    });
                    outCompatType = liftArrayListDims(outCompatType, dims1);
                } else {
                    outCompatible = false;
                }
            }
            outCompatType
        }
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { .. },
            varLst: __inType1_varLst,
            ..
        } => {
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            vars = metamodelica::Own::own(__pa0);
            outCompatible = List::isEqualOnTrue(
                __inType1_varLst.clone(),
                vars,
                &move |__a0: metamodelica::Ref<DAE::Var>,
                       __a1: metamodelica::Ref<DAE::Var>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(varEqualName(&__a0, &__a1))
                },
            )?;
            inType1
        }
        DAE::Type::T_FUNCTION {
            funcArg: __inType1_funcArg,
            funcResultType: __inType1_funcResultType,
            ..
        } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut args: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_FUNCTION { funcResultType: __pa0, funcArg: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            args = metamodelica::Own::own(__pa1);
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(
                inExp1.clone(),
                __inType1_funcResultType.clone(),
                inExp2.clone(),
                &ty,
                false,
            )?;
            if outCompatible {
                tys = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                    for mut arg in (__inType1_funcArg.clone()).into_iter().cloned() {
                        let __x = funcArgType(&(arg.clone()));
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                tys2 = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                    for mut arg in (args).into_iter().cloned() {
                        let __x = funcArgType(&(arg.clone()));
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                (_, outCompatible) = checkTypeCompatList(inExp1, &tys, inExp2, tys2)?;
            }
            inType1
        }
        DAE::Type::T_TUPLE {
            names: __inType1_names,
            types: __inType1_types,
        } => {
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_TUPLE { types: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tys = metamodelica::Own::own(__pa0);
            (tys, outCompatible) =
                checkTypeCompatList(inExp1, metamodelica::AsArg::as_arg(&__inType1_types), inExp2, tys)?;
            metamodelica::Ref::new(DAE::Type::T_TUPLE {
                types: tys,
                names: __inType1_names.clone(),
            })
        }
        DAE::Type::T_METALIST { ty: __inType1_ty } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_METALIST { ty: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            (outExp1, outExp2, outCompatType, outCompatible) =
                checkTypeCompat(inExp1, __inType1_ty.clone(), inExp2, &ty, true)?;
            metamodelica::Ref::new(DAE::Type::T_METALIST { ty: outCompatType })
        }
        DAE::Type::T_METAARRAY { ty: __inType1_ty } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_METAARRAY { ty: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            (outExp1, outExp2, outCompatType, outCompatible) =
                checkTypeCompat(inExp1, __inType1_ty.clone(), inExp2, &ty, true)?;
            metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: outCompatType })
        }
        DAE::Type::T_METAOPTION { ty: __inType1_ty } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_METAOPTION { ty: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            (outExp1, outExp2, outCompatType, outCompatible) =
                checkTypeCompat(inExp1, __inType1_ty.clone(), inExp2, &ty, true)?;
            metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: outCompatType })
        }
        DAE::Type::T_METATUPLE { types: __inType1_types } => {
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_METATUPLE { types: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tys = metamodelica::Own::own(__pa0);
            (tys, outCompatible) =
                checkTypeCompatList(inExp1, metamodelica::AsArg::as_arg(&__inType1_types), inExp2, tys)?;
            metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys })
        }
        DAE::Type::T_METABOXED { ty: __inType1_ty } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_METABOXED { ty: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            (outExp1, outExp2, outCompatType, outCompatible) =
                checkTypeCompat(inExp1, __inType1_ty.clone(), inExp2, &ty, false)?;
            metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: outCompatType })
        }
        DAE::Type::T_METAPOLYMORPHIC { name: __inType1_name } => {
            let mut name: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { name: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            outCompatible = metamodelica::stringEq(&__inType1_name, &name);
            inType1
        }
        DAE::Type::T_METAUNIONTYPE { path: p1, .. } => {
            let mut p2: metamodelica::Ref<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_METAUNIONTYPE { path: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p2 = metamodelica::Own::own(__pa0);
            outCompatible = AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), &p2);
            inType1
        }
        DAE::Type::T_METARECORD { utPath: p1, .. } => {
            let mut p2: metamodelica::Ref<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_METARECORD { utPath: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p2 = metamodelica::Own::own(__pa0);
            outCompatible = AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), &p2);
            inType1
        }
        DAE::Type::T_FUNCTION_REFERENCE_VAR {
            functionType: __inType1_functionType,
        } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let __pa0 = ::match_deref::match_deref! { match &((*inType2)) {
                Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            (outExp1, outExp2, outCompatType, outCompatible) =
                checkTypeCompat(inExp1, __inType1_functionType.clone(), inExp2, &ty, false)?;
            metamodelica::Ref::new(DAE::Type::T_FUNCTION_REFERENCE_VAR {
                functionType: outCompatType,
            })
        }
        _ => {
            outCompatible = false;
            DAE::T_UNKNOWN_DEFAULT().clone()
        }
    });
    Ok((outExp1, outExp2, outCompatType, outCompatible))
}

fn checkTypeCompatList(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inTypes1: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inTypes2: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Type>>, bool)> {
    let mut outCompatibleTypes: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
    let mut outCompatible: bool = true;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut rest_ty2: metamodelica::List<metamodelica::Ref<DAE::Type>> = inTypes2.clone();
    let mut compat: bool;
    if ((inTypes1).len() as i32) != ((inTypes2).len() as i32) {
        outCompatible = false;
        return Ok((outCompatibleTypes, outCompatible));
    }
    for mut ty1 in &**inTypes1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_ty2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty2 = metamodelica::Own::own(__pa0);
        rest_ty2 = metamodelica::Own::own(__pa1);
        (_, _, ty2, compat) = checkTypeCompat(inExp1.clone(), ty1.clone(), inExp2.clone(), &ty2, false)?;
        if !(compat) {
            outCompatible = false;
            return Ok((outCompatibleTypes, outCompatible));
        }
        outCompatibleTypes = metamodelica::cons(ty2, outCompatibleTypes);
    }
    outCompatibleTypes = outCompatibleTypes.reverse();
    Ok((outCompatibleTypes, outCompatible))
}

fn checkTypeCompat_cast(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inType1: &metamodelica::Ref<DAE::Type>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inType2: &metamodelica::Ref<DAE::Type>,
    mut inAllowUnknown: bool,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    bool,
)> {
    let mut outExp1: metamodelica::Ref<DAE::Exp> = inExp1.clone();
    let mut outExp2: metamodelica::Ref<DAE::Exp> = inExp2.clone();
    let mut outCompatType: metamodelica::Ref<DAE::Type>;
    let mut outCompatible: bool = true;
    let mut ty1: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    ty1 = derivedBasicType(inType1);
    ty2 = derivedBasicType(inType2);
    outCompatType = (::match_deref::match_deref! { match &((ty1.clone(), ty2.clone())) {
        (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            outExp2 = Expression::typeCastElements(inExp2, &(DAE::T_REAL_DEFAULT().clone()))?;
            DAE::T_REAL_DEFAULT().clone()
        },
        (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
            outExp1 = Expression::typeCastElements(inExp1, &(DAE::T_REAL_DEFAULT().clone()))?;
            DAE::T_REAL_DEFAULT().clone()
        },
        (Deref @ DAE::Type::T_METABOXED { .. }, _) => {
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1, var_field!((*ty1).ty, DAE::Type::T_METABOXED).clone(), inExp2, &ty2, inAllowUnknown)?;
            outExp1 = if (isBoxedType(&ty2)) {outExp1} else {metamodelica::Ref::new(DAE::Exp::UNBOX { exp: outExp1, ty: outCompatType })};
            ty2
        },
        (_, Deref @ DAE::Type::T_METABOXED { .. }) => {
            (outExp1, outExp2, outCompatType, outCompatible) = checkTypeCompat(inExp1, ty1.clone(), inExp2, var_field!((*ty2).ty, DAE::Type::T_METABOXED), inAllowUnknown)?;
            outExp2 = if (isBoxedType(&ty1)) {outExp2} else {metamodelica::Ref::new(DAE::Exp::UNBOX { exp: outExp2, ty: outCompatType })};
            ty1
        },
        (Deref @ DAE::Type::T_METARECORD { .. }, Deref @ DAE::Type::T_METAUNIONTYPE { .. }) => {
            outCompatible = AbsynUtil::pathEqual(var_field!((*ty1).utPath, DAE::Type::T_METARECORD), var_field!((*ty2).path, DAE::Type::T_METAUNIONTYPE));
            ty2
        },
        (Deref @ DAE::Type::T_METAUNIONTYPE { .. }, Deref @ DAE::Type::T_METARECORD { .. }) => {
            outCompatible = AbsynUtil::pathEqual(var_field!((*ty1).path, DAE::Type::T_METAUNIONTYPE), var_field!((*ty2).utPath, DAE::Type::T_METARECORD));
            ty1
        },
        (Deref @ DAE::Type::T_UNKNOWN { .. }, _) => {
            outCompatible = inAllowUnknown;
            ty2
        },
        (_, Deref @ DAE::Type::T_UNKNOWN { .. }) => {
            outCompatible = inAllowUnknown;
            ty1
        },
        _ => {
            outCompatible = false;
            DAE::T_UNKNOWN_DEFAULT().clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp1, outExp2, outCompatType, outCompatible))
}

pub fn arrayHasUnknownDims(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<bool> {
    let mut outUnknownDims: bool;
    outUnknownDims = (match &**inType {
        DAE::Type::T_ARRAY {
            dims: __inType_dims,
            ty: __inType_ty,
        } => {
            List::any(
                metamodelica::AsArg::as_arg(&__inType_dims),
                &move |__a0: metamodelica::Ref<DAE::Dimension>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Expression::dimensionUnknown(&__a0))
                },
            )? || arrayHasUnknownDims(metamodelica::AsArg::as_arg(&__inType_ty))?
        }
        _ => false,
    });
    Ok(outUnknownDims)
}

pub fn metaArrayElementType<'__b>(
    mut inType: &'__b metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_METAARRAY { .. } => return Ok(var_field!((**inType).ty, DAE::Type::T_METAARRAY).clone()),
            DAE::Type::T_METATYPE { .. } => {
                inType = var_field!((**inType).ty, DAE::Type::T_METATYPE);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub(crate) fn isMetaArray<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_METAARRAY { .. } => return true,
            DAE::Type::T_METATYPE { .. } => {
                inType = var_field!((**inType).ty, DAE::Type::T_METATYPE);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn getAttributes<'__b>(
    mut inType: &'__b metamodelica::Ref<DAE::Type>,
) -> metamodelica::List<metamodelica::Ref<DAE::Var>> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_REAL { .. } => return var_field!((**inType).varLst, DAE::Type::T_REAL).clone(),
            DAE::Type::T_INTEGER { .. } => return var_field!((**inType).varLst, DAE::Type::T_INTEGER).clone(),
            DAE::Type::T_STRING { .. } => return var_field!((**inType).varLst, DAE::Type::T_STRING).clone(),
            DAE::Type::T_BOOL { .. } => return var_field!((**inType).varLst, DAE::Type::T_BOOL).clone(),
            DAE::Type::T_ENUMERATION { .. } => {
                return var_field!((**inType).attributeLst, DAE::Type::T_ENUMERATION).clone();
            }
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                inType = var_field!((**inType).complexType, DAE::Type::T_SUBTYPE_BASIC);
                continue '__tco;
            }
            _ => return metamodelica::nil(),
        }
    }
}

pub fn lookupAttributeValue(
    mut inAttributes: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inName: &ArcStr,
) -> Option<metamodelica::Ref<Values::Value>> {
    let mut outValue: Option<metamodelica::Ref<Values::Value>> = None;
    for mut attr in &**inAttributes {
        if metamodelica::stringEq(&inName, &(TypesDump::getVarName(metamodelica::AsArg::as_arg(&attr)))) {
            outValue = DAEUtil::bindingValue(&(varBinding(metamodelica::AsArg::as_arg(&attr))));
            break;
        }
    }
    outValue
}

pub fn lookupAttributeExp(
    mut inAttributes: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inName: &ArcStr,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outExp: Option<metamodelica::Ref<DAE::Exp>> = None;
    for mut attr in &**inAttributes {
        if metamodelica::stringEq(&inName, &(TypesDump::getVarName(metamodelica::AsArg::as_arg(&attr)))) {
            outExp = DAEUtil::bindingExp(&(varBinding(metamodelica::AsArg::as_arg(&attr))))?;
            break;
        }
    }
    Ok(outExp)
}

fn unboxedTypeTraverseHelper<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut dummy: T,
) -> Result<(metamodelica::Ref<DAE::Type>, T)> {
    let mut oty: metamodelica::Ref<DAE::Type> = unboxedType(ty.clone())?;
    let mut odummy: T = dummy;
    Ok((oty, odummy))
}

pub fn getMetaRecordFields(
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut fields: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    fields = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METARECORD { fields: __esc_fields, .. } => {
            fields = (*__esc_fields).clone();
            fields.clone()
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: false, .. } => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.getMetaRecordFields")); __mm_s.push_str(&*literal!(" called on a non-singleton uniontype: ")); __mm_s.push_str(&*TypesDump::unparseType(ty)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/Types.mo"))?;
            return Err("fail")
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: Deref @ DAE::Type::T_METARECORD { fields: __esc_fields, .. } }, .. } => {
            fields = (*__esc_fields).clone();
            fields.clone()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Types.getMetaRecordFields")); __mm_s.push_str(&*literal!(" called on a non-singleton uniontype: ")); __mm_s.push_str(&*TypesDump::unparseType(ty)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/Types.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(fields)
}

pub fn getMetaRecordIfSingleton(mut ty: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut oty: metamodelica::Ref<DAE::Type>;
    oty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: false, .. } => {
            ty
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: __esc_oty }, typeVars: __ty_typeVars, .. } => {
            oty = (*__esc_oty).clone();
            setTypeVariables(oty.clone(), __ty_typeVars.clone())
        },
        _ => {
            ty
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oty)
}

pub fn setTypeVariables(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut typeVars: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> metamodelica::Ref<DAE::Type> {
    let mut oty: metamodelica::Ref<DAE::Type>;
    oty = (::match_deref::match_deref! { match &(ty.clone()) {
        __esc_oty @ Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
            oty = (*__esc_oty).clone();
            assign_variant_field!(oty => DAE::Type::T_METAUNIONTYPE; typeVars = typeVars);
            oty.clone()
        },
        __esc_oty @ Deref @ DAE::Type::T_METARECORD { .. } => {
            oty = (*__esc_oty).clone();
            assign_variant_field!(oty => DAE::Type::T_METARECORD; typeVars = typeVars);
            oty.clone()
        },
        _ => ty,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oty
}

pub fn isExpandableConnector(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut isExpandable: bool;
    isExpandable = (match &**ty {
        DAE::Type::T_COMPLEX {
            complexClassType:
                ClassInf::State::CONNECTOR {
                    path: _,
                    isExpandable: true,
                },
            ..
        } => true,
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType:
                ClassInf::State::CONNECTOR {
                    path: _,
                    isExpandable: true,
                },
            ..
        } => true,
        _ => false,
    });
    isExpandable
}

pub fn getBasicType<'__b>(mut ty: &'__b metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    '__tco: loop {
        match &**ty {
            DAE::Type::T_ARRAY { .. } => {
                ty = var_field!((**ty).ty, DAE::Type::T_ARRAY);
                continue '__tco;
            }
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                ty = var_field!((**ty).complexType, DAE::Type::T_SUBTYPE_BASIC);
                continue '__tco;
            }
            _ => return ty.clone(),
        }
    }
}

pub(crate) fn resultExps<'__b>(
    mut inCases: &'__b metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    exps = (::match_deref::match_deref! { match inCases {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { result: Some(exp), .. }, tail: cases } => {
            exps = resultExps(cases);
            metamodelica::cons(exp.clone(), exps)
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: cases } => {
            resultExps(cases)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exps
}

pub fn fixCaseReturnTypes2(
    mut inCases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::MatchCase>>> {
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    outCases = 'mc: {
        let __mc_input = (&**inCases, inExps, inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls: decls, body, result: Some(_), resultInfo, jump, info: info2 }, tail: cases }, Deref @ metamodelica::ListNode::Cons { head: exp, tail: exps }, info) => {
                    let mut cases = (*cases).clone();
                    cases = fixCaseReturnTypes2(metamodelica::AsArg::as_arg(&cases), exps.clone(), info.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard.clone(), localDecls: decls.clone(), body: body.clone(), result: Some(exp.clone()), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info2.clone() }), cases.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: case_ @ Deref @ DAE::MatchCase { result: None, .. }, tail: cases }, exps, info) => {
                    let mut cases = (*cases).clone();
                    cases = fixCaseReturnTypes2(metamodelica::AsArg::as_arg(&cases), exps.clone(), info.clone())?;
                    Ok(metamodelica::cons(case_.clone(), cases.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Types.fixCaseReturnTypes2 failed")], &inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCases)
}
