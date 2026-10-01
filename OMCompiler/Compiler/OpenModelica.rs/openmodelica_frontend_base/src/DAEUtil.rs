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

use crate::Algorithm;
use crate::ComponentReference;
use crate::DAEDump;
use crate::Expression;
use crate::ExpressionSimplify;
use crate::Types;
use crate::ValuesUtil;
use crate::VarTransform;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlSetCR;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub fn constStr(mut r#const: DAE::Const) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match r#const {
        DAE::Const::C_VAR { .. } => literal!("VAR"),
        DAE::Const::C_PARAM { .. } => literal!("PARAM"),
        DAE::Const::C_CONST { .. } => literal!("CONST"),
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

pub fn const2VarKind(mut r#const: DAE::Const) -> Result<DAE::VarKind> {
    let mut kind: DAE::VarKind;
    kind = (match r#const {
        DAE::Const::C_VAR { .. } => openmodelica_frontend_types::DAE::VarKind::VARIABLE,
        DAE::Const::C_PARAM { .. } => openmodelica_frontend_types::DAE::VarKind::PARAM,
        DAE::Const::C_CONST { .. } => openmodelica_frontend_types::DAE::VarKind::CONST,
        _ => return Err("match: no arm matched"),
    });
    Ok(kind)
}

pub(crate) fn expTypeSimple(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut isSimple: bool;
    isSimple = (match &**tp {
        DAE::Type::T_REAL { .. } => true,
        DAE::Type::T_INTEGER { .. } => true,
        DAE::Type::T_STRING { .. } => true,
        DAE::Type::T_BOOL { .. } => true,
        DAE::Type::T_CLOCK { .. } => true,
        DAE::Type::T_ENUMERATION { .. } => true,
        _ => false,
    });
    isSimple
}

pub fn expTypeElementType<'__b>(mut tp: &'__b metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    '__tco: loop {
        match &**tp {
            DAE::Type::T_ARRAY { ty, .. } => {
                tp = ty;
                continue '__tco;
            }
            _ => return tp.clone(),
        }
    }
}

pub fn expTypeComplex(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut isComplex: bool;
    isComplex = (match &**tp {
        DAE::Type::T_COMPLEX { .. } => true,
        _ => false,
    });
    isComplex
}

pub fn expTypeArray(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut isArray: bool;
    isArray = (match &**tp {
        DAE::Type::T_ARRAY { .. } => true,
        _ => false,
    });
    isArray
}

pub fn expTypeTuple(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut isTuple: bool;
    isTuple = (match &**tp {
        DAE::Type::T_TUPLE { .. } => true,
        _ => false,
    });
    isTuple
}

pub fn expTypeArrayDimensions(mut tp: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::List<i32>> {
    let mut dims: metamodelica::List<i32>;
    dims = (match &**tp {
        DAE::Type::T_ARRAY { dims: array_dims, .. } => {
            dims = List::map(array_dims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| {
                Expression::dimensionSize(&__a0)
            })?;
            dims
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(dims)
}

pub(crate) fn typeExp(mut tp: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (::match_deref::match_deref! { match tp {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: rest }, .. } => {
            exp = dimExp(dim.clone())?;
            for mut d in &*rest.clone() {
                exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp, operator: DAE::Operator::MUL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: dimExp(d.clone())? });
            }
            exp
        },
        _ => {
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn dimExp(mut dim: metamodelica::Ref<DAE::Dimension>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (match &*dim {
        DAE::Dimension::DIM_INTEGER { integer: iconst } => metamodelica::Ref::new(DAE::Exp::ICONST {
            integer: iconst.clone(),
        }),
        DAE::Dimension::DIM_EXP { exp: __esc_exp } => {
            exp = (*__esc_exp).clone();
            exp.clone()
        }
        _ => {
            Error::addMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![anyString(dim)])?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub fn derivativeOrder(mut e1: &(i32, DAE::derivativeCond), mut e2: &(i32, DAE::derivativeCond)) -> bool {
    let mut b: bool;
    let mut i1: i32;
    let mut i2: i32;
    b = (match (e1.clone(), e2.clone()) {
        ((mut __esc_i1, _), (mut __esc_i2, _)) => {
            i1 = __esc_i1.clone();
            i2 = __esc_i2.clone();
            Util::isIntGreater(i1, i2)
        }
    });
    b
}

pub fn getDerivativePaths<'__b>(
    mut inFuncDefs: &'__b metamodelica::List<DAE::FunctionDefinition>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    paths = (::match_deref::match_deref! { match inFuncDefs {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivativeFunction: p1, defaultDerivative: Some(p2), lowerOrderDerivatives: pLst1, .. }, tail: funcDefs } => {
            let mut pLst2: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            pLst2 = getDerivativePaths(funcDefs);
            paths = List::union(&(metamodelica::cons(p1.clone(), metamodelica::cons(p2.clone(), pLst1.clone()))), &pLst2);
            paths
        },
        Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivativeFunction: p1, defaultDerivative: None, lowerOrderDerivatives: pLst1, .. }, tail: funcDefs } => {
            let mut pLst2: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            pLst2 = getDerivativePaths(funcDefs);
            paths = List::union(&(metamodelica::cons(p1.clone(), pLst1.clone())), &pLst2);
            paths
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: funcDefs } => {
            getDerivativePaths(funcDefs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    paths
}

pub(crate) fn addEquationBoundString(
    mut bindExp: metamodelica::Ref<DAE::Exp>,
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    oattr = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: e1, unit: e2, displayUnit: e3, min, max, start: e4, fixed: e5, nominal: e6, stateSelectOption: sSelectOption, uncertainOption: unc, distributionOption: distOption, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: e1.clone(), unit: e2.clone(), displayUnit: e3.clone(), min: min.clone(), max: max.clone(), start: e4.clone(), fixed: e5.clone(), nominal: e6.clone(), stateSelectOption: sSelectOption.clone(), uncertainOption: unc.clone(), distributionOption: distOption.clone(), equationBound: Some(bindExp), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: e1, min, max, start: e2, fixed: e3, uncertainOption: unc, distributionOption: distOption, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: e1.clone(), min: min.clone(), max: max.clone(), start: e2.clone(), fixed: e3.clone(), uncertainOption: unc.clone(), distributionOption: distOption.clone(), equationBound: Some(bindExp), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: e1, start: e2, fixed: e3, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: e1.clone(), start: e2.clone(), fixed: e3.clone(), equationBound: Some(bindExp), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: e1, start: e2, fixed: e3, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: e1.clone(), start: e2.clone(), fixed: e3.clone(), equationBound: Some(bindExp), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: e1, min, max, start: e2, fixed: e3, equationBound: _, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: e1.clone(), min: min.clone(), max: max.clone(), start: e2.clone(), fixed: e3.clone(), equationBound: Some(bindExp), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        _ => {
            metamodelica::print(literal!("-failure in DAEUtil.addEquationBoundString\n"));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oattr)
}

pub fn getClassList(mut v: &metamodelica::Ref<DAE::Element>) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    lst = (::match_deref::match_deref! { match v {
        Deref @ DAE::Element::VAR { source: Deref @ DAE::ElementSource { typeLst: __esc_lst, .. }, .. } => {
            lst = (*__esc_lst).clone();
            lst.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub fn getEmptyVarAttr(mut ty: &metamodelica::Ref<DAE::Type>) -> Option<metamodelica::Ref<DAE::VariableAttributes>> {
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    attr = (match &*(Types::getBasicType(ty)) {
        DAE::Type::T_REAL { .. } => Some(DAE::emptyVarAttrReal().clone()),
        DAE::Type::T_INTEGER { .. } => Some(DAE::emptyVarAttrInt().clone()),
        DAE::Type::T_BOOL { .. } => Some(DAE::emptyVarAttrBool().clone()),
        DAE::Type::T_STRING { .. } => Some(DAE::emptyVarAttrString().clone()),
        DAE::Type::T_ENUMERATION { .. } => Some(DAE::emptyVarAttrEnum().clone()),
        DAE::Type::T_CLOCK { .. } => Some(DAE::emptyVarAttrClock().clone()),
        _ => None,
    });
    attr
}

pub(crate) fn getBoundStartEquation(
    mut attr: &metamodelica::Ref<DAE::VariableAttributes>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oe: metamodelica::Ref<DAE::Exp>;
    oe = (::match_deref::match_deref! { match attr {
        Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { equationBound: Some(beq), .. } => {
            beq.clone()
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_INT { equationBound: Some(beq), .. } => {
            beq.clone()
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { equationBound: Some(beq), .. } => {
            beq.clone()
        },
        Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { equationBound: Some(beq), .. } => {
            beq.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(oe)
}

pub fn splitDAEIntoVarsAndEquations(mut inDae: DAE::DAElist) -> Result<(DAE::DAElist, DAE::DAElist)> {
    let mut allVars: DAE::DAElist;
    let mut allEqs: DAE::DAElist;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut vars: DoubleEnded::MutableList<metamodelica::Ref<DAE::Element>>;
    let mut eqs: DoubleEnded::MutableList<metamodelica::Ref<DAE::Element>>;
    let DAE::DAE { elementLst: __pa0 } = inDae;
    rest = metamodelica::Own::own(__pa0);
    vars = DoubleEnded::fromList(&(metamodelica::nil()))?;
    eqs = DoubleEnded::fromList(&(metamodelica::nil()))?;
    for mut elt in &*rest {
        let () = (match &*elt.clone() {
            DAE::Element::VAR { .. } => {
                DoubleEnded::push_back(vars.clone(), elt.clone())?;
                ()
            }
            DAE::Element::COMP {
                ident: id,
                dAElist: elts1,
                source,
                comment: cmt,
            } => {
                let mut elts11: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut elts3: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let (DAE::DAE { elementLst: __pa0 }, DAE::DAE { elementLst: __pa1 }) =
                    splitDAEIntoVarsAndEquations(DAE::DAElist {
                        elementLst: elts1.clone(),
                    })?;
                elts11 = metamodelica::Own::own(__pa0);
                elts3 = metamodelica::Own::own(__pa1);
                DoubleEnded::push_back(
                    vars.clone(),
                    metamodelica::Ref::new(DAE::Element::COMP {
                        ident: id.clone(),
                        dAElist: elts11,
                        source: source.clone(),
                        comment: cmt.clone(),
                    }),
                )?;
                DoubleEnded::push_list_back(eqs.clone(), &elts3)?;
                ()
            }
            DAE::Element::EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::EQUEQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIALEQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::ARRAY_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::COMPLEX_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIALDEFINE { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::DEFINE { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::WHEN_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::FOR_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIAL_FOR_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::IF_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIAL_IF_EQUATION { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::ALGORITHM { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIALALGORITHM { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::EXTOBJECTCLASS { .. } => {
                DoubleEnded::push_back(vars.clone(), elt.clone())?;
                ()
            }
            DAE::Element::ASSERT { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIAL_ASSERT { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::TERMINATE { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIAL_TERMINATE { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::REINIT { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::NORETCALL { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            DAE::Element::INITIAL_NORETCALL { .. } => {
                DoubleEnded::push_back(eqs.clone(), elt.clone())?;
                ()
            }
            _ => {
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("DAEUtil.splitDAEIntoVarsAndEquations"));
                        __mm_s.push_str(&*literal!(" failed for "));
                        __mm_s.push_str(&*DAEDump::dumpDAEElementsStr(
                            &(DAE::DAElist {
                                elementLst: list![elt.clone()],
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    },
                    metamodelica::sourceInfo!("FrontEnd/DAEUtil.mo"),
                )?;
                return Err("fail");
            }
        });
    }
    allVars = DAE::DAElist {
        elementLst: DoubleEnded::toListAndClear(vars, metamodelica::nil())?,
    };
    allEqs = DAE::DAElist {
        elementLst: DoubleEnded::toListAndClear(eqs, metamodelica::nil())?,
    };
    Ok((allVars, allEqs))
}

pub fn removeVariables(
    mut dae: DAE::DAElist,
    mut vars: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((dae.clone(), vars.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            dae
        },
        (DAE::DAElist { elementLst: elements }, _) => {
            let mut elements = (*elements).clone();
            elements = removeVariablesFromElements(elements.clone(), vars)?;
            DAE::DAElist { elementLst: elements.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDae)
}

fn removeVariablesFromElements(
    mut inElements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut variableNames: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    if (variableNames).is_empty() {
        outElements = inElements;
        return Ok(outElements);
    }
    for mut el in &*inElements {
        let () = (::match_deref::match_deref! { match &(el.clone()) {
            v @ Deref @ DAE::Element::VAR { componentRef: cr, .. } => {
                if ((List::select1(variableNames.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone())?)).is_empty() {
                    outElements = metamodelica::cons(v.clone(), outElements);
                }
                ()
            },
            Deref @ DAE::Element::COMP { ident: id, dAElist: elist, source, comment: cmt } => {
                let mut elist = (*elist).clone();
                elist = removeVariablesFromElements(elist.clone(), variableNames)?;
                outElements = metamodelica::cons(metamodelica::Ref::new(DAE::Element::COMP { ident: id.clone(), dAElist: elist.clone(), source: source.clone(), comment: cmt.clone() }), outElements);
                ()
            },
            _ => {
                outElements = metamodelica::cons(el.clone(), outElements);
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outElements = metamodelica::Dangerous::listReverseInPlace(outElements);
    Ok(outElements)
}

fn removeVariable(mut var: &metamodelica::Ref<DAE::ComponentRef>, mut dae: &DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = dae;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(DAE::DAElist { elementLst: metamodelica::nil() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, .. }, tail: elist } } => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(var, metamodelica::AsArg::as_arg(&cr))?) else { return Err("pattern mismatch") };
                    Ok(DAE::DAElist { elementLst: elist.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { ident: id, dAElist: elist, source, comment: cmt }, tail: elist2 } } => {
                    let mut elist = (*elist).clone();
                    let mut elist2 = (*elist2).clone();
                    let DAE::DAE { elementLst: __pa0 } = removeVariable(var, &(DAE::DAElist { elementLst: elist.clone() }))?;
                    elist = metamodelica::Own::own(__pa0);
                    let DAE::DAE { elementLst: __pa1 } = removeVariable(var, &(DAE::DAElist { elementLst: elist2.clone() }))?;
                    elist2 = metamodelica::Own::own(__pa1);
                    Ok(DAE::DAElist { elementLst: metamodelica::cons(metamodelica::Ref::new(DAE::Element::COMP { ident: id.clone(), dAElist: elist.clone(), source: source.clone(), comment: cmt.clone() }), elist2.clone()) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: elist } } => {
                    let mut elist = (*elist).clone();
                    let DAE::DAE { elementLst: __pa0 } = removeVariable(var, &(DAE::DAElist { elementLst: elist.clone() }))?;
                    elist = metamodelica::Own::own(__pa0);
                    Ok(DAE::DAElist { elementLst: metamodelica::cons(e.clone(), elist.clone()) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDae)
}

pub(crate) fn removeInnerAttrs(
    mut dae: DAE::DAElist,
    mut vars: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = List::fold(
        vars,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: DAE::DAElist| removeInnerAttr(&__a0, &__a1),
        dae,
    )?;
    Ok(outDae)
}

pub(crate) fn removeInnerAttr(
    mut var: &metamodelica::Ref<DAE::ComponentRef>,
    mut dae: &DAE::DAElist,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &(dae) {
        DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Nil } => {
            DAE::DAElist { elementLst: metamodelica::nil() }
        },
        DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: oldVar, kind, direction: dir, parallelism: prl, protection: prot, ty: tp, binding: bind, dims: dim, connectorType: ct, source, variableAttributesOption: attr, comment: cmt, innerOuter: Absyn::InnerOuter::INNER_OUTER { .. }, encrypted: ie }, tail: elist } } if (compareUniquedVarWithNonUnique(var, metamodelica::AsArg::as_arg(&oldVar))?) => {
            let mut newVar: metamodelica::Ref<DAE::ComponentRef>;
            let mut u: metamodelica::Ref<DAE::Element>;
            let mut o: metamodelica::Ref<DAE::Element>;
            let mut elist = (*elist).clone();
            newVar = nameInnerouterUniqueCref(metamodelica::AsArg::as_arg(&oldVar))?;
            o = metamodelica::Ref::new(DAE::Element::VAR { componentRef: oldVar.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: None, dims: dim.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: openmodelica_ast::Absyn::InnerOuter::OUTER, encrypted: ie.clone() });
            u = metamodelica::Ref::new(DAE::Element::VAR { componentRef: newVar, kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: bind.clone(), dims: dim.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: ie.clone() });
            elist = metamodelica::cons(u, metamodelica::cons(o, elist.clone()));
            DAE::DAElist { elementLst: elist.clone() }
        },
        DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, kind, direction: dir, parallelism: prl, protection: prot, ty: tp, binding: bind, dims: dim, connectorType: ct, source, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: ie }, tail: elist } } if (ComponentReferenceBasics::crefEqualNoStringCompare(var, metamodelica::AsArg::as_arg(&cr))?) => {
            let mut io2: Absyn::InnerOuter;
            io2 = removeInnerAttribute(io.clone());
            DAE::DAElist { elementLst: metamodelica::cons(metamodelica::Ref::new(DAE::Element::VAR { componentRef: cr.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: bind.clone(), dims: dim.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io2, encrypted: ie.clone() }), elist.clone()) }
        },
        DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { ident: id, dAElist: elist, source, comment: cmt }, tail: elist2 } } => {
            let mut elist = (*elist).clone();
            let mut elist2 = (*elist2).clone();
            let DAE::DAE { elementLst: __pa0 } = removeInnerAttr(var, &(DAE::DAElist { elementLst: elist.clone() }))?;
            elist = metamodelica::Own::own(__pa0);
            let DAE::DAE { elementLst: __pa1 } = removeInnerAttr(var, &(DAE::DAElist { elementLst: elist2.clone() }))?;
            elist2 = metamodelica::Own::own(__pa1);
            DAE::DAElist { elementLst: metamodelica::cons(metamodelica::Ref::new(DAE::Element::COMP { ident: id.clone(), dAElist: elist.clone(), source: source.clone(), comment: cmt.clone() }), elist2.clone()) }
        },
        DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: elist } } => {
            let mut elist = (*elist).clone();
            let DAE::DAE { elementLst: __pa0 } = removeInnerAttr(var, &(DAE::DAElist { elementLst: elist.clone() }))?;
            elist = metamodelica::Own::own(__pa0);
            DAE::DAElist { elementLst: metamodelica::cons(e.clone(), elist.clone()) }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outDae)
}

fn compareUniquedVarWithNonUnique(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut equal: bool;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    s1 = ComponentReferenceBasics::printComponentRefStr(cr1)?;
    s2 = ComponentReferenceBasics::printComponentRefStr(cr2)?;
    s1 = System::stringReplace(s1, arcstr::literal!(DAE::UNIQUEIO), literal!(""))?;
    s2 = System::stringReplace(s2, arcstr::literal!(DAE::UNIQUEIO), literal!(""))?;
    equal = stringEq(&s1, &s2);
    Ok(equal)
}

pub(crate) fn nameInnerouterUniqueCref(
    mut inCr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    outCr = (match &**inCr {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: idt,
            subscriptLst: subs,
        } => {
            let mut id = (*id).clone();
            id = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(DAE::UNIQUEIO));
                __mm_s.push_str(&*id);
                ArcStr::from(__mm_s)
            };
            ComponentReferenceBasics::makeCrefIdent(id.clone(), idt.clone(), subs.clone())
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: idt,
            subscriptLst: subs,
            componentRef: child,
        } => {
            let mut newChild: metamodelica::Ref<DAE::ComponentRef>;
            newChild = nameInnerouterUniqueCref(child)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), idt.clone(), subs.clone(), newChild)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCr)
}

pub(crate) fn unNameInnerouterUniqueCref(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut removalString: &ArcStr,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut ocr: metamodelica::Ref<DAE::ComponentRef>;
    ocr = 'mc: {
        let __mc_input = cr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: r#str, identType: ty, subscriptLst: subs } => {
                    let mut str2: ArcStr;
                    str2 = System::stringReplace(r#str.clone(), removalString.clone(), literal!(""))?;
                    Ok(ComponentReferenceBasics::makeCrefIdent(str2.clone(), ty.clone(), subs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: r#str, identType: ty, subscriptLst: subs, componentRef: child } => {
                    let mut str2: ArcStr;
                    let mut child_2: metamodelica::Ref<DAE::ComponentRef>;
                    child_2 = unNameInnerouterUniqueCref(child.clone(), removalString)?;
                    str2 = System::stringReplace(r#str.clone(), removalString.clone(), literal!(""))?;
                    Ok(ComponentReferenceBasics::makeCrefQual(str2.clone(), ty.clone(), subs.clone(), child_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::WILD { .. } => {
                    Ok(openmodelica_frontend_types::DAE::ComponentRef::interned_WILD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                child => {
                    metamodelica::print(literal!(" failure unNameInnerouterUniqueCref: "));
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&child))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(ocr)
}

fn removeInnerAttribute(mut io: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut ioOut: Absyn::InnerOuter;
    ioOut = (match io {
        Absyn::InnerOuter::INNER { .. } => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        Absyn::InnerOuter::INNER_OUTER { .. } => openmodelica_ast::Absyn::InnerOuter::OUTER,
        _ => io,
    });
    ioOut
}

pub fn varCref(mut elt: &metamodelica::Ref<DAE::Element>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &((*elt)) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr = metamodelica::Own::own(__pa0);
    Ok(cr)
}

pub(crate) fn getVariableAttributes(
    mut elt: &metamodelica::Ref<DAE::Element>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut variableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let __pa0 = ::match_deref::match_deref! { match &((*elt)) {
        Deref @ DAE::Element::VAR { variableAttributesOption: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    variableAttributesOption = metamodelica::Own::own(__pa0);
    Ok(variableAttributesOption)
}

pub fn getUnitAttr(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut start: metamodelica::Ref<DAE::Exp>;
    start = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: Some(u), .. }) => {
            u.clone()
        },
        _ => {
            metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    start
}

pub(crate) fn getStartAttrEmpty(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut optExp: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut start: metamodelica::Ref<DAE::Exp>;
    start = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { start: Some(r), .. }) => {
            r.clone()
        },
        _ => {
            optExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    start
}

pub fn getMinMax(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut oExps: metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>;
    oExps = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min: e1, max: e2, .. }) => {
            list![e1.clone(), e2.clone()]
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min: e1, max: e2, .. }) => {
            list![e1.clone(), e2.clone()]
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min: e1, max: e2, .. }) => {
            list![e1.clone(), e2.clone()]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oExps
}

pub fn getMinMaxValues(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>) {
    let mut outMinValue: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outMaxValue: Option<metamodelica::Ref<DAE::Exp>>;
    (outMinValue, outMaxValue) = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min: minValue, max: maxValue, .. }) => {
            (minValue.clone(), maxValue.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min: minValue, max: maxValue, .. }) => {
            (minValue.clone(), maxValue.clone())
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min: minValue, max: maxValue, .. }) => {
            (minValue.clone(), maxValue.clone())
        },
        _ => {
            (None, None)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outMinValue, outMaxValue)
}

pub fn setMinMax(
    mut inAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut inMin: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMax: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(inAttr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            if ((match (&(min), &(inMin)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && (match (&(max), &(inMax)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inAttr} else {Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: inMin, max: inMax, start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))}
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            if ((match (&(min), &(inMin)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && (match (&(max), &(inMax)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inAttr} else {Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: inMin, max: inMax, start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))}
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            if ((match (&(min), &(inMin)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && (match (&(max), &(inMax)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inAttr} else {Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: inMin, max: inMax, start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))}
        },
        None => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: inMin, max: inMax, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getStartAttr(
    mut inAttributes: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut start: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    start = (::match_deref::match_deref! { match &(inAttributes) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(__esc_e), .. }) => {
            e = (*__esc_e).clone();
            e.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(__esc_e), .. }) => {
            e = (*__esc_e).clone();
            e.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: Some(__esc_e), .. }) => {
            e = (*__esc_e).clone();
            e.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: Some(__esc_e), .. }) => {
            e = (*__esc_e).clone();
            e.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { start: Some(__esc_e), .. }) => {
            e = (*__esc_e).clone();
            e.clone()
        },
        _ => (match &*(Types::getBasicType(inType)) {
        DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
        DAE::Type::T_STRING { .. } => metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }),
        DAE::Type::T_BOOL { .. } => metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
        DAE::Type::T_ENUMERATION { .. } => Types::getNthEnumLiteral(inType, 1)?,
        _ => metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }),
    }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(start)
}

pub fn getStartOrigin(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<Option<DAE::StartOrigin>> {
    let mut startOrigin: Option<DAE::StartOrigin>;
    startOrigin = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { startOrigin: so, .. }) => {
            so.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { startOrigin: so, .. }) => {
            so.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { startOrigin: so, .. }) => {
            so.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { startOrigin: so, .. }) => {
            so.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { startOrigin: so, .. }) => {
            so.clone()
        },
        None => {
            None
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(startOrigin)
}

pub fn getStartAttrFail(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut start: metamodelica::Ref<DAE::Exp>;
    start = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: Some(r), .. }) => {
            r.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { start: Some(r), .. }) => {
            r.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(start)
}

pub fn getNominalAttrFail(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut nominal: metamodelica::Ref<DAE::Exp>;
    nominal = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { nominal: Some(r), .. }) => {
            r.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(nominal)
}

pub fn getMinAttrFail(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outMin: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min: Some(__pa0), .. }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outMin = metamodelica::Own::own(__pa0);
    Ok(outMin)
}

pub fn getMaxAttrFail(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outMax: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { max: Some(__pa0), .. }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outMax = metamodelica::Own::own(__pa0);
    Ok(outMax)
}

pub fn setVariableAttributes(
    mut var: metamodelica::Ref<DAE::Element>,
    mut varOpt: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut v: metamodelica::Ref<DAE::Element> = var;
    v = (match &*v {
        DAE::Element::VAR { .. } => {
            assign_variant_field!(v => DAE::Element::VAR; variableAttributesOption = varOpt);
            v
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(v)
}

pub fn setStateSelect(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut s: DAE::StateSelect,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(attr) {
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_REAL; stateSelectOption = Some(s));
            Some(va.clone())
        },
        None => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: Some(s), uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn setStartAttr(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut start: metamodelica::Ref<DAE::Exp>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = setStartAttrOption(attr, Some(start))?;
    Ok(outAttr)
}

pub fn setStartAttrOption(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut start: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            let mut at: Option<metamodelica::Ref<DAE::VariableAttributes>>;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_REAL).clone() == start.clone() {
                at = attr;
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_REAL; start = start);
                at = Some(va.clone());
            }
            at
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_INT { .. }) => {
            let mut at: Option<metamodelica::Ref<DAE::VariableAttributes>>;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_INT).clone() == start.clone() {
                at = attr;
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_INT; start = start);
                at = Some(va.clone());
            }
            at
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { .. }) => {
            let mut at: Option<metamodelica::Ref<DAE::VariableAttributes>>;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_BOOL).clone() == start.clone() {
                at = attr;
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_BOOL; start = start);
                at = Some(va.clone());
            }
            at
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { .. }) => {
            let mut at: Option<metamodelica::Ref<DAE::VariableAttributes>>;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_STRING).clone() == start.clone() {
                at = attr;
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_STRING; start = start);
                at = Some(va.clone());
            }
            at
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. }) => {
            let mut at: Option<metamodelica::Ref<DAE::VariableAttributes>>;
            let mut va = (*va).clone();
            if var_field!((*va).start, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone() == start.clone() {
                at = attr;
            } else {
                assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_ENUMERATION; start = start);
                at = Some(va.clone());
            }
            at
        },
        None => {
            if ((start).is_none()) {None} else {Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: start, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))}
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn setStartOrigin(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut startOrigin: Option<DAE::StartOrigin>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(attr) {
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_REAL; startOrigin = startOrigin);
            Some(va.clone())
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_INT { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_INT; startOrigin = startOrigin);
            Some(va.clone())
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_BOOL; startOrigin = startOrigin);
            Some(va.clone())
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_STRING; startOrigin = startOrigin);
            Some(va.clone())
        },
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_ENUMERATION; startOrigin = startOrigin);
            Some(va.clone())
        },
        None => {
            if ((startOrigin).is_none()) {None} else {Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: startOrigin }))}
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getNominalAttr(mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>) -> metamodelica::Ref<DAE::Exp> {
    let mut nominal: metamodelica::Ref<DAE::Exp>;
    nominal = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { nominal: Some(n), .. }) => {
            n.clone()
        },
        _ => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    nominal
}

pub fn setNominalAttr(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut nominal: metamodelica::Ref<DAE::Exp>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(attr) {
        Some(va @ Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            let mut va = (*va).clone();
            assign_variant_field!(va => DAE::VariableAttributes::VAR_ATTR_REAL; nominal = Some(nominal));
            Some(va.clone())
        },
        None => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: Some(nominal), stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn setUnitAttr(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut unit: metamodelica::Ref<DAE::Exp>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: _, displayUnit: du, min, max, start: s, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: Some(unit), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: s.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        None => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: Some(unit), displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn setElementVarVisibility(
    mut elt: metamodelica::Ref<DAE::Element>,
    mut visibility: DAE::VarVisibility,
) -> metamodelica::Ref<DAE::Element> {
    let mut e: metamodelica::Ref<DAE::Element> = elt;
    e = (match &*e {
        DAE::Element::VAR { .. } => {
            assign_variant_field!(e => DAE::Element::VAR; protection = visibility);
            e
        }
        _ => e,
    });
    e
}

pub fn setElementVarDirection(
    mut elt: metamodelica::Ref<DAE::Element>,
    mut direction: DAE::VarDirection,
) -> metamodelica::Ref<DAE::Element> {
    let mut e: metamodelica::Ref<DAE::Element> = elt;
    e = (match &*e {
        DAE::Element::VAR { .. } => {
            assign_variant_field!(e => DAE::Element::VAR; direction = direction);
            e
        }
        _ => e,
    });
    e
}

pub fn setElementVarBinding(
    mut elt: metamodelica::Ref<DAE::Element>,
    mut binding: Option<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::Ref<DAE::Element> {
    let mut e: metamodelica::Ref<DAE::Element> = elt;
    e = (match &*e {
        DAE::Element::VAR { .. } => {
            assign_variant_field!(e => DAE::Element::VAR; binding = binding);
            e
        }
        _ => e,
    });
    e
}

pub fn setProtectedAttr(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut isProtected: bool,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(attr) {
            Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
                return Ok(Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: Some(isProtected), finalPrefix: r#fn.clone(), startOrigin: so.clone() })))
            },
            Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
                return Ok(Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: Some(isProtected), finalPrefix: r#fn.clone(), startOrigin: so.clone() })))
            },
            Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
                return Ok(Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: Some(isProtected), finalPrefix: r#fn.clone(), startOrigin: so.clone() })))
            },
            Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
                return Ok(Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: Some(isProtected), finalPrefix: r#fn.clone(), startOrigin: so.clone() })))
            },
            Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: _, finalPrefix: r#fn, startOrigin: so }) => {
                return Ok(Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: Some(isProtected), finalPrefix: r#fn.clone(), startOrigin: so.clone() })))
            },
            Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: r#fn, finalPrefix: _ }) => {
                return Ok(Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: r#fn.clone(), finalPrefix: Some(isProtected) })))
            },
            None => {
                { (attr, isProtected) = (Some(DAE::emptyVarAttrReal().clone()), isProtected); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn getProtectedAttr(mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>) -> bool {
    let mut isProtected: bool;
    isProtected = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { isProtected: Some(__esc_isProtected), .. }) => {
            isProtected = (*__esc_isProtected).clone();
            isProtected.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { isProtected: Some(__esc_isProtected), .. }) => {
            isProtected = (*__esc_isProtected).clone();
            isProtected.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { isProtected: Some(__esc_isProtected), .. }) => {
            isProtected = (*__esc_isProtected).clone();
            isProtected.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { isProtected: Some(__esc_isProtected), .. }) => {
            isProtected = (*__esc_isProtected).clone();
            isProtected.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { isProtected: Some(__esc_isProtected), .. }) => {
            isProtected = (*__esc_isProtected).clone();
            isProtected.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: Some(__esc_isProtected), .. }) => {
            isProtected = (*__esc_isProtected).clone();
            isProtected.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isProtected
}

pub fn setFixedAttr(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut fixed: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: ini, fixed: _, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: ini.clone(), fixed: fixed, nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: ini, fixed: _, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: ini.clone(), fixed: fixed, uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: ini, fixed: _, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: ini.clone(), fixed: fixed, equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: ini, fixed: _, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: ini.clone(), fixed: fixed, equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: _, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: fixed, equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getFixedAttr(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut isFixed: Option<metamodelica::Ref<DAE::Exp>>;
    isFixed = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { fixed: __esc_isFixed, .. }) => {
            isFixed = (*__esc_isFixed).clone();
            isFixed.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { fixed: __esc_isFixed, .. }) => {
            isFixed = (*__esc_isFixed).clone();
            isFixed.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { fixed: __esc_isFixed, .. }) => {
            isFixed = (*__esc_isFixed).clone();
            isFixed.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { fixed: __esc_isFixed, .. }) => {
            isFixed = (*__esc_isFixed).clone();
            isFixed.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: __esc_isFixed, .. }) => {
            isFixed = (*__esc_isFixed).clone();
            isFixed.clone()
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isFixed
}

pub fn setFinalAttr(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut finalPrefix: bool,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: ip, finalPrefix: _ }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: ip.clone(), finalPrefix: Some(finalPrefix) }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: ip, finalPrefix: _, startOrigin: so }) => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: Some(finalPrefix), startOrigin: so.clone() }))
        },
        None => {
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: Some(finalPrefix), startOrigin: None }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub fn getFinalAttr(mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>) -> bool {
    let mut finalPrefix: bool;
    finalPrefix = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { finalPrefix: Some(b), .. }) => {
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    finalPrefix
}

pub fn boolVarVisibility(mut vp: DAE::VarVisibility) -> Result<bool> {
    let mut prot: bool;
    prot = (match vp {
        DAE::VarVisibility::PUBLIC { .. } => false,
        DAE::VarVisibility::PROTECTED { .. } => true,
        _ => {
            metamodelica::print(literal!("- DAEUtil.boolVarVisibility failed\n"));
            return Err("fail");
        }
    });
    Ok(prot)
}

pub fn hasStartAttr(mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>) -> bool {
    let mut hasStart: bool;
    hasStart = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(_), .. }) => true,
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(_), .. }) => true,
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: Some(_), .. }) => true,
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: Some(_), .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasStart
}

pub(crate) fn getStartAttrString(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inVariableAttributesOption;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: Some(r), .. }) => {
                    let mut s: ArcStr;
                    s = ExpressionBasics::printExpStr(r.clone())?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: Some(r), .. }) => {
                    let mut s: ArcStr;
                    s = ExpressionBasics::printExpStr(r.clone())?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

pub(crate) fn getMatchingElements(
    mut elist: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut cond: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    pub type FuncTypeElementTo =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>;

    let mut oelist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    oelist = List::filterOnTrue(elist, cond.clone())?;
    Ok(oelist)
}

pub(crate) fn getAllMatchingElements(
    mut elist: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut cond: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<()>,
) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    pub type FuncTypeElementTo =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<()> + 'static>;

    let mut outElist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    outElist = 'mc: {
        let __mc_input = &**elist;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { dAElist: elist1, .. }, tail: elist2 } => {
                    let mut elist1 = (*elist1).clone();
                    let mut elist2 = (*elist2).clone();
                    elist1 = getAllMatchingElements(metamodelica::AsArg::as_arg(&elist1), cond);
                    elist2 = getAllMatchingElements(metamodelica::AsArg::as_arg(&elist2), cond);
                    Ok(listAppend(elist1.clone(), elist2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: elist2 } => {
                    let mut elist2 = (*elist2).clone();
                    cond(e.clone())?;
                    elist2 = getAllMatchingElements(metamodelica::AsArg::as_arg(&elist2), cond);
                    Ok(metamodelica::cons(e.clone(), elist2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: elist2 } => {
                    Ok(getAllMatchingElements(metamodelica::AsArg::as_arg(&elist2), cond))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outElist
}

pub(crate) fn findAllMatchingElements(
    mut dae: DAE::DAElist,
    mut cond1: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool>,
    mut cond2: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool>,
) -> Result<(DAE::DAElist, DAE::DAElist)> {
    pub type CondFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>;

    let mut firstList: DAE::DAElist;
    let mut secondList: DAE::DAElist;
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut el1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut el2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let DAE::DAE { elementLst: __pa0 } = dae;
    elements = metamodelica::Own::own(__pa0);
    (el1, el2) = findAllMatchingElements2(&elements, cond1, cond2, metamodelica::nil(), metamodelica::nil())?;
    firstList = DAE::DAElist {
        elementLst: metamodelica::Dangerous::listReverseInPlace(el1),
    };
    secondList = DAE::DAElist {
        elementLst: metamodelica::Dangerous::listReverseInPlace(el2),
    };
    Ok((firstList, secondList))
}

fn findAllMatchingElements2(
    mut elements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut cond1: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool>,
    mut cond2: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool>,
    mut accumFirst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut accumSecond: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    pub type CondFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>;

    let mut firstList: metamodelica::List<metamodelica::Ref<DAE::Element>> = accumFirst;
    let mut secondList: metamodelica::List<metamodelica::Ref<DAE::Element>> = accumSecond;
    for mut e in &**elements {
        let () = (match &*e.clone() {
            DAE::Element::COMP {
                dAElist: __e_dAElist, ..
            } => {
                (firstList, secondList) = findAllMatchingElements2(
                    metamodelica::AsArg::as_arg(&__e_dAElist),
                    cond1,
                    cond2,
                    firstList,
                    secondList,
                )?;
                ()
            }
            _ => {
                if cond1(e.clone())? {
                    firstList = metamodelica::cons(e.clone(), firstList);
                }
                if cond2(e.clone())? {
                    secondList = metamodelica::cons(e.clone(), secondList);
                }
                ()
            }
        });
    }
    Ok((firstList, secondList))
}

pub(crate) fn isAfterIndexInlineFunc(mut inElem: &DAE::Function) -> bool {
    let mut b: bool;
    b = (match inElem.clone() {
        DAE::Function::FUNCTION {
            inlineType: DAE::InlineType::AFTER_INDEX_RED_INLINE { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

pub fn isParameter(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outB: bool;
    outB = (match &**inElement {
        DAE::Element::VAR {
            kind: DAE::VarKind::PARAM { .. },
            ..
        } => true,
        _ => false,
    });
    outB
}

pub(crate) fn isParameterOrConstant(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut b: bool;
    b = (match &**inElement {
        DAE::Element::VAR {
            kind: DAE::VarKind::CONST { .. },
            ..
        } => true,
        DAE::Element::VAR {
            kind: DAE::VarKind::PARAM { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

pub(crate) fn isParamOrConstVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<bool> {
    let mut outIsParamOrConst: bool;
    let mut var: SCode::Variability;
    let __arc2 = &(*inVar);
    let DAE::TYPES_VAR { attributes: __t1, .. } = &**__arc2;
    let __arc3 = __t1.clone();
    let DAE::ATTR { variability: __pa0, .. } = &*__arc3;
    var = metamodelica::Own::own(__pa0);
    outIsParamOrConst = SCodeUtil::isParameterOrConst(var);
    Ok(outIsParamOrConst)
}

pub fn isConstVar(mut var: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut isConstVar: bool = false;
    isConstVar = (match var.attributes.variability.clone() {
        SCode::Variability::CONST { .. } => true,
        _ => false,
    });
    isConstVar
}

pub(crate) fn isNotParamOrConstVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<bool> {
    let mut outIsNotParamOrConst: bool;
    outIsNotParamOrConst = !(isParamOrConstVar(inVar)?);
    Ok(outIsNotParamOrConst)
}

pub(crate) fn isParamConstOrComplexVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<bool> {
    let mut outIsParamConstComplex: bool;
    outIsParamConstComplex = isParamOrConstVar(inVar)? || isComplexVar(inVar);
    Ok(outIsParamConstComplex)
}

pub fn isParamOrConstVarKind(mut inVarKind: DAE::VarKind) -> bool {
    let mut outIsParamOrConst: bool;
    outIsParamOrConst = (match inVarKind {
        DAE::VarKind::PARAM { .. } => true,
        DAE::VarKind::CONST { .. } => true,
        _ => false,
    });
    outIsParamOrConst
}

pub(crate) fn isInnerVar(mut element: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut isInner: bool;
    isInner = (match &**element {
        DAE::Element::VAR {
            innerOuter: __element_innerOuter,
            ..
        } => AbsynUtil::isInner(__element_innerOuter.clone()),
        _ => false,
    });
    isInner
}

pub fn isOuterVar(mut element: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut isOuter: bool;
    isOuter = (match &**element {
        DAE::Element::VAR {
            innerOuter: Absyn::InnerOuter::OUTER { .. },
            ..
        } => true,
        _ => false,
    });
    isOuter
}

pub(crate) fn isComp(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<()> {
    let () = (match &**inElement {
        DAE::Element::COMP { .. } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn getOutputVars(
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut vl_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    vl_1 = getMatchingElements(
        vl,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isOutputVar(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(vl_1)
}

pub fn getOutputElements(
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut vl_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    vl_1 = getMatchingElements(
        vl,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isOutputElement(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(vl_1)
}

pub(crate) fn getProtectedVars(
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut vl_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    vl_1 = getMatchingElements(
        vl,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isProtectedVar(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(vl_1)
}

pub(crate) fn getBidirVars(
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut vl_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    vl_1 = getMatchingElements(
        vl,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isBidirVar(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(vl_1)
}

pub fn getBidirElements(
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut vl_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    vl_1 = getMatchingElements(
        vl,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isBidirElement(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(vl_1)
}

pub fn getInputVars(
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut vl_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    vl_1 = getMatchingElements(
        vl,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isInput(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(vl_1)
}

pub(crate) fn isFlowVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<()> {
    ::match_deref::match_deref! { match &((*inElement)) {
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::VARIABLE { .. }, connectorType: Deref @ DAE::ConnectorType::FLOW { .. }, .. } => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub(crate) fn isStreamVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<()> {
    ::match_deref::match_deref! { match &((*inElement)) {
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::VARIABLE { .. }, connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. } => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub(crate) fn isFlow(mut inFlow: &metamodelica::Ref<DAE::ConnectorType>) -> bool {
    let mut outIsFlow: bool;
    outIsFlow = (match &**inFlow {
        DAE::ConnectorType::FLOW { .. } => true,
        _ => false,
    });
    outIsFlow
}

pub(crate) fn isStream(mut inStream: &metamodelica::Ref<DAE::ConnectorType>) -> bool {
    let mut outIsStream: bool;
    outIsStream = (match &**inStream {
        DAE::ConnectorType::STREAM { .. } => true,
        _ => false,
    });
    outIsStream
}

pub fn isOutputVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::VAR {
            kind: DAE::VarKind::VARIABLE { .. },
            direction: DAE::VarDirection::OUTPUT { .. },
            ..
        } => true,
        _ => false,
    });
    outMatch
}

pub(crate) fn isOutputElement(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::VAR {
            direction: DAE::VarDirection::OUTPUT { .. },
            ..
        } => true,
        _ => false,
    });
    outMatch
}

pub(crate) fn assertProtectedVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<()> {
    let () = (match &**inElement {
        DAE::Element::VAR {
            protection: DAE::VarVisibility::PROTECTED { .. },
            ..
        } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub fn isProtectedVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut b: bool;
    b = (match &**inElement {
        DAE::Element::VAR {
            protection: DAE::VarVisibility::PROTECTED { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

pub fn isPublicVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::VAR {
            protection: DAE::VarVisibility::PUBLIC { .. },
            ..
        } => true,
        _ => false,
    });
    outMatch
}

pub(crate) fn isBidirVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::VAR {
            kind: DAE::VarKind::VARIABLE { .. },
            direction: DAE::VarDirection::BIDIR { .. },
            ..
        } => true,
        _ => false,
    });
    outMatch
}

pub(crate) fn isBidirElement(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::VAR {
            direction: DAE::VarDirection::BIDIR { .. },
            ..
        } => true,
        _ => false,
    });
    outMatch
}

pub fn isInputVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::VAR {
            kind: DAE::VarKind::VARIABLE { .. },
            direction: DAE::VarDirection::INPUT { .. },
            ..
        } => true,
        _ => false,
    });
    outMatch
}

pub fn isInput(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::VAR {
            direction: DAE::VarDirection::INPUT { .. },
            ..
        } => true,
        _ => false,
    });
    outMatch
}

pub fn isNotVar(mut e: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**e {
        DAE::Element::VAR { componentRef: _, .. } => false,
        _ => true,
    });
    outMatch
}

pub(crate) fn isVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::VAR { componentRef: _, .. } => true,
        _ => false,
    });
    outMatch
}

pub(crate) fn isFunctionRefVar(mut inElem: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inElem {
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_FUNCTION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isComment(mut elt: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut b: bool;
    b = (match &**elt {
        DAE::Element::COMMENT { cmt: _ } => true,
        _ => false,
    });
    b
}

pub fn isAlgorithm(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::ALGORITHM { algorithm_: _, .. } => true,
        _ => false,
    });
    outMatch
}

pub fn isStmtAssert(mut stmt: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut b: bool;
    b = (match &**stmt {
        DAE::Statement::STMT_ASSERT { cond: _, .. } => true,
        _ => false,
    });
    b
}

pub fn isStmtReturn(mut stmt: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut b: bool;
    b = (match &**stmt {
        DAE::Statement::STMT_RETURN { source: _ } => true,
        _ => false,
    });
    b
}

pub(crate) fn isStmtReinit(mut stmt: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut b: bool;
    b = (match &**stmt {
        DAE::Statement::STMT_REINIT { var: _, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isStmtTerminate(mut stmt: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut b: bool;
    b = (match &**stmt {
        DAE::Statement::STMT_TERMINATE { msg: _, .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isComplexEquation(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outMatch: bool;
    outMatch = (match &**inElement {
        DAE::Element::COMPLEX_EQUATION { lhs: _, .. } => true,
        _ => false,
    });
    outMatch
}

pub(crate) fn isFunctionInlineFalse(mut inElement: &DAE::Function) -> bool {
    let mut res: bool;
    res = (match inElement.clone() {
        DAE::Function::FUNCTION {
            inlineType: DAE::InlineType::NO_INLINE { .. },
            ..
        } => true,
        _ => false,
    });
    res
}

pub(crate) fn findElement(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inFuncTypeElementTo: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<()> + 'static>,
) -> Result<Option<metamodelica::Ref<DAE::Element>>> {
    pub type FuncTypeElementTo =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<()> + 'static>;

    let mut outElementOption: Option<metamodelica::Ref<DAE::Element>>;
    outElementOption = (::match_deref::match_deref! { match inElementLst {
        Deref @ metamodelica::ListNode::Nil => {
            None
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
            let mut f = inFuncTypeElementTo.clone();
            let mut e_1: Option<metamodelica::Ref<DAE::Element>> = None;
            e_1 = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            f(e.clone())?;
            Ok(Some(e.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut e_1: Option<metamodelica::Ref<DAE::Element>>;
            if '__try0: {
                unwrap_break_err!(f(e.clone()), '__try0);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            e_1 = findElement(rest, f.clone())?;
            Ok(e_1.clone())
        })() { break 'mc __v; }
        return Err("matchcontinue: no arm matched")
    };
            e_1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElementOption)
}

pub fn getVariableBindingsStr(mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut varlst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    r#str = (::match_deref::match_deref! { match &(elts.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { dAElist: __esc_els, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            els = (*__esc_els).clone();
            getVariableBindingsStr(els.clone())?
        },
        _ => {
            varlst = getVariableList(elts);
            r#str = getBindingsStr(&varlst)?;
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

fn getVariableList(
    mut inElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    let mut outElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    outElementLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
        for mut e in (inElementLst).into_iter().cloned() {
            if !(::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. } => false,
                Deref @ DAE::Element::VAR { .. } => true,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElementLst
}

pub fn getVariableType(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inElement {
        DAE::Element::VAR { ty: tp, .. } => tp.clone(),
        _ => return Err("fail"),
    });
    Ok(outType)
}

fn getBindingsStr(mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inElementLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { binding: Some(e), .. }, tail: lst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => {
            let mut expstr: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut r#str: ArcStr;
            expstr = ExpressionBasics::printExpStr(e.clone())?;
            s3 = stringAppend(expstr, literal!(","));
            s4 = getBindingsStr(metamodelica::AsArg::as_arg(&lst))?;
            r#str = stringAppend(s3, s4);
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { binding: None, .. }, tail: lst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => {
            let mut r#str: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = literal!("-,");
            s2 = getBindingsStr(metamodelica::AsArg::as_arg(&lst))?;
            r#str = stringAppend(s1, s2);
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { binding: Some(e), .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut r#str: ArcStr;
            r#str = ExpressionBasics::printExpStr(e.clone())?;
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { binding: None, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            literal!("")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub(crate) fn getBindings(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut outc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut oute: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    (outc, oute) = 'mc: {
        let __mc_input = &**inElementLst;
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
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, binding: Some(e), .. }, tail: rest } => {
                    let mut outc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = outc.clone();
                    let mut oute: metamodelica::List<metamodelica::Ref<DAE::Exp>> = oute.clone();
                    (outc, oute) = getBindings(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(((metamodelica::cons(cr.clone(), outc.clone()), metamodelica::cons(e.clone(), oute.clone())), outc.clone(), oute.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outc = __wb0;
            oute = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { binding: None, .. }, tail: rest } => {
                    let mut outc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = outc.clone();
                    let mut oute: metamodelica::List<metamodelica::Ref<DAE::Exp>> = oute.clone();
                    (outc, oute) = getBindings(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(((outc.clone(), oute.clone()), outc.clone(), oute.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outc = __wb0;
            oute = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!(" error in getBindings \n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outc, oute))
}

pub fn toConnectorType(
    mut inConnectorType: SCode::ConnectorType,
    mut inState: &ClassInf::State,
) -> metamodelica::Ref<DAE::ConnectorType> {
    let mut outConnectorType: metamodelica::Ref<DAE::ConnectorType>;
    outConnectorType = (match (inConnectorType, inState.clone()) {
        (SCode::ConnectorType::FLOW { .. }, _) => openmodelica_frontend_types::DAE::ConnectorType::interned_FLOW(),
        (SCode::ConnectorType::STREAM { .. }, _) => {
            metamodelica::Ref::new(DAE::ConnectorType::STREAM { associatedFlow: None })
        }
        (_, ClassInf::State::CONNECTOR { .. }) => openmodelica_frontend_types::DAE::ConnectorType::interned_POTENTIAL(),
        _ => openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
    });
    outConnectorType
}

pub fn toConnectorTypeNoState(
    mut scodeConnectorType: SCode::ConnectorType,
    mut flowName: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> metamodelica::Ref<DAE::ConnectorType> {
    let mut daeConnectorType: metamodelica::Ref<DAE::ConnectorType>;
    daeConnectorType = (match scodeConnectorType {
        SCode::ConnectorType::FLOW { .. } => openmodelica_frontend_types::DAE::ConnectorType::interned_FLOW(),
        SCode::ConnectorType::STREAM { .. } => metamodelica::Ref::new(DAE::ConnectorType::STREAM {
            associatedFlow: flowName,
        }),
        _ => openmodelica_frontend_types::DAE::ConnectorType::interned_POTENTIAL(),
    });
    daeConnectorType
}

pub fn toDaeParallelism(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inParallelism: SCode::Parallelism,
    mut inState: &ClassInf::State,
    mut inInfo: &SourceInfo,
) -> Result<DAE::VarParallelism> {
    let mut outParallelism: DAE::VarParallelism;
    outParallelism = 'mc: {
        let __mc_input = (inParallelism, inState.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (SCode::Parallelism::NON_PARALLEL { .. }, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (SCode::Parallelism::PARGLOBAL { .. }, ClassInf::State::FUNCTION { path: _, isImpure: _ }) =
                __mc_input.clone()
            else {
                return Err("nomatch");
            };
            Ok(openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (SCode::Parallelism::PARLOCAL { .. }, ClassInf::State::FUNCTION { path: _, isImpure: _ }) =
                __mc_input.clone()
            else {
                return Err("nomatch");
            };
            Ok(openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (SCode::Parallelism::PARGLOBAL { .. }, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut str1: ArcStr;
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = ClassInfUtil::getStateName(inState);
            str1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("- DAEUtil.toDaeParallelism: parglobal component '"));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref)?);
                __mm_s.push_str(&*literal!("' in non-function class: "));
                __mm_s.push_str(&*ClassInfUtil::printStateStr(inState));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessage(&(Error::PARMODELICA_WARNING.clone()), list![str1.clone()], inInfo)?;
            Ok(openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (SCode::Parallelism::PARLOCAL { .. }, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut str1: ArcStr;
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = ClassInfUtil::getStateName(inState);
            str1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("- DAEUtil.toDaeParallelism: parlocal component '"));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref)?);
                __mm_s.push_str(&*literal!("' in non-function class: "));
                __mm_s.push_str(&*ClassInfUtil::printStateStr(inState));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessage(&(Error::PARMODELICA_WARNING.clone()), list![str1.clone()], inInfo)?;
            Ok(openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL)
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outParallelism)
}

pub fn scodePrlToDaePrl(mut inParallelism: SCode::Parallelism) -> DAE::VarParallelism {
    let mut outVarParallelism: DAE::VarParallelism;
    outVarParallelism = (match inParallelism {
        SCode::Parallelism::NON_PARALLEL { .. } => openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        SCode::Parallelism::PARGLOBAL { .. } => openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL,
        SCode::Parallelism::PARLOCAL { .. } => openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL,
    });
    outVarParallelism
}

pub fn daeParallelismEqual(mut inParallelism1: DAE::VarParallelism, mut inParallelism2: DAE::VarParallelism) -> bool {
    let mut equal: bool;
    equal = (match (inParallelism1, inParallelism2) {
        (DAE::VarParallelism::NON_PARALLEL { .. }, DAE::VarParallelism::NON_PARALLEL { .. }) => true,
        (DAE::VarParallelism::PARGLOBAL { .. }, DAE::VarParallelism::PARGLOBAL { .. }) => true,
        (DAE::VarParallelism::PARLOCAL { .. }, DAE::VarParallelism::PARLOCAL { .. }) => true,
        _ => false,
    });
    equal
}

pub(crate) fn getFlowVariables(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outExpComponentRefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outExpComponentRefLst = 'mc: {
        let __mc_input = &**inElementLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, connectorType: Deref @ DAE::ConnectorType::FLOW { .. }, .. }, tail: xs } => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    res = getFlowVariables(metamodelica::AsArg::as_arg(&xs));
                    Ok(metamodelica::cons(cr.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { ident: id, dAElist: lst, .. }, tail: xs } => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut res1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut res1_1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut res2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    res1 = getFlowVariables(metamodelica::AsArg::as_arg(&lst));
                    res1_1 = getFlowVariables2(&res1, id.clone())?;
                    res2 = getFlowVariables(metamodelica::AsArg::as_arg(&xs));
                    res = listAppend(res1_1.clone(), res2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    res = getFlowVariables(metamodelica::AsArg::as_arg(&xs));
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExpComponentRefLst
}

fn getFlowVariables2(
    mut inExpComponentRefLst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outExpComponentRefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outExpComponentRefLst = (::match_deref::match_deref! { match inExpComponentRefLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: cr, tail: xs } => {
            let mut id = inIdent;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            res = getFlowVariables2(xs, id.clone())?;
            cr_1 = ComponentReferenceBasics::makeCrefQual(id, DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), cr.clone());
            metamodelica::cons(cr_1, res)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpComponentRefLst)
}

pub(crate) fn getStreamVariables(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outExpComponentRefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outExpComponentRefLst = 'mc: {
        let __mc_input = &**inElementLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. }, tail: xs } => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    res = getStreamVariables(metamodelica::AsArg::as_arg(&xs));
                    Ok(metamodelica::cons(cr.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { ident: id, dAElist: lst, .. }, tail: xs } => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut res1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut res1_1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut res2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    res1 = getStreamVariables(metamodelica::AsArg::as_arg(&lst));
                    res1_1 = getStreamVariables2(&res1, id.clone())?;
                    res2 = getStreamVariables(metamodelica::AsArg::as_arg(&xs));
                    res = listAppend(res1_1.clone(), res2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    res = getStreamVariables(metamodelica::AsArg::as_arg(&xs));
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExpComponentRefLst
}

fn getStreamVariables2(
    mut inExpComponentRefLst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outExpComponentRefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outExpComponentRefLst = (::match_deref::match_deref! { match inExpComponentRefLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: cr, tail: xs } => {
            let mut id = inIdent;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            res = getStreamVariables2(xs, id.clone())?;
            cr_1 = ComponentReferenceBasics::makeCrefQual(id, DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), cr.clone());
            metamodelica::cons(cr_1, res)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpComponentRefLst)
}

pub(crate) fn toModelicaForm(mut inDAElist: &DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDAElist: DAE::DAElist;
    outDAElist = (match inDAElist.clone() {
        DAE::DAElist { elementLst: ref elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            elts_1 = toModelicaFormElts(metamodelica::AsArg::as_arg(&elts))?;
            DAE::DAElist { elementLst: elts_1 }
        }
    });
    Ok(outDAElist)
}

fn toModelicaFormElts(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    outElementLst = (::match_deref::match_deref! { match inElementLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, kind: a, direction: b, parallelism: prl, protection: prot, ty: t, binding: d, dims: instDim, connectorType: ct, source, variableAttributesOption: dae_var_attr, comment, innerOuter: io, encrypted }, tail: elts } => {
            let mut r#str: ArcStr;
            let mut str_1: ArcStr;
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut d_1: Option<metamodelica::Ref<DAE::Exp>>;
            let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            r#str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            str_1 = Util::stringReplaceChar(r#str, literal!("."), literal!("_"))?;
            elts_1 = toModelicaFormElts(elts)?;
            d_1 = toModelicaFormExpOpt(d.clone());
            ty = ComponentReference::crefLastType(metamodelica::AsArg::as_arg(&cr))?;
            cref_ = ComponentReferenceBasics::makeCrefIdent(str_1, ty, metamodelica::nil());
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::VAR { componentRef: cref_, kind: a.clone(), direction: b.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: t.clone(), binding: d_1, dims: instDim.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: dae_var_attr.clone(), comment: comment.clone(), innerOuter: io.clone(), encrypted: encrypted.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            e_1 = toModelicaFormExp(e.clone());
            cr_1 = toModelicaFormCref(metamodelica::AsArg::as_arg(&cr))?;
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::DEFINE { componentRef: cr_1, exp: e_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIALDEFINE { componentRef: cr, exp: e, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            e_1 = toModelicaFormExp(e.clone());
            cr_1 = toModelicaFormCref(metamodelica::AsArg::as_arg(&cr))?;
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIALDEFINE { componentRef: cr_1, exp: e_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone());
            e2_1 = toModelicaFormExp(e2.clone());
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone());
            e2_1 = toModelicaFormExp(e2.clone());
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::COMPLEX_EQUATION { lhs: e1_1, rhs: e2_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone());
            e2_1 = toModelicaFormExp(e2.clone());
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1_1, rhs: e2_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut cr1 = (*cr1).clone();
            let mut cr2 = (*cr2).clone();
            let __pa0 = ::match_deref::match_deref! { match &(toModelicaFormExp(Expression::crefExp(cr1.clone())?)) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr1 = metamodelica::Own::own(__pa0);
            let __pa1 = ::match_deref::match_deref! { match &(toModelicaFormExp(Expression::crefExp(cr2.clone())?)) {
                Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ } => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr2 = metamodelica::Own::own(__pa1);
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::EQUEQUATION { cr1: cr1.clone(), cr2: cr2.clone(), source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { condition: e1, equations: welts, elsewhen_: Some(elt), source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut welts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut elt_1: metamodelica::Ref<DAE::Element>;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone());
            welts_1 = toModelicaFormElts(metamodelica::AsArg::as_arg(&welts))?;
            let __pa0 = ::match_deref::match_deref! { match &(toModelicaFormElts(&(list![elt.clone()]))?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            elt_1 = metamodelica::Own::own(__pa0);
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: e1_1, equations: welts_1, elsewhen_: Some(elt_1), source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { condition: e1, equations: welts, elsewhen_: None, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut welts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone());
            welts_1 = toModelicaFormElts(metamodelica::AsArg::as_arg(&welts))?;
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: e1_1, equations: welts_1, elsewhen_: None, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: conds, equations2: trueBranches, equations3: eelts, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut eelts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut conds_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut trueBranches_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
            conds_1 = List::map(conds.clone(), &fnptr!(toModelicaFormExp, metamodelica::Ref<DAE::Exp>))?;
            trueBranches_1 = List::map(trueBranches.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Element>>| toModelicaFormElts(&__a0))?;
            eelts_1 = toModelicaFormElts(metamodelica::AsArg::as_arg(&eelts))?;
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::IF_EQUATION { condition1: conds_1, equations2: trueBranches_1, equations3: eelts_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: conds, equations2: trueBranches, equations3: eelts, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut eelts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut conds_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut trueBranches_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
            conds_1 = List::map(conds.clone(), &fnptr!(toModelicaFormExp, metamodelica::Ref<DAE::Exp>))?;
            trueBranches_1 = List::map(trueBranches.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Element>>| toModelicaFormElts(&__a0))?;
            eelts_1 = toModelicaFormElts(metamodelica::AsArg::as_arg(&eelts))?;
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIAL_IF_EQUATION { condition1: conds_1, equations2: trueBranches_1, equations3: eelts_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            e1_1 = toModelicaFormExp(e1.clone());
            e2_1 = toModelicaFormExp(e2.clone());
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIALEQUATION { exp1: e1_1, exp2: e2_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: alg, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            metamodelica::print(literal!("to_modelica_form_elts(ALGORITHM) not impl. yet\n"));
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: alg.clone(), source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIALALGORITHM { algorithm_: alg, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            metamodelica::print(literal!("to_modelica_form_elts(INITIALALGORITHM) not impl. yet\n"));
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIALALGORITHM { algorithm_: alg.clone(), source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { ident: id, dAElist: elts2, source, comment }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut elts2 = (*elts2).clone();
            elts2 = toModelicaFormElts(metamodelica::AsArg::as_arg(&elts2))?;
            elts_1 = toModelicaFormElts(elts)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::COMP { ident: id.clone(), dAElist: elts2.clone(), source: source.clone(), comment: comment.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ASSERT { condition: e1, message: e2, level: e3, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut e_2: metamodelica::Ref<DAE::Exp>;
            let mut e_3: metamodelica::Ref<DAE::Exp>;
            elts_1 = toModelicaFormElts(elts)?;
            e_1 = toModelicaFormExp(e1.clone());
            e_2 = toModelicaFormExp(e2.clone());
            e_3 = toModelicaFormExp(e3.clone());
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::ASSERT { condition: e_1, message: e_2, level: e_3, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_ASSERT { condition: e1, message: e2, level: e3, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut e_2: metamodelica::Ref<DAE::Exp>;
            let mut e_3: metamodelica::Ref<DAE::Exp>;
            elts_1 = toModelicaFormElts(elts)?;
            e_1 = toModelicaFormExp(e1.clone());
            e_2 = toModelicaFormExp(e2.clone());
            e_3 = toModelicaFormExp(e3.clone());
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIAL_ASSERT { condition: e_1, message: e_2, level: e_3, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::TERMINATE { message: e1, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            elts_1 = toModelicaFormElts(elts)?;
            e_1 = toModelicaFormExp(e1.clone());
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::TERMINATE { message: e_1, source: source.clone() }), elts_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, source }, tail: elts } => {
            let mut elts_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            elts_1 = toModelicaFormElts(elts)?;
            e_1 = toModelicaFormExp(e1.clone());
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIAL_TERMINATE { message: e_1, source: source.clone() }), elts_1)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outElementLst)
}

pub fn replaceCrefInVar(
    mut newCr: metamodelica::Ref<DAE::ComponentRef>,
    mut inelem: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outelem: metamodelica::Ref<DAE::Element>;
    outelem = (match &**inelem {
        DAE::Element::VAR {
            componentRef: _,
            kind: a2,
            direction: a3,
            parallelism: prl,
            protection: a4,
            ty: a5,
            binding: a6,
            dims: a7,
            connectorType: ct,
            source,
            variableAttributesOption: a11,
            comment: a12,
            innerOuter: a13,
            encrypted: e,
        } => metamodelica::Ref::new(DAE::Element::VAR {
            componentRef: newCr,
            kind: a2.clone(),
            direction: a3.clone(),
            parallelism: prl.clone(),
            protection: a4.clone(),
            ty: a5.clone(),
            binding: a6.clone(),
            dims: a7.clone(),
            connectorType: ct.clone(),
            source: source.clone(),
            variableAttributesOption: a11.clone(),
            comment: a12.clone(),
            innerOuter: a13.clone(),
            encrypted: e.clone(),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outelem)
}

pub fn replaceTypeInVar(
    mut newType: metamodelica::Ref<DAE::Type>,
    mut inelem: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outelem: metamodelica::Ref<DAE::Element>;
    outelem = (match &**inelem {
        DAE::Element::VAR {
            componentRef: a1,
            kind: a2,
            direction: a3,
            parallelism: prl,
            protection: a4,
            ty: _,
            binding: a6,
            dims: a7,
            connectorType: ct,
            source,
            variableAttributesOption: a11,
            comment: a12,
            innerOuter: a13,
            encrypted: e,
        } => metamodelica::Ref::new(DAE::Element::VAR {
            componentRef: a1.clone(),
            kind: a2.clone(),
            direction: a3.clone(),
            parallelism: prl.clone(),
            protection: a4.clone(),
            ty: newType,
            binding: a6.clone(),
            dims: a7.clone(),
            connectorType: ct.clone(),
            source: source.clone(),
            variableAttributesOption: a11.clone(),
            comment: a12.clone(),
            innerOuter: a13.clone(),
            encrypted: e.clone(),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outelem)
}

pub fn replaceCrefandTypeInVar(
    mut newCr: metamodelica::Ref<DAE::ComponentRef>,
    mut newType: metamodelica::Ref<DAE::Type>,
    mut inelem: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outelem: metamodelica::Ref<DAE::Element>;
    outelem = (match &**inelem {
        DAE::Element::VAR {
            componentRef: _,
            kind: a2,
            direction: a3,
            parallelism: prl,
            protection: a4,
            ty: _,
            binding: a6,
            dims: a7,
            connectorType: ct,
            source,
            variableAttributesOption: a11,
            comment: a12,
            innerOuter: a13,
            encrypted: e,
        } => {
            outelem = metamodelica::Ref::new(DAE::Element::VAR {
                componentRef: newCr,
                kind: a2.clone(),
                direction: a3.clone(),
                parallelism: prl.clone(),
                protection: a4.clone(),
                ty: newType,
                binding: a6.clone(),
                dims: a7.clone(),
                connectorType: ct.clone(),
                source: source.clone(),
                variableAttributesOption: a11.clone(),
                comment: a12.clone(),
                innerOuter: a13.clone(),
                encrypted: e.clone(),
            });
            outelem
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outelem)
}

pub fn replaceBindungInVar(
    mut newBindung: metamodelica::Ref<DAE::Exp>,
    mut inelem: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outelem: metamodelica::Ref<DAE::Element>;
    outelem = (match &**inelem {
        DAE::Element::VAR {
            componentRef: a1,
            kind: a2,
            direction: a3,
            parallelism: prl,
            protection: a4,
            ty: a5,
            binding: _,
            dims: a7,
            connectorType: ct,
            source,
            variableAttributesOption: a11,
            comment: a12,
            innerOuter: a13,
            encrypted: e,
        } => metamodelica::Ref::new(DAE::Element::VAR {
            componentRef: a1.clone(),
            kind: a2.clone(),
            direction: a3.clone(),
            parallelism: prl.clone(),
            protection: a4.clone(),
            ty: a5.clone(),
            binding: Some(newBindung),
            dims: a7.clone(),
            connectorType: ct.clone(),
            source: source.clone(),
            variableAttributesOption: a11.clone(),
            comment: a12.clone(),
            innerOuter: a13.clone(),
            encrypted: e.clone(),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outelem)
}

fn toModelicaFormExpOpt(
    mut inExpExpOption: Option<metamodelica::Ref<DAE::Exp>>,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut outExpExpOption: Option<metamodelica::Ref<DAE::Exp>>;
    outExpExpOption = (::match_deref::match_deref! { match &(inExpExpOption) {
        Some(e) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            e_1 = toModelicaFormExp(e.clone());
            Some(e_1)
        },
        None => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExpExpOption
}

fn toModelicaFormCref(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut r#str: ArcStr;
    let mut str_1: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    r#str = ComponentReferenceBasics::printComponentRefStr(cr)?;
    ty = ComponentReference::crefLastType(cr)?;
    str_1 = Util::stringReplaceChar(r#str, literal!("."), literal!("_"))?;
    outComponentRef = ComponentReferenceBasics::makeCrefIdent(str_1, ty, metamodelica::nil());
    Ok(outComponentRef)
}

fn toModelicaFormExp(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty: t } => {
                    let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
                    cr_1 = toModelicaFormCref(metamodelica::AsArg::as_arg(&cr))?;
                    Ok(Expression::makeCrefExp(cr_1.clone(), t.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    e1_1 = toModelicaFormExp(e1.clone());
                    e2_1 = toModelicaFormExp(e2.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    e1_1 = toModelicaFormExp(e1.clone());
                    e2_1 = toModelicaFormExp(e2.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, exp: e } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    e_1 = toModelicaFormExp(e.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, exp: e } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    e_1 = toModelicaFormExp(e.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index: i, optionExpisASUB } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    e1_1 = toModelicaFormExp(e1.clone());
                    e2_1 = toModelicaFormExp(e2.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone(), index: i.clone(), optionExpisASUB: optionExpisASUB.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3_1: metamodelica::Ref<DAE::Exp>;
                    e1_1 = toModelicaFormExp(e1.clone());
                    e2_1 = toModelicaFormExp(e2.clone());
                    e3_1 = toModelicaFormExp(e3.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1_1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: f, expLst: expl, attr } => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expl_1 = List::map(expl.clone(), &fnptr!(toModelicaFormExp, metamodelica::Ref<DAE::Exp>))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::CALL { path: f.clone(), expLst: expl_1.clone(), attr: attr.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: t, scalar: b, array: expl } => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expl_1 = List::map(expl.clone(), &fnptr!(toModelicaFormExp, metamodelica::Ref<DAE::Exp>))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: b.clone(), array: expl_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expl } => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expl_1 = List::map(expl.clone(), &fnptr!(toModelicaFormExp, metamodelica::Ref<DAE::Exp>))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expl_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: t, exp: e } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    e_1 = toModelicaFormExp(e.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::CAST { ty: t.clone(), exp: e_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::ASUB { exp: e, sub: subs } => {
                            let mut e_1: metamodelica::Ref<DAE::Exp>;
                            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            e_1 = toModelicaFormExp(e.clone());
                            expl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok(Expression::makeASUB(e_1.clone(), expl.clone())?)
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: e, sz: eopt } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut eopt_1: Option<metamodelica::Ref<DAE::Exp>>;
                    e_1 = toModelicaFormExp(e.clone());
                    eopt_1 = toModelicaFormExpOpt(eopt.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::SIZE { exp: e_1.clone(), sz: eopt_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

pub fn getNamedFunction(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut functions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<DAE::Function> {
    let mut outElement: DAE::Function;
    outElement = 'mc: {
        let __mc_input = &**functions;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Util::getOption(AvlTreePathFunction::get(functions, path.clone())?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut msg: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    msg = stringDelimitList(List::mapMap(getFunctionList(functions, false)?, &move |__a0: DAE::Function| -> metamodelica::Result<_> { ::std::result::Result::Ok(functionName(&__a0)) }, &AbsynUtil::pathStringDefault)?, literal!("\n  "));
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.getNamedFunction failed: ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("\nThe following functions were part of the cache:\n  ")); __mm_s.push_str(&*msg); ArcStr::from(__mm_s) };
                    Debug::traceln(msg.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElement)
}

pub fn getNamedFunctionWithError(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut functions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut info: &SourceInfo,
) -> Result<DAE::Function> {
    let mut outElement: DAE::Function;
    outElement = 'mc: {
        let __mc_input = info.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(Util::getOption(AvlTreePathFunction::get(functions, path.clone())?)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut msg: ArcStr;
            msg = stringDelimitList(
                List::mapMap(
                    getFunctionList(functions, false)?,
                    &move |__a0: DAE::Function| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(functionName(&__a0))
                    },
                    &AbsynUtil::pathStringDefault,
                )?,
                literal!("\n  "),
            );
            msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("DAEUtil.getNamedFunction failed: "));
                __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
                __mm_s.push_str(&*literal!("\nThe following functions were part of the cache:\n  "));
                __mm_s.push_str(&*msg);
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![msg.clone()], info)?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElement)
}

pub(crate) fn getNamedFunctionFromList(
    mut ipath: metamodelica::Ref<Absyn::Path>,
    mut ifns: &metamodelica::List<DAE::Function>,
) -> Result<DAE::Function> {
    let mut r#fn: DAE::Function;
    r#fn = 'mc: {
        let __mc_input = (ipath, &**ifns);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ metamodelica::ListNode::Cons { head: r#fn, tail: _ }) => {
                    let true = (AbsynUtil::pathEqual(&(functionName(metamodelica::AsArg::as_arg(&r#fn))), metamodelica::AsArg::as_arg(&path))) else { return Err("pattern mismatch") };
                    Ok(r#fn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ metamodelica::ListNode::Cons { head: _, tail: fns }) => {
                    Ok(getNamedFunctionFromList(path.clone(), metamodelica::AsArg::as_arg(&fns))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ metamodelica::ListNode::Nil) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- DAEUtil.getNamedFunctionFromList failed ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(r#fn)
}

pub fn getFunctionVisibility(mut r#fn: &DAE::Function) -> SCode::Visibility {
    let mut visibility: SCode::Visibility;
    visibility = (match r#fn.clone() {
        DAE::Function::FUNCTION {
            visibility: mut __esc_visibility,
            ..
        } => {
            visibility = __esc_visibility.clone();
            visibility
        }
        _ => openmodelica_frontend_types::SCode::Visibility::PUBLIC,
    });
    visibility
}

fn getFunctionsElements(
    mut elements: metamodelica::List<DAE::Function>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elsList: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
    elsList = List::map(elements, &move |__a0: DAE::Function| getFunctionElements(&__a0))?;
    els = List::flatten(elsList)?;
    Ok(els)
}

pub fn getFunctionElements(mut r#fn: &DAE::Function) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    els = (::match_deref::match_deref! { match &(r#fn) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: elements }, tail: _ }, .. } => {
            elements.clone()
        },
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { body: elements, .. }, tail: _ }, .. } => {
            elements.clone()
        },
        DAE::Function::RECORD_CONSTRUCTOR { .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(els)
}

pub fn getFunctionType(mut r#fn: &DAE::Function) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match r#fn.clone() {
        DAE::Function::FUNCTION {
            type_: ref __esc_outType,
            ..
        } => {
            outType = __esc_outType.clone();
            outType.clone()
        }
        DAE::Function::FUNCTION {
            type_: ref __esc_outType,
            ..
        } => {
            outType = __esc_outType.clone();
            outType.clone()
        }
        DAE::Function::RECORD_CONSTRUCTOR {
            type_: ref __esc_outType,
            ..
        } => {
            outType = __esc_outType.clone();
            outType.clone()
        }
    });
    outType
}

pub fn getFunctionImpureAttribute(mut r#fn: &DAE::Function) -> Result<bool> {
    let mut outImpure: bool;
    outImpure = (match r#fn.clone() {
        DAE::Function::FUNCTION {
            isImpure: mut __esc_outImpure,
            ..
        } => {
            outImpure = __esc_outImpure.clone();
            outImpure
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outImpure)
}

pub fn getFunctionInlineType(mut r#fn: &DAE::Function) -> Result<DAE::InlineType> {
    let mut outInlineType: DAE::InlineType;
    outInlineType = (match r#fn.clone() {
        DAE::Function::FUNCTION {
            inlineType: mut __esc_outInlineType,
            ..
        } => {
            outInlineType = __esc_outInlineType.clone();
            outInlineType
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outInlineType)
}

pub fn getFunctionInputVars(mut r#fn: &DAE::Function) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outEls: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    elements = getFunctionElements(r#fn)?;
    outEls = List::filterOnTrue(
        elements,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isInputVar(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(outEls)
}

pub fn getFunctionOutputVars(mut r#fn: &DAE::Function) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outEls: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    elements = getFunctionElements(r#fn)?;
    outEls = List::filterOnTrue(
        elements,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isOutputElement(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(outEls)
}

pub fn getFunctionProtectedVars(
    mut r#fn: &DAE::Function,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outEls: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    elements = getFunctionElements(r#fn)?;
    outEls = List::filterOnTrue(
        elements,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isProtectedVar(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(outEls)
}

pub(crate) fn getFunctionAlgorithms(
    mut r#fn: &DAE::Function,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outEls: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    elements = getFunctionElements(r#fn)?;
    outEls = List::filterOnTrue(
        elements,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isAlgorithm(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(outEls)
}

pub fn getFunctionAlgorithmStmts(
    mut r#fn: &DAE::Function,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut bodyStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    elements = getFunctionElements(r#fn)?;
    bodyStmts = List::mapFlat(
        &(List::filterOnTrue(
            elements,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isAlgorithm(&__a0))
                },
            )
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
        )?),
        &move |__a0: metamodelica::Ref<DAE::Element>| getStatement(&__a0),
    )?;
    Ok(bodyStmts)
}

pub fn getStatement(
    mut inElement: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outStatements = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            stmts.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("- Differentiatte.getStatement failed\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStatements)
}

pub fn getTupleSize(mut inExp: &metamodelica::Ref<DAE::Exp>) -> i32 {
    let mut size: i32;
    size = (match &**inExp {
        DAE::Exp::TUPLE { PR: exps } => {
            size = ((exps).len() as i32);
            size
        }
        _ => 0,
    });
    size
}

pub fn getTupleExps(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    exps = (match &*inExp {
        DAE::Exp::TUPLE { PR: __esc_exps } => {
            exps = (*__esc_exps).clone();
            exps.clone()
        }
        _ => list![inExp],
    });
    exps
}

fn crefToExp(mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = Expression::makeCrefExp(inComponentRef, DAE::T_UNKNOWN_DEFAULT().clone())?;
    Ok(outExp)
}

pub fn verifyEquationsDAE(mut dae: DAE::DAElist) -> Result<()> {
    let mut cond: metamodelica::Ref<DAE::Exp>;
    let mut dae_elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut ew: Option<metamodelica::Ref<DAE::Element>>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut el: metamodelica::Ref<DAE::Element> =
        <metamodelica::Ref<DAE::Element> as ::std::default::Default>::default();
    let mut info: SourceInfo;
    let DAE::DAE { elementLst: __pa0 } = dae;
    dae_elts = metamodelica::Own::own(__pa0);
    for mut el in &*dae_elts {
        let mut el = el.clone();
        let () = (match &*el {
            DAE::Element::WHEN_EQUATION {
                condition: __esc_cond,
                equations: __esc_eqs,
                elsewhen_: __esc_ew,
                source: __esc_source,
            } => {
                cond = (*__esc_cond).clone();
                eqs = (*__esc_eqs).clone();
                ew = (*__esc_ew).clone();
                source = (*__esc_source).clone();
                verifyWhenEquation(cond.clone(), eqs.clone(), ew.clone(), source.clone())?;
                ()
            }
            DAE::Element::REINIT { .. } => {
                info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(&el)?);
                Error::addSourceMessageAndFail(&(Error::REINIT_NOT_IN_WHEN.clone()), metamodelica::nil(), &info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

fn verifyWhenEquation(
    mut cond: metamodelica::Ref<DAE::Exp>,
    mut eqs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut ew: Option<metamodelica::Ref<DAE::Element>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<()> {
    if Types::isClockOrSubTypeClock(Expression::r#typeof(cond.clone())?) {
        verifyClockWhenEquation(&cond, &eqs, ew, source)?;
    } else {
        verifyBoolWhenEquation(&cond, eqs, ew, source)?;
    }
    Ok(())
}

fn verifyClockWhenEquation(
    mut cond: &metamodelica::Ref<DAE::Exp>,
    mut eqs: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut ew: Option<metamodelica::Ref<DAE::Element>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<()> {
    let mut info: SourceInfo;
    if (ew).is_some() {
        info = ElementSource::getElementSourceFileInfo(source);
        Error::addSourceMessageAndFail(&(Error::ELSE_WHEN_CLOCK.clone()), metamodelica::nil(), &info)?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    verifyClockWhenEquation1(eqs)?;
    Ok(())
}

fn verifyClockWhenEquation1(mut inEqs: &metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<()> {
    let mut el: metamodelica::Ref<DAE::Element> =
        <metamodelica::Ref<DAE::Element> as ::std::default::Default>::default();
    for mut el in &**inEqs {
        let mut el = el.clone();
        let () = (match &*el.clone() {
            DAE::Element::REINIT { .. } => {
                let mut info: SourceInfo;
                info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(&el)?);
                Error::addSourceMessageAndFail(&(Error::REINIT_NOT_IN_WHEN.clone()), metamodelica::nil(), &info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                ()
            }
            DAE::Element::WHEN_EQUATION {
                condition: cond,
                equations: eqs,
                elsewhen_: ew,
                source,
            } => {
                let mut info: SourceInfo;
                if Types::isClockOrSubTypeClock(Expression::r#typeof(cond.clone())?) {
                    info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(&el)?);
                    Error::addSourceMessageAndFail(&(Error::NESTED_CLOCKED_WHEN.clone()), metamodelica::nil(), &info)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
                verifyBoolWhenEquation(
                    metamodelica::AsArg::as_arg(&cond),
                    eqs.clone(),
                    ew.clone(),
                    source.clone(),
                )?;
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

fn verifyBoolWhenEquation(
    mut inCond: &metamodelica::Ref<DAE::Exp>,
    mut inEqs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inElseWhen: Option<metamodelica::Ref<DAE::Element>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<()> {
    let mut crefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut whenBranches: metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    )>;
    let mut whenBranch: (
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    ) = (
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default(),
        metamodelica::nil(),
    );
    let mut cond: metamodelica::Ref<DAE::Exp>;
    let mut eqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut info: SourceInfo;
    crefs1 = verifyBoolWhenEquationBranch(inCond, inEqs)?;
    whenBranches = collectWhenEquationBranches(inElseWhen, metamodelica::nil())?;
    for mut whenBranch in &*whenBranches {
        let mut whenBranch = whenBranch.clone();
        (cond, eqs) = whenBranch;
        if Types::isClockOrSubTypeClock(Expression::r#typeof(cond.clone())?) {
            info = ElementSource::getElementSourceFileInfo(source.clone());
            Error::addSourceMessageAndFail(&(Error::CLOCKED_WHEN_BRANCH.clone()), metamodelica::nil(), &info)?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        crefs2 = verifyBoolWhenEquationBranch(&cond, eqs)?;
        crefs2 = List::unionOnTrue(
            &crefs1,
            &crefs2,
            &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::crefEqual(&__a0, &__a1)
            },
        )?;
        if ((crefs2).len() as i32) != ((crefs1).len() as i32) {
            info = ElementSource::getElementSourceFileInfo(source.clone());
            Error::addSourceMessageAndFail(
                &(Error::DIFFERENT_VARIABLES_SOLVED_IN_ELSEWHEN.clone()),
                metamodelica::nil(),
                &info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok(())
}

fn collectWhenEquationBranches(
    mut inElseWhen: Option<metamodelica::Ref<DAE::Element>>,
    mut inWhenBranches: metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    )>,
> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inElseWhen) {
            None => {
                return Ok(inWhenBranches)
            },
            Some(Deref @ DAE::Element::WHEN_EQUATION { condition: cond, equations: eqs, elsewhen_: ew, source: _ }) => {
                { (inElseWhen, inWhenBranches) = (ew.clone(), metamodelica::cons((cond.clone(), eqs.clone()), inWhenBranches)); continue '__tco; }
            },
            Some(el) => {
                let mut info: SourceInfo;
                let mut msg: ArcStr;
                msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- DAEUtil.collectWhenEquationBranches failed on: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![el.clone()]))?); ArcStr::from(__mm_s) };
                info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(metamodelica::AsArg::as_arg(&el))?);
                Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![msg], &info)?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn verifyBoolWhenEquationBranch(
    mut inCond: &metamodelica::Ref<DAE::Exp>,
    mut inEqs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut initCond: bool = Expression::containsInitialCall(inCond)?;
    crefs = verifyBoolWhenEquation1(inEqs, initCond, metamodelica::nil())?;
    Ok(crefs)
}

fn verifyBoolWhenEquation1(
    mut inElems: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut initCond: bool,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inElems) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inCrefs)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { .. }, tail: rest } => {
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, inCrefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, .. }, tail: rest } => {
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, metamodelica::cons(cr.clone(), inCrefs)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: e, source, .. }, tail: rest } => {
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                crefs = collectWhenCrefs1(e.clone(), source.clone(), inCrefs)?;
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, crefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { exp: e, source, .. }, tail: rest } => {
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                crefs = collectWhenCrefs1(e.clone(), source.clone(), inCrefs)?;
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, crefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e, source, .. }, tail: rest } => {
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                crefs = collectWhenCrefs1(e.clone(), source.clone(), inCrefs)?;
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, crefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cr, .. }, tail: rest } => {
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, metamodelica::cons(cr.clone(), inCrefs)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { equations2: trueEqs, equations3: falseEqs, source, .. }, tail: rest } => {
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut crefsLists: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
                let mut info: SourceInfo;
                let mut b: bool;
                let mut msg: ArcStr;
                crefsLists = List::map2(trueEqs.clone(), &verifyBoolWhenEquation1, initCond, metamodelica::nil())?;
                crefs = verifyBoolWhenEquation1(falseEqs.clone(), initCond, metamodelica::nil())?;
                crefsLists = metamodelica::cons(crefs, crefsLists);
                (crefs, b) = compareCrefList(&crefsLists)?;
                if !(b) {
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    msg = literal!("All branches must write to the same variable");
                    Error::addSourceMessage(&(Error::WHEN_EQ_LHS.clone()), list![msg], &info)?;
                    return Err("fail");
                }
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, listAppend(crefs, inCrefs)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ASSERT { .. }, tail: rest } => {
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, inCrefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::TERMINATE { .. }, tail: rest } => {
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, inCrefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::REINIT { source, .. }, tail: rest } => {
                let mut info: SourceInfo;
                if initCond {
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Error::addSourceMessage(&(Error::REINIT_IN_WHEN_INITIAL.clone()), metamodelica::nil(), &info)?;
                    return Err("fail");
                }
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, inCrefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::NORETCALL { .. }, tail: rest } => {
                { (inElems, initCond, inCrefs) = (rest.clone(), initCond, inCrefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { condition: e, source, .. }, tail: _ } => {
                let mut info: SourceInfo;
                info = ElementSource::getElementSourceFileInfo(source.clone());
                if Types::isClockOrSubTypeClock(Expression::r#typeof(e.clone())?) {
                    Error::addSourceMessage(&(Error::CLOCKED_WHEN_IN_WHEN_EQ.clone()), metamodelica::nil(), &info)?;
                } else {
                    Error::addSourceMessage(&(Error::NESTED_WHEN.clone()), metamodelica::nil(), &info)?;
                }
                return Ok(return Err("fail"))
            },
            Deref @ metamodelica::ListNode::Cons { head: el, tail: _ } => {
                let mut info: SourceInfo;
                let mut msg: ArcStr;
                msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- DAEUtil.verifyWhenEquationStatements failed on: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![el.clone()]))?); ArcStr::from(__mm_s) };
                info = ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(metamodelica::AsArg::as_arg(&el))?);
                Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![msg], &info)?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn collectWhenCrefs(
    mut inExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outCrefs = List::fold1(inExps, &collectWhenCrefs1, source, inCrefs)?;
    Ok(outCrefs)
}

fn collectWhenCrefs1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    outCrefs = (match &*inExp {
        DAE::Exp::CREF {
            componentRef: __esc_cr,
            ty: _,
        } => {
            cr = (*__esc_cr).clone();
            metamodelica::cons(cr.clone(), inCrefs)
        }
        DAE::Exp::TUPLE { PR: __esc_exps } => {
            exps = (*__esc_exps).clone();
            collectWhenCrefs(metamodelica::AsArg::as_arg(&exps), source, inCrefs)?
        }
        _ => {
            let mut msg: ArcStr;
            let mut info: SourceInfo;
            msg = ExpressionBasics::printExpStr(inExp)?;
            info = ElementSource::getElementSourceFileInfo(source);
            Error::addSourceMessage(&(Error::WHEN_EQ_LHS.clone()), list![msg], &info)?;
            return Err("fail");
        }
    });
    Ok(outCrefs)
}

fn compareCrefList(
    mut inCrefs: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, bool)> {
    let mut outrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut matching: bool;
    (outrefs, matching) = (::match_deref::match_deref! { match inCrefs {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), true)
        },
        Deref @ metamodelica::ListNode::Cons { head: crefs, tail: Deref @ metamodelica::ListNode::Nil } => {
            (crefs.clone(), true)
        },
        Deref @ metamodelica::ListNode::Cons { head: crefs, tail: llrefs } => {
            let mut recRefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut i: i32;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut crefs = (*crefs).clone();
            (recRefs, b3) = compareCrefList(llrefs)?;
            i = ((recRefs).len() as i32);
            if intGt(i, 0) {
                b1 = 0 == intMod(((crefs).len() as i32), i);
                crefs = List::unionOnTrueList(&(list![recRefs, crefs.clone()]), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                b2 = intEq(((crefs).len() as i32), i);
                b1 = boolAnd(b1, boolAnd(b2, b3));
            } else {
                let true = (intEq(i, 0)) else { return Err("pattern mismatch") };
                let true = ((crefs).is_empty()) else { return Err("pattern mismatch") };
                b1 = true;
            }
            (crefs.clone(), b1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outrefs, matching))
}

pub(crate) fn renameUniqueOuterVars(mut dae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut odae: DAE::DAElist;
    (odae, _, _) = traverseDAE(
        dae,
        openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(removeUniqieIdentifierFromCref, metamodelica::Ref<DAE::Exp>, _))
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
            0,
        ),
    )?;
    Ok(odae)
}

fn removeUniqieIdentifierFromCref<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut oarg: Type_a,
) -> (metamodelica::Ref<DAE::Exp>, Type_a) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outDummy: Type_a;
    (outExp, outDummy) = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty } => {
                    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    cr2 = unNameInnerouterUniqueCref(cr.clone(), &(arcstr::literal!(DAE::UNIQUEIO)))?;
                    exp = Expression::makeCrefExp(cr2.clone(), ty.clone())?;
                    Ok((exp.clone(), oarg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), oarg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outDummy)
}

pub fn nameUniqueOuterVars(mut dae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut odae: DAE::DAElist;
    (odae, _, _) = traverseDAE(
        dae,
        openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(addUniqueIdentifierToCref, metamodelica::Ref<DAE::Exp>, _))
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
            0,
        ),
    )?;
    Ok(odae)
}

fn addUniqueIdentifierToCref<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut oarg: Type_a,
) -> (metamodelica::Ref<DAE::Exp>, Type_a) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outDummy: Type_a;
    (outExp, outDummy) = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty } => {
                    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    cr2 = nameInnerouterUniqueCref(metamodelica::AsArg::as_arg(&cr))?;
                    exp = Expression::makeCrefExp(cr2.clone(), ty.clone())?;
                    Ok((exp.clone(), oarg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), oarg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outDummy)
}

// helper functions for traverseDAE
fn traverseDAEOptExp<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut oexp: Option<metamodelica::Ref<DAE::Exp>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut iextraArg: Type_a,
) -> Result<(Option<metamodelica::Ref<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut ooexp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut oextraArg: Type_a;
    (ooexp, oextraArg) = (::match_deref::match_deref! { match &(oexp) {
        None => {
            let mut extraArg = iextraArg;
            (None, extraArg)
        },
        Some(e) => {
            let mut extraArg = iextraArg;
            let mut e = (*e).clone();
            (e, extraArg) = func(e.clone(), extraArg)?;
            (Some(e.clone()), extraArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ooexp, oextraArg))
}

fn traverseDAEExpList<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iexps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut oexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut oextraArg: Type_a;
    (oexps, oextraArg) = (::match_deref::match_deref! { match iexps {
        Deref @ metamodelica::ListNode::Nil => {
            let mut extraArg = iextraArg;
            (metamodelica::nil(), extraArg)
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: exps } => {
            let mut extraArg = iextraArg;
            let mut e = (*e).clone();
            (e, extraArg) = func(e.clone(), extraArg)?;
            (oexps, extraArg) = traverseDAEExpList(exps, func, extraArg)?;
            (metamodelica::cons(e.clone(), oexps), extraArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oexps, oextraArg))
}

fn traverseDAEList<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut idaeList: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >,
    mut iextraArg: Type_a,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    Type_a,
)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut traversedDaeList: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
    let mut oextraArg: Type_a;
    (traversedDaeList, oextraArg) = (::match_deref::match_deref! { match idaeList {
        Deref @ metamodelica::ListNode::Nil => {
            let mut extraArg = iextraArg;
            (metamodelica::nil(), extraArg)
        },
        Deref @ metamodelica::ListNode::Cons { head: branch, tail: daeList } => {
            let mut extraArg = iextraArg;
            let mut branch2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut recRes: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
            (branch2, extraArg) = traverseDAEElementList(branch.clone(), func.clone(), extraArg)?;
            (recRes, extraArg) = traverseDAEList(daeList, func.clone(), extraArg)?;
            (metamodelica::cons(branch2, recRes), extraArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((traversedDaeList, oextraArg))
}

pub fn getFunctionList(
    mut ft: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut failOnError: bool,
) -> Result<metamodelica::List<DAE::Function>> {
    let mut fns: metamodelica::List<DAE::Function>;
    let mut lst: metamodelica::List<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)>;
    let mut lstInvalid: metamodelica::List<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)>;
    let mut r#str: ArcStr;
    match '__try0: {
        fns = unwrap_break_err!(List::map(AvlTreePathFunction::listValues(ft, metamodelica::nil()), &Util::getOption), '__try0);
        Ok::<_, &'static str>((fns.clone(),))
    } {
        Ok((__try0_o0,)) => {
            fns = __try0_o0;
        }
        Err(_) => {
            lst = AvlTreePathFunction::toList(ft, metamodelica::nil());
            lstInvalid = List::select(
                lst.clone(),
                (std::sync::Arc::new(
                    move |__a0: (metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(isInvalidFunctionEntry(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn((metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)) -> Result<bool>
                            + 'static,
                    >),
            )?;
            r#str = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut p in (List::map(lstInvalid.clone(), &fnptr!(Util::tuple21, _))?)
                        .into_iter()
                        .cloned()
                    {
                        let __x = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!("\n "),
            );
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n "));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::NON_INSTANTIATED_FUNCTION.clone(), list![r#str.clone()])?;
            if failOnError {
                return Err("fail");
            }
            fns = List::mapMap(List::select(lst.clone(), (std::sync::Arc::new(move |__a0: (metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(isValidFunctionEntry(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)) -> Result<bool> + 'static>))?, &fnptr!(Util::tuple22, _), &Util::getOption)?;
        }
    }
    Ok(fns)
}

pub(crate) fn getFunctionNames(
    mut ft: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut strs: metamodelica::List<ArcStr>;
    strs = List::mapMap(
        getFunctionList(ft, false)?,
        &move |__a0: DAE::Function| -> metamodelica::Result<_> { ::std::result::Result::Ok(functionName(&__a0)) },
        &AbsynUtil::pathStringDefault,
    )?;
    Ok(strs)
}

fn isInvalidFunctionEntry(mut tpl: &(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(tpl) {
        (_, None) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn isValidFunctionEntry(mut tpl: &(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)) -> bool {
    let mut b: bool;
    b = !(isInvalidFunctionEntry(tpl));
    b
}

pub fn traverseDAE<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut dae: DAE::DAElist,
    mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut arg: ArgT,
) -> Result<(DAE::DAElist, metamodelica::Ref<AvlTreePathFunction::Tree>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut dae: DAE::DAElist = dae;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree> = functionTree;
    let mut arg: ArgT = arg;
    let mut el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    (el, arg) = traverseDAEElementList(dae.elementLst.clone(), func.clone(), arg)?;
    dae.elementLst = el;
    (functionTree, arg) = AvlTreePathFunction::mapFold(
        functionTree,
        &({
            let __pe_b2: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> = func.clone();
            move |__pe_a0, __pe_a1, __pe_a3| traverseDAEFuncHelper(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_a3)
        }),
        arg,
    )?;
    Ok((dae, functionTree, arg))
}

fn traverseDAEFuncHelper<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut key: metamodelica::Ref<Absyn::Path>,
    mut value: Option<DAE::Function>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut arg: ArgT,
) -> Result<(Option<DAE::Function>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut value: Option<DAE::Function> = value;
    let mut arg: ArgT = arg;
    (value, arg) = (match value.clone() {
        Some(mut daeFunc1) => {
            let mut daeFunc2: DAE::Function;
            (daeFunc2, arg) = traverseDAEFunc(daeFunc1.clone(), func.clone(), arg)?;
            (
                if (match (&(daeFunc1), &(daeFunc2.clone())) {
                    (
                        DAE::Function::FUNCTION {
                            path: __refeq_v0l,
                            functions: __refeq_v1l,
                            type_: __refeq_v2l,
                            visibility: __refeq_v3l,
                            partialPrefix: __refeq_v4l,
                            isImpure: __refeq_v5l,
                            inlineType: __refeq_v6l,
                            unusedInputs: __refeq_v7l,
                            source: __refeq_v8l,
                            comment: __refeq_v9l,
                        },
                        DAE::Function::FUNCTION {
                            path: __refeq_v0r,
                            functions: __refeq_v1r,
                            type_: __refeq_v2r,
                            visibility: __refeq_v3r,
                            partialPrefix: __refeq_v4r,
                            isImpure: __refeq_v5r,
                            inlineType: __refeq_v6r,
                            unusedInputs: __refeq_v7r,
                            source: __refeq_v8r,
                            comment: __refeq_v9r,
                        },
                    ) => {
                        referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                            && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v1l), &(*__refeq_v1r))
                            && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                            && (match (&(*__refeq_v3l), &(*__refeq_v3r)) {
                                (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true,
                                (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true,
                                _ => false,
                            })
                            && ((*__refeq_v4l) == (*__refeq_v4r))
                            && ((*__refeq_v5l) == (*__refeq_v5r))
                            && (match (&(*__refeq_v6l), &(*__refeq_v6r)) {
                                (DAE::InlineType::AFTER_INDEX_RED_INLINE, DAE::InlineType::AFTER_INDEX_RED_INLINE) => {
                                    true
                                }
                                (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => true,
                                (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true,
                                (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true,
                                (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true,
                                (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true,
                                _ => false,
                            })
                            && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v7l), &(*__refeq_v7r))
                            && referenceEq(&*(*__refeq_v8l), &*(*__refeq_v8r))
                            && (match (&(*__refeq_v9l), &(*__refeq_v9r)) {
                                (None, None) => true,
                                (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
                                _ => false,
                            })
                    }
                    (
                        DAE::Function::RECORD_CONSTRUCTOR {
                            path: __refeq_v0l,
                            type_: __refeq_v1l,
                            source: __refeq_v2l,
                        },
                        DAE::Function::RECORD_CONSTRUCTOR {
                            path: __refeq_v0r,
                            type_: __refeq_v1r,
                            source: __refeq_v2r,
                        },
                    ) => {
                        referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                            && referenceEq(&*(*__refeq_v1l), &*(*__refeq_v1r))
                            && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                    }
                    _ => false,
                }) {
                    value
                } else {
                    Some(daeFunc2)
                },
                arg,
            )
        }
        None => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- DAEUtil.traverseDAEFuncLst failed: "));
                __mm_s.push_str(&*AbsynUtil::pathString(key, literal!("."), true, false)?);
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
    });
    Ok((value, arg))
}

pub fn traverseDAEFunctions<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut functions: metamodelica::List<DAE::Function>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::List<DAE::Function>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut functions: metamodelica::List<DAE::Function> = functions;
    let mut arg: ArgT = arg;
    (functions, arg) = List::mapFold(
        &functions,
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> = func.clone();
            move |__pe_a0, __pe_a2| traverseDAEFunc(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        arg,
    )?;
    Ok((functions, arg))
}

fn traverseDAEFunc<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut daeFunction: DAE::Function,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut arg: ArgT,
) -> Result<(DAE::Function, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut daeFunction: DAE::Function = daeFunction;
    let mut arg: ArgT = arg;
    let () = (::match_deref::match_deref! { match &(daeFunction.clone()) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: fdef @ DAE::FunctionDefinition::FUNCTION_DEF { .. }, tail: rest_defs }, .. } => {
            let mut el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut fdef = (*fdef).clone();
            (el, arg) = traverseDAEElementList(var_field!(fdef.body, DAE::FunctionDefinition::FUNCTION_DEF).clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(var_field!(fdef.body, DAE::FunctionDefinition::FUNCTION_DEF).clone()), &(el))) {
                let __owned_variant_body_0 = el;
                if let DAE::FunctionDefinition::FUNCTION_DEF { body, .. } = &mut fdef {
                    *body = __owned_variant_body_0;
                } else { panic!("owned-variant field-assign: value held a different variant than DAE::FunctionDefinition::FUNCTION_DEF"); }
                let __owned_variant_functions_0 = metamodelica::cons(fdef.clone(), rest_defs.clone());
                if let DAE::Function::FUNCTION { functions, .. } = &mut daeFunction {
                    *functions = __owned_variant_functions_0;
                } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            }
            ()
        },
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: fdef @ DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: rest_defs }, .. } => {
            let mut el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut fdef = (*fdef).clone();
            (el, arg) = traverseDAEElementList(var_field!(fdef.body, DAE::FunctionDefinition::FUNCTION_EXT).clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(var_field!(fdef.body, DAE::FunctionDefinition::FUNCTION_EXT).clone()), &(el))) {
                let __owned_variant_body_0 = el;
                if let DAE::FunctionDefinition::FUNCTION_EXT { body, .. } = &mut fdef {
                    *body = __owned_variant_body_0;
                } else { panic!("owned-variant field-assign: value held a different variant than DAE::FunctionDefinition::FUNCTION_EXT"); }
                let __owned_variant_functions_0 = metamodelica::cons(fdef.clone(), rest_defs.clone());
                if let DAE::Function::FUNCTION { functions, .. } = &mut daeFunction {
                    *functions = __owned_variant_functions_0;
                } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
            }
            ()
        },
        DAE::Function::RECORD_CONSTRUCTOR { .. } => {
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((daeFunction, arg))
}

pub fn traverseDAEElementList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Element>>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    let mut arg: ArgT = arg;
    (elements, arg) = List::mapFold(
        &elements,
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> = func.clone();
            move |__pe_a0, __pe_a2| traverseDAEElement(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        arg,
    )?;
    Ok((elements, arg))
}

fn traverseDAEElement<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut element: metamodelica::Ref<DAE::Element>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<DAE::Element>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut element: metamodelica::Ref<DAE::Element> = element;
    let mut arg: ArgT = arg;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::VAR { componentRef: cr1, binding, variableAttributesOption: attr, .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_cr1: metamodelica::Ref<DAE::ComponentRef>;
            let mut new_binding: Option<metamodelica::Ref<DAE::Exp>>;
            let mut new_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
            let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            let mut daebinding: metamodelica::Ref<DAE::Binding>;
            let mut new_daebinding: metamodelica::Ref<DAE::Binding>;
            let mut changed: bool;
            let mut new_ty: metamodelica::Ref<DAE::Type>;
            (e1, arg) = func(Expression::crefExp(cr1.clone())?, arg)?;
            if Expression::isCref(&e1) {
                new_cr1 = Expression::expCref(&e1)?;
                if !(referenceEq(&*(cr1.clone()),&*(&*new_cr1))) {
                    assign_variant_field!(element => DAE::Element::VAR; componentRef = new_cr1);
                }
            }
            assign_variant_field!(element => DAE::Element::VAR; dims = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
        for mut d in (var_field!((*element).dims, DAE::Element::VAR).clone()).into_iter().cloned() {
            let __x = (match &*d.clone() {
        DAE::Dimension::DIM_EXP { exp: __esc_e1 } => {
            e1 = (*__esc_e1).clone();
            (new_e1, arg) = func(e1.clone(), arg.clone())?;
            if (referenceEq(&*(e1.clone()),&*(&*new_e1))) {d.clone()} else {metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: new_e1.clone() })}
        },
        _ => d.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            new_ty = { let mut ty = var_field!((*element).ty, DAE::Element::VAR).clone(); (match &*ty {
        DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } => {
            changed = false;
            varLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
        for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ DAE::Var { binding: __esc_daebinding @ Deref @ DAE::Binding::EQBOUND { .. }, .. } => {
            daebinding = (*__esc_daebinding).clone();
            (e2, arg) = func(var_field!((*daebinding).exp, DAE::Binding::EQBOUND).clone(), arg.clone())?;
            if !(referenceEq(&*(var_field!((*daebinding).exp, DAE::Binding::EQBOUND).clone()),&*(&*e2))) {
                daebinding = metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: e2.clone(), evaluatedExp: None, constant_: var_field!((*daebinding).constant_, DAE::Binding::EQBOUND).clone(), source: var_field!((*daebinding).source, DAE::Binding::EQBOUND).clone() });
                assign_field!(v.binding = daebinding.clone());
                changed = true;
            }
            v.clone()
        },
        Deref @ DAE::Var { binding: __esc_daebinding @ Deref @ DAE::Binding::VALBOUND { .. }, .. } => {
            daebinding = (*__esc_daebinding).clone();
            e1 = ValuesUtil::valueExp(var_field!((*daebinding).valBound, DAE::Binding::VALBOUND).clone(), None)?;
            (e2, arg) = func(e1.clone(), arg.clone())?;
            if !(referenceEq(&*(&*e1),&*(&*e2))) {
                new_daebinding = metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: e2.clone(), evaluatedExp: None, constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: var_field!((*daebinding).source, DAE::Binding::VALBOUND).clone() });
                assign_field!(v.binding = new_daebinding.clone());
                changed = true;
            }
            v.clone()
        },
        _ => v.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if !(metamodelica::ReferenceEq::reference_eq(&(varLst), &(var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()))) {
                assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = varLst);
            }
            ty.clone()
        },
        _ => ty.clone(),
    }) };
            if !(referenceEq(&*(var_field!((*element).ty, DAE::Element::VAR).clone()),&*(&*new_ty))) {
                assign_variant_field!(element => DAE::Element::VAR; ty = new_ty);
            }
            (new_binding, arg) = traverseDAEOptExp(binding.clone(), &*func, arg)?;
            if !((match (&(binding), &(new_binding)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {
                assign_variant_field!(element => DAE::Element::VAR; binding = new_binding);
            }
            (new_attr, arg) = traverseDAEVarAttr(attr.clone(), &*func, arg)?;
            if !((match (&(attr), &(new_attr)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {
                assign_variant_field!(element => DAE::Element::VAR; variableAttributesOption = new_attr);
            }
            ()
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr1, exp: e1, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_cr1: metamodelica::Ref<DAE::ComponentRef>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::DEFINE; exp = new_e1);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr1.clone())?, arg)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            new_cr1 = metamodelica::Own::own(__pa0);
            arg = metamodelica::Own::own(__pa1);
            if !(referenceEq(&*(cr1.clone()),&*(&*new_cr1))) {
                assign_variant_field!(element => DAE::Element::DEFINE; componentRef = new_cr1);
            }
            ()
        },
        Deref @ DAE::Element::INITIALDEFINE { componentRef: cr1, exp: e1, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_cr1: metamodelica::Ref<DAE::ComponentRef>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::INITIALDEFINE; exp = new_e1);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr1.clone())?, arg)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            new_cr1 = metamodelica::Own::own(__pa0);
            arg = metamodelica::Own::own(__pa1);
            if !(referenceEq(&*(cr1.clone()),&*(&*new_cr1))) {
                assign_variant_field!(element => DAE::Element::INITIALDEFINE; componentRef = new_cr1);
            }
            ()
        },
        Deref @ DAE::Element::EQUEQUATION { cr1, cr2, .. } => {
            let mut new_cr1: metamodelica::Ref<DAE::ComponentRef>;
            let mut new_cr2: metamodelica::Ref<DAE::ComponentRef>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr1.clone())?, arg)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            new_cr1 = metamodelica::Own::own(__pa0);
            arg = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr2.clone())?, arg)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa3, .. }, __pa4) => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            new_cr2 = metamodelica::Own::own(__pa3);
            arg = metamodelica::Own::own(__pa4);
            if !(referenceEq(&*(cr1.clone()),&*(&*new_cr1))) || !(referenceEq(&*(cr2.clone()),&*(&*new_cr2))) {
                element = metamodelica::Ref::new(DAE::Element::EQUEQUATION { cr1: new_cr1, cr2: new_cr2, source: var_field!((*element).source, DAE::Element::EQUEQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_e2: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            (new_e2, arg) = func(e2.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) || !(referenceEq(&*(e2.clone()),&*(&*new_e2))) {
                element = metamodelica::Ref::new(DAE::Element::EQUATION { exp: new_e1, scalar: new_e2, source: var_field!((*element).source, DAE::Element::EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_e2: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            (new_e2, arg) = func(e2.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) || !(referenceEq(&*(e2.clone()),&*(&*new_e2))) {
                element = metamodelica::Ref::new(DAE::Element::INITIALEQUATION { exp1: new_e1, exp2: new_e2, source: var_field!((*element).source, DAE::Element::INITIALEQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_e2: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            (new_e2, arg) = func(e2.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) || !(referenceEq(&*(e2.clone()),&*(&*new_e2))) {
                element = metamodelica::Ref::new(DAE::Element::COMPLEX_EQUATION { lhs: new_e1, rhs: new_e2, source: var_field!((*element).source, DAE::Element::COMPLEX_EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_e2: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            (new_e2, arg) = func(e2.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) || !(referenceEq(&*(e2.clone()),&*(&*new_e2))) {
                element = metamodelica::Ref::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: new_e1, rhs: new_e2, source: var_field!((*element).source, DAE::Element::INITIAL_COMPLEX_EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::ARRAY_EQUATION { exp: e1, array: e2, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_e2: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            (new_e2, arg) = func(e2.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) || !(referenceEq(&*(e2.clone()),&*(&*new_e2))) {
                element = metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: var_field!((*element).dimension, DAE::Element::ARRAY_EQUATION).clone(), exp: new_e1, array: new_e2, source: var_field!((*element).source, DAE::Element::ARRAY_EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { exp: e1, array: e2, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_e2: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            (new_e2, arg) = func(e2.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) || !(referenceEq(&*(e2.clone()),&*(&*new_e2))) {
                element = metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: var_field!((*element).dimension, DAE::Element::INITIAL_ARRAY_EQUATION).clone(), exp: new_e1, array: new_e2, source: var_field!((*element).source, DAE::Element::INITIAL_ARRAY_EQUATION).clone() });
            }
            ()
        },
        Deref @ DAE::Element::WHEN_EQUATION { condition: e1, equations: el, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e: metamodelica::Ref<DAE::Element>;
            let mut new_e: metamodelica::Ref<DAE::Element>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::WHEN_EQUATION; condition = new_e1);
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(el.clone()), &(new_el))) {
                assign_variant_field!(element => DAE::Element::WHEN_EQUATION; equations = new_el);
            }
            if (var_field!((*element).elsewhen_, DAE::Element::WHEN_EQUATION)).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*element).elsewhen_, DAE::Element::WHEN_EQUATION).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa0);
                (new_e, arg) = traverseDAEElement(e.clone(), func.clone(), arg)?;
                if !(referenceEq(&*(e),&*(&*new_e))) {
                    assign_variant_field!(element => DAE::Element::WHEN_EQUATION; elsewhen_ = Some(new_e));
                }
            }
            ()
        },
        Deref @ DAE::Element::FOR_EQUATION { range: e1, equations: el, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::FOR_EQUATION; range = new_e1);
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(el.clone()), &(new_el))) {
                assign_variant_field!(element => DAE::Element::FOR_EQUATION; equations = new_el);
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { range: e1, equations: el, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::INITIAL_FOR_EQUATION; range = new_e1);
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(el.clone()), &(new_el))) {
                assign_variant_field!(element => DAE::Element::INITIAL_FOR_EQUATION; equations = new_el);
            }
            ()
        },
        Deref @ DAE::Element::COMP { dAElist: el, .. } => {
            let mut new_el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(el.clone()), &(new_el))) {
                assign_variant_field!(element => DAE::Element::COMP; dAElist = new_el);
            }
            ()
        },
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
            ()
        },
        Deref @ DAE::Element::ASSERT { condition: e1, message: e2, level: e3, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_e2: metamodelica::Ref<DAE::Exp>;
            let mut new_e3: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::ASSERT; condition = new_e1);
            }
            (new_e2, arg) = func(e2.clone(), arg)?;
            if !(referenceEq(&*(e2.clone()),&*(&*new_e2))) {
                assign_variant_field!(element => DAE::Element::ASSERT; message = new_e2);
            }
            (new_e3, arg) = func(e3.clone(), arg)?;
            if !(referenceEq(&*(e3.clone()),&*(&*new_e3))) {
                assign_variant_field!(element => DAE::Element::ASSERT; level = new_e3);
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_ASSERT { condition: e1, message: e2, level: e3, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_e2: metamodelica::Ref<DAE::Exp>;
            let mut new_e3: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::INITIAL_ASSERT; condition = new_e1);
            }
            (new_e2, arg) = func(e2.clone(), arg)?;
            if !(referenceEq(&*(e2.clone()),&*(&*new_e2))) {
                assign_variant_field!(element => DAE::Element::INITIAL_ASSERT; message = new_e2);
            }
            (new_e3, arg) = func(e3.clone(), arg)?;
            if !(referenceEq(&*(e3.clone()),&*(&*new_e3))) {
                assign_variant_field!(element => DAE::Element::INITIAL_ASSERT; level = new_e3);
            }
            ()
        },
        Deref @ DAE::Element::TERMINATE { message: e1, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::TERMINATE; message = new_e1);
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::INITIAL_TERMINATE; message = new_e1);
            }
            ()
        },
        Deref @ DAE::Element::NORETCALL { exp: e1, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::NORETCALL; exp = new_e1);
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { exp: e1, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::INITIAL_NORETCALL; exp = new_e1);
            }
            ()
        },
        Deref @ DAE::Element::REINIT { componentRef: cr1, exp: e1, .. } => {
            let mut new_e1: metamodelica::Ref<DAE::Exp>;
            let mut new_cr1: metamodelica::Ref<DAE::ComponentRef>;
            (new_e1, arg) = func(e1.clone(), arg)?;
            if !(referenceEq(&*(e1.clone()),&*(&*new_e1))) {
                assign_variant_field!(element => DAE::Element::REINIT; exp = new_e1);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(func(Expression::crefExp(cr1.clone())?, arg)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            new_cr1 = metamodelica::Own::own(__pa0);
            arg = metamodelica::Own::own(__pa1);
            if !(referenceEq(&*(cr1.clone()),&*(&*new_cr1))) {
                assign_variant_field!(element => DAE::Element::REINIT; componentRef = new_cr1);
            }
            ()
        },
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            let mut new_stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (new_stmts, arg) = traverseDAEEquationsStmts(stmts.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(new_stmts))) {
                assign_variant_field!(element => DAE::Element::ALGORITHM; algorithm_ = metamodelica::Ref::new(DAE::Algorithm { statementLst: new_stmts }));
            }
            ()
        },
        Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            let mut new_stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (new_stmts, arg) = traverseDAEEquationsStmts(stmts.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(new_stmts))) {
                assign_variant_field!(element => DAE::Element::INITIALALGORITHM; algorithm_ = metamodelica::Ref::new(DAE::Algorithm { statementLst: new_stmts }));
            }
            ()
        },
        Deref @ DAE::Element::CONSTRAINT { constraints: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: expl }, .. } => {
            let mut new_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (new_expl, arg) = traverseDAEExpList(metamodelica::AsArg::as_arg(&expl), &*func, arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(new_expl))) {
                assign_variant_field!(element => DAE::Element::CONSTRAINT; constraints = metamodelica::Ref::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: new_expl }));
            }
            ()
        },
        Deref @ DAE::Element::CLASS_ATTRIBUTES { .. } => {
            ()
        },
        Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: eqll, equations3: el, .. } => {
            let mut new_el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut new_eqll: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
            let mut new_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (new_expl, arg) = traverseDAEExpList(metamodelica::AsArg::as_arg(&expl), &*func, arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(new_expl))) {
                assign_variant_field!(element => DAE::Element::IF_EQUATION; condition1 = new_expl);
            }
            (new_eqll, arg) = traverseDAEList(metamodelica::AsArg::as_arg(&eqll), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(eqll.clone()), &(new_eqll))) {
                assign_variant_field!(element => DAE::Element::IF_EQUATION; equations2 = new_eqll);
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(el.clone()), &(new_el))) {
                assign_variant_field!(element => DAE::Element::IF_EQUATION; equations3 = new_el);
            }
            ()
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: expl, equations2: eqll, equations3: el, .. } => {
            let mut new_el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut new_eqll: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
            let mut new_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (new_expl, arg) = traverseDAEExpList(metamodelica::AsArg::as_arg(&expl), &*func, arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(new_expl))) {
                assign_variant_field!(element => DAE::Element::INITIAL_IF_EQUATION; condition1 = new_expl);
            }
            (new_eqll, arg) = traverseDAEList(metamodelica::AsArg::as_arg(&eqll), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(eqll.clone()), &(new_eqll))) {
                assign_variant_field!(element => DAE::Element::INITIAL_IF_EQUATION; equations2 = new_eqll);
            }
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(el.clone()), &(new_el))) {
                assign_variant_field!(element => DAE::Element::INITIAL_IF_EQUATION; equations3 = new_el);
            }
            ()
        },
        Deref @ DAE::Element::FLAT_SM { dAElist: el, .. } => {
            let mut new_el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(el.clone()), &(new_el))) {
                assign_variant_field!(element => DAE::Element::FLAT_SM; dAElist = new_el);
            }
            ()
        },
        Deref @ DAE::Element::SM_COMP { dAElist: el, .. } => {
            let mut new_el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            (new_el, arg) = traverseDAEElementList(el.clone(), func.clone(), arg)?;
            if !(metamodelica::ReferenceEq::reference_eq(&(el.clone()), &(new_el))) {
                assign_variant_field!(element => DAE::Element::SM_COMP; dAElist = new_el);
            }
            ()
        },
        Deref @ DAE::Element::COMMENT { .. } => {
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.traverseDAEElement not implemented correctly for element: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![element.clone()]))?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((element, arg))
}

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum TraverseStatementsOptions {
    TRAVERSE_ALL,
    TRAVERSE_RHS_ONLY,
}
impl metamodelica::gc::MMTrace for TraverseStatementsOptions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TraverseStatementsOptions::TRAVERSE_ALL => Ok(()),
            TraverseStatementsOptions::TRAVERSE_RHS_ONLY => Ok(()),
        }
    }
}
pub(crate) use self::TraverseStatementsOptions::{TRAVERSE_ALL, TRAVERSE_RHS_ONLY};

pub fn traverseAlgorithmExps<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inAlgorithm: &metamodelica::Ref<DAE::Algorithm>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >,
    mut inTypeA: Type_a,
) -> Result<Type_a> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outTypeA: Type_a;
    outTypeA = (match &**inAlgorithm {
        DAE::Algorithm { statementLst: stmts } => {
            let mut ext_arg_1: Type_a;
            (_, ext_arg_1) = traverseDAEEquationsStmts(stmts.clone(), func.clone(), inTypeA)?;
            ext_arg_1
        }
    });
    Ok(outTypeA)
}

pub fn traverseDAEEquationsStmts<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut oextraArg: Type_a;
    (outStmts, oextraArg) = traverseDAEEquationsStmtsList(
        inStmts,
        func.clone(),
        crate::DAEUtil::TraverseStatementsOptions::TRAVERSE_ALL,
        iextraArg,
    )?;
    Ok((outStmts, oextraArg))
}

pub fn traverseDAEEquationsStmtsRhsOnly<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut oextraArg: Type_a;
    (outStmts, oextraArg) = traverseDAEEquationsStmtsList(
        inStmts,
        func.clone(),
        crate::DAEUtil::TraverseStatementsOptions::TRAVERSE_RHS_ONLY,
        iextraArg,
    )?;
    Ok((outStmts, oextraArg))
}

fn traverseDAEEquationsStmtsList<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >,
    mut opt: TraverseStatementsOptions,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut oextraArg: Type_a;
    let mut outStmtsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>>;
    let mut b: bool;
    (outStmtsLst, oextraArg) = List::map2Fold(
        &inStmts,
        &traverseDAEEquationsStmtsWork,
        func.clone(),
        opt,
        iextraArg,
        metamodelica::nil(),
    )?;
    outStmts = List::flatten(outStmtsLst)?;
    b = List::allReferenceEq(inStmts.clone(), outStmts.clone())?;
    outStmts = if (b) { inStmts } else { outStmts };
    Ok((outStmts, oextraArg))
}

fn traverseStatementsOptionsEvalLhs<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inA: Type_a,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut opt: TraverseStatementsOptions,
) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outA: Type_a;
    (outExp, outA) = (match opt {
        TraverseStatementsOptions::TRAVERSE_ALL { .. } => {
            (outExp, outA) = func(inExp, inA)?;
            (outExp, outA)
        }
        _ => (inExp, inA),
    });
    Ok((outExp, outA))
}

fn traverseDAEEquationsStmtsWork<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStmt: metamodelica::Ref<DAE::Statement>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >,
    mut opt: TraverseStatementsOptions,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut oextraArg: Type_a;
    (outStmts, oextraArg) = 'mc: {
        let __mc_input = (inStmt.clone(), iextraArg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, exp1: e, exp: e2, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = traverseStatementsOptionsEvalLhs(e.clone(), extraArg.clone(), &*func, opt)?;
                    (e_2, extraArg) = func(e2.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1)) && referenceEq(&*(e2.clone()),&*(&*e_2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e_1.clone(), exp: e_2.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, expExpLst: expl1, exp: e, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseStatementsOptionsEvalLhs(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expl1.clone() }), extraArg.clone(), &*func, opt)?) {
                        (Deref @ DAE::Exp::TUPLE { PR: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl2 = metamodelica::Own::own(__pa0);
                    extraArg = metamodelica::Own::own(__pa1);
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(expl1.clone()), &(expl2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: expl2.clone(), exp: e_1.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: tp, lhs: e, exp: e2, source }, extraArg) => {
                            let mut e_1: metamodelica::Ref<DAE::Exp>;
                            let mut e_2: metamodelica::Ref<DAE::Exp>;
                            let mut x: metamodelica::Ref<DAE::Statement>;
                            let mut extraArg = (*extraArg).clone();
                            (e_2, extraArg) = func(e2.clone(), extraArg.clone())?;
                            (e_1, extraArg) = traverseStatementsOptionsEvalLhs(e.clone(), extraArg.clone(), &*func, opt)?;
                            x = (match &*e_1 {
                DAE::Exp::CREF { .. } => if (referenceEq(&*(e2.clone()),&*(&*e_2)) && referenceEq(&*(e.clone()),&*(&*e_1))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_1.clone(), exp: e_2.clone(), source: source.clone() })},
                _ => if (referenceEq(&*(e2.clone()),&*(&*e_2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e.clone(), exp: e_2.clone(), source: source.clone() })},
            });
                            Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { exp: e, statementLst: stmts, else_: algElse, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut algElse1: metamodelica::Ref<DAE::Else>;
                    let mut b: bool;
                    let mut extraArg = (*extraArg).clone();
                    (algElse1, extraArg) = traverseDAEEquationsStmtsElse(metamodelica::AsArg::as_arg(&algElse), func.clone(), opt, extraArg.clone())?;
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt, extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    (stmts1, b) = Algorithm::optimizeIf(&e_1, stmts2.clone(), &algElse1, source.clone());
                    stmts1 = if (!(b) && referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2)) && referenceEq(&*(algElse.clone()),&*(&*algElse1))) {metamodelica::cons(inStmt.clone(), metamodelica::nil())} else {stmts1.clone()};
                    Ok((stmts1.clone(), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { type_: tp, iterIsArray: b1, iter: id1, range: e, statementLst: stmts, source, sub_iters }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt, extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: id1.clone(), range: e_1.clone(), statementLst: stmts2.clone(), source: source.clone(), sub_iters: sub_iters.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_PARFOR { type_: tp, iterIsArray: b1, iter: id1, range: e, statementLst: stmts, loopPrlVars, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt, extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_PARFOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: id1.clone(), range: e_1.clone(), statementLst: stmts2.clone(), loopPrlVars: loopPrlVars.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHILE { exp: e, statementLst: stmts, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt, extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmts, elseWhen: None, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt, extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: None, source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmts, elseWhen: Some(ew), source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut ew_1: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseDAEEquationsStmtsList(list![ew.clone()], func.clone(), opt, extraArg.clone())?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ew_1 = metamodelica::Own::own(__pa0);
                    extraArg = metamodelica::Own::own(__pa1);
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt, extraArg.clone())?;
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(ew.clone()),&*(&*ew_1)) && referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: Some(ew_1.clone()), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSERT { cond: e, msg: e2, level: e3, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut e_3: metamodelica::Ref<DAE::Exp>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    (e_2, extraArg) = func(e2.clone(), extraArg.clone())?;
                    (e_3, extraArg) = func(e3.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1)) && referenceEq(&*(e2.clone()),&*(&*e_2)) && referenceEq(&*(e3.clone()),&*(&*e_3))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: e_1.clone(), msg: e_2.clone(), level: e_3.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TERMINATE { msg: e, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: e_1.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_REINIT { var: e, value: e2, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    (e_2, extraArg) = func(e2.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1)) && referenceEq(&*(e2.clone()),&*(&*e_2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_REINIT { var: e_1.clone(), value: e_2.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_NORETCALL { exp: e, source }, extraArg) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (e_1, extraArg) = func(e.clone(), extraArg.clone())?;
                    x = if (referenceEq(&*(e.clone()),&*(&*e_1))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (x @ Deref @ DAE::Statement::STMT_RETURN { .. }, extraArg) => {
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (x @ Deref @ DAE::Statement::STMT_BREAK { .. }, extraArg) => {
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (x @ Deref @ DAE::Statement::STMT_CONTINUE { .. }, extraArg) => {
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, source }, extraArg) => {
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut x: metamodelica::Ref<DAE::Statement>;
                    let mut extraArg = (*extraArg).clone();
                    (stmts2, extraArg) = traverseDAEEquationsStmtsList(stmts.clone(), func.clone(), opt, extraArg.clone())?;
                    x = if (metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {inStmt.clone()} else {metamodelica::Ref::new(DAE::Statement::STMT_FAILURE { body: stmts2.clone(), source: source.clone() })};
                    Ok((metamodelica::cons(x.clone(), metamodelica::nil()), extraArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (x, _) => {
                    let mut r#str: ArcStr;
                    r#str = DAEDump::ppStatementStr(x.clone());
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.traverseDAEEquationsStmts not implemented correctly: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
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
    Ok((outStmts, oextraArg))
}

fn traverseDAEEquationsStmtsElse<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElse: &metamodelica::Ref<DAE::Else>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >,
    mut opt: TraverseStatementsOptions,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::Ref<DAE::Else>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outElse: metamodelica::Ref<DAE::Else>;
    let mut oextraArg: Type_a;
    (outElse, oextraArg) = (match &**inElse {
        DAE::Else::NOELSE { .. } => {
            let mut extraArg = iextraArg;
            (openmodelica_frontend_types::DAE::Else::interned_NOELSE(), extraArg)
        }
        DAE::Else::ELSEIF {
            exp: e,
            statementLst: st,
            else_: el,
        } => {
            let mut extraArg = iextraArg;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut st_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut el_1: metamodelica::Ref<DAE::Else>;
            let mut b: bool;
            (el_1, extraArg) = traverseDAEEquationsStmtsElse(el, func.clone(), opt, extraArg)?;
            (st_1, extraArg) = traverseDAEEquationsStmtsList(st.clone(), func.clone(), opt, extraArg)?;
            (e_1, extraArg) = func(e.clone(), extraArg)?;
            outElse = Algorithm::optimizeElseIf(e_1.clone(), st_1.clone(), el_1.clone());
            b = referenceEq(&*(el.clone()), &*(el_1))
                && metamodelica::ReferenceEq::reference_eq(&(st.clone()), &(st_1))
                && referenceEq(&*(e.clone()), &*(e_1));
            outElse = if (b) { inElse.clone() } else { outElse };
            (outElse, extraArg)
        }
        DAE::Else::ELSE { statementLst: st } => {
            let mut extraArg = iextraArg;
            let mut st_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (st_1, extraArg) = traverseDAEEquationsStmtsList(st.clone(), func.clone(), opt, extraArg)?;
            outElse = if (metamodelica::ReferenceEq::reference_eq(&(st.clone()), &(st_1))) {
                inElse.clone()
            } else {
                metamodelica::Ref::new(DAE::Else::ELSE { statementLst: st_1 })
            };
            (outElse, extraArg)
        }
    });
    Ok((outElse, oextraArg))
}

pub fn traverseDAEStmts<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<DAE::Statement>,
        Type_a,
    ) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<DAE::Statement>,
                Type_a,
            ) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut extraArg: Type_a = iextraArg;
    let mut e_1: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e_2: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut e3: metamodelica::Ref<DAE::Exp>;
    let mut e_3: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut ew: metamodelica::Ref<DAE::Statement>;
    let mut b1: bool;
    let mut id1: ArcStr;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut algElse: metamodelica::Ref<DAE::Else>;
    let mut loopPrlVars: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>;
    let mut conditions: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut initialCall: bool;
    let mut sub_iters: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    )>;
    for mut stmt in &**inStmts {
        outStmts = 'mc: {
            let __mc_input = stmt.clone();
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, exp1: e2, exp: e, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut e_2: metamodelica::Ref<DAE::Exp> = e_2.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        (e_2, extraArg) = func(e2.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1)) && referenceEq(&*(e2.clone()),&*(&*e_2))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e_2.clone(), exp: e_1.clone(), source: source.clone() }), outStmts.clone())}, e_1.clone(), e_2.clone(), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                e_2 = __wb1;
                extraArg = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, expExpLst: expl1, exp: e, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expl2.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        (expl2, extraArg) = traverseDAEExpListStmt(metamodelica::AsArg::as_arg(&expl1), func, metamodelica::AsArg::as_arg(&stmt), extraArg.clone())?;
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(expl2), &(expl1.clone()))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: expl2.clone(), exp: e_1.clone(), source: source.clone() }), outStmts.clone())}, e_1.clone(), expl2.clone(), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                expl2 = __wb1;
                extraArg = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: tp, lhs: e, exp: e2, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut e_2: metamodelica::Ref<DAE::Exp> = e_2.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        (e_2, extraArg) = func(e2.clone(), stmt.clone(), extraArg.clone())?;
                        match '__try0: {
                            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(func(e.clone(), stmt.clone(), extraArg.clone()), '__try0)) {
                                (__pa1 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, __pa2) => (__pa1.clone(), __pa2.clone()),
                                _ => break '__try0 Err::<_, _>("pattern mismatch"),
                            } };
                            e_1 = metamodelica::Own::own(__pa1);
                            extraArg = metamodelica::Own::own(__pa2);
                            Ok::<_, &'static str>((e_1.clone(),))
                        } {
                            Ok((__try0_o0,)) => {
                                e_1 = __try0_o0;
                            }
                            Err(_) => {
                                e_1 = e.clone();
                            }
                        }
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1)) && referenceEq(&*(e2.clone()),&*(&*e_2))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_1.clone(), exp: e_2.clone(), source: source.clone() }), outStmts.clone())}, e_1.clone(), e_2.clone(), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                e_2 = __wb1;
                extraArg = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_IF { exp: e, statementLst: stmts, else_: algElse, source } => {
                        let mut algElse = (*algElse).clone();
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts1.clone();
                        let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts2.clone();
                        (algElse, extraArg) = traverseDAEStmtsElse(metamodelica::AsArg::as_arg(&algElse), func, metamodelica::AsArg::as_arg(&stmt), extraArg.clone())?;
                        (stmts2, extraArg) = traverseDAEStmts(metamodelica::AsArg::as_arg(&stmts), func, extraArg.clone())?;
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        (stmts1, _) = Algorithm::optimizeIf(&e_1, stmts2.clone(), metamodelica::AsArg::as_arg(&algElse), source.clone());
                        Ok((List::append_reverse(&stmts1, outStmts.clone()), e_1.clone(), extraArg.clone(), stmts1.clone(), stmts2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                extraArg = __wb1;
                stmts1 = __wb2;
                stmts2 = __wb3;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_FOR { type_: tp, iterIsArray: b1, iter: id1, range: e, statementLst: stmts, source, sub_iters } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts2.clone();
                        (stmts2, extraArg) = traverseDAEStmts(metamodelica::AsArg::as_arg(&stmts), func, extraArg.clone())?;
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: id1.clone(), range: e_1.clone(), statementLst: stmts2.clone(), source: source.clone(), sub_iters: sub_iters.clone() }), outStmts.clone())}, e_1.clone(), extraArg.clone(), stmts2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                extraArg = __wb1;
                stmts2 = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_PARFOR { type_: tp, iterIsArray: b1, iter: id1, range: e, statementLst: stmts, loopPrlVars, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts2.clone();
                        (stmts2, extraArg) = traverseDAEStmts(metamodelica::AsArg::as_arg(&stmts), func, extraArg.clone())?;
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_PARFOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: id1.clone(), range: e_1.clone(), statementLst: stmts2.clone(), loopPrlVars: loopPrlVars.clone(), source: source.clone() }), outStmts.clone()), e_1.clone(), extraArg.clone(), stmts2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                extraArg = __wb1;
                stmts2 = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_WHILE { exp: e, statementLst: stmts, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts2.clone();
                        (stmts2, extraArg) = traverseDAEStmts(metamodelica::AsArg::as_arg(&stmts), func, extraArg.clone())?;
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1)) && metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() }), outStmts.clone())}, e_1.clone(), extraArg.clone(), stmts2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                extraArg = __wb1;
                stmts2 = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmts, elseWhen: None, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts2.clone();
                        (stmts2, extraArg) = traverseDAEStmts(metamodelica::AsArg::as_arg(&stmts), func, extraArg.clone())?;
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: None, source: source.clone() }), outStmts.clone()), e_1.clone(), extraArg.clone(), stmts2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                extraArg = __wb1;
                stmts2 = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmts, elseWhen: Some(ew), source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts2.clone();
                        let __pa0 = ::match_deref::match_deref! { match &(traverseDAEStmts(&(list![ew.clone()]), func, extraArg.clone())?) {
                            (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, __pa0) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        extraArg = metamodelica::Own::own(__pa0);
                        (stmts2, extraArg) = traverseDAEStmts(metamodelica::AsArg::as_arg(&stmts), func, extraArg.clone())?;
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: Some(ew.clone()), source: source.clone() }), outStmts.clone()), e_1.clone(), extraArg.clone(), stmts2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                extraArg = __wb1;
                stmts2 = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_ASSERT { cond: e, msg: e2, level: e3, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut e_2: metamodelica::Ref<DAE::Exp> = e_2.clone();
                        let mut e_3: metamodelica::Ref<DAE::Exp> = e_3.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        (e_2, extraArg) = func(e2.clone(), stmt.clone(), extraArg.clone())?;
                        (e_3, extraArg) = func(e3.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1)) && referenceEq(&*(e2.clone()),&*(&*e_2)) && referenceEq(&*(e3.clone()),&*(&*e_3))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: e_1.clone(), msg: e_2.clone(), level: e_3.clone(), source: source.clone() }), outStmts.clone())}, e_1.clone(), e_2.clone(), e_3.clone(), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                e_2 = __wb1;
                e_3 = __wb2;
                extraArg = __wb3;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_TERMINATE { msg: e, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: e_1.clone(), source: source.clone() }), outStmts.clone())}, e_1.clone(), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                extraArg = __wb1;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_REINIT { var: e, value: e2, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut e_2: metamodelica::Ref<DAE::Exp> = e_2.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        (e_2, extraArg) = func(e2.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1)) && referenceEq(&*(e2.clone()),&*(&*e_2))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_REINIT { var: e_1.clone(), value: e_2.clone(), source: source.clone() }), outStmts.clone())}, e_1.clone(), e_2.clone(), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                e_2 = __wb1;
                extraArg = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_NORETCALL { exp: e, source } => {
                        let mut e_1: metamodelica::Ref<DAE::Exp> = e_1.clone();
                        let mut extraArg: Type_a = extraArg.clone();
                        (e_1, extraArg) = func(e.clone(), stmt.clone(), extraArg.clone())?;
                        Ok((if (referenceEq(&*(e.clone()),&*(&*e_1))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() }), outStmts.clone())}, e_1.clone(), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e_1 = __wb0;
                extraArg = __wb1;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_RETURN { .. } => {
                        let mut extraArg: Type_a = extraArg.clone();
                        (_, extraArg) = func(metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 }), stmt.clone(), extraArg.clone())?;
                        Ok((metamodelica::cons(stmt.clone(), outStmts.clone()), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                extraArg = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_BREAK { .. } => {
                        let mut extraArg: Type_a = extraArg.clone();
                        (_, extraArg) = func(metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 }), stmt.clone(), extraArg.clone())?;
                        Ok((metamodelica::cons(stmt.clone(), outStmts.clone()), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                extraArg = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_CONTINUE { .. } => {
                        let mut extraArg: Type_a = extraArg.clone();
                        (_, extraArg) = func(metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 }), stmt.clone(), extraArg.clone())?;
                        Ok((metamodelica::cons(stmt.clone(), outStmts.clone()), extraArg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                extraArg = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_FAILURE { body: stmts, source } => {
                        let mut extraArg: Type_a = extraArg.clone();
                        let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts2.clone();
                        (stmts2, extraArg) = traverseDAEStmts(metamodelica::AsArg::as_arg(&stmts), func, extraArg.clone())?;
                        Ok((if (metamodelica::ReferenceEq::reference_eq(&(stmts.clone()), &(stmts2))) {metamodelica::cons(stmt.clone(), outStmts.clone())} else {metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_FAILURE { body: stmts2.clone(), source: source.clone() }), outStmts.clone())}, extraArg.clone(), stmts2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                extraArg = __wb0;
                stmts2 = __wb1;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let mut r#str: ArcStr = r#str.clone();
                        r#str = DAEDump::ppStatementStr(stmt.clone());
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEUtil.traverseDAEStmts not implemented correctly: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
                        Ok((return Err("fail"), r#str.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                r#str = __wb0;
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
    }
    outStmts = metamodelica::Dangerous::listReverseInPlace(outStmts);
    Ok((outStmts, extraArg))
}

fn traverseDAEStmtsElse<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElse: &metamodelica::Ref<DAE::Else>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<DAE::Statement>,
        Type_a,
    ) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut istmt: &metamodelica::Ref<DAE::Statement>,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::Ref<DAE::Else>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<DAE::Statement>,
                Type_a,
            ) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outElse: metamodelica::Ref<DAE::Else>;
    let mut oextraArg: Type_a;
    (outElse, oextraArg) = (match &**inElse {
        DAE::Else::NOELSE { .. } => {
            let mut extraArg = iextraArg;
            (openmodelica_frontend_types::DAE::Else::interned_NOELSE(), extraArg)
        }
        DAE::Else::ELSEIF {
            exp: e,
            statementLst: st,
            else_: el,
        } => {
            let mut extraArg = iextraArg;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut st_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut el_1: metamodelica::Ref<DAE::Else>;
            (el_1, extraArg) = traverseDAEStmtsElse(el, func, istmt, extraArg)?;
            (st_1, extraArg) = traverseDAEStmts(st, func, extraArg)?;
            (e_1, extraArg) = func(e.clone(), istmt.clone(), extraArg)?;
            (Algorithm::optimizeElseIf(e_1, st_1, el_1), extraArg)
        }
        DAE::Else::ELSE { statementLst: st } => {
            let mut extraArg = iextraArg;
            let mut st_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (st_1, extraArg) = traverseDAEStmts(st, func, extraArg)?;
            (metamodelica::Ref::new(DAE::Else::ELSE { statementLst: st_1 }), extraArg)
        }
    });
    Ok((outElse, oextraArg))
}

fn traverseDAEExpListStmt<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iexps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<DAE::Statement>,
        Type_a,
    ) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut istmt: &metamodelica::Ref<DAE::Statement>,
    mut iextraArg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<DAE::Statement>,
                Type_a,
            ) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut oexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut oextraArg: Type_a;
    (oexps, oextraArg) = (::match_deref::match_deref! { match iexps {
        Deref @ metamodelica::ListNode::Nil => {
            let mut extraArg = iextraArg;
            (metamodelica::nil(), extraArg)
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: exps } => {
            let mut extraArg = iextraArg;
            let mut e = (*e).clone();
            (e, extraArg) = func(e.clone(), istmt.clone(), extraArg)?;
            (oexps, extraArg) = traverseDAEExpListStmt(exps, func, istmt, extraArg)?;
            (metamodelica::cons(e.clone(), oexps), extraArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oexps, oextraArg))
}

fn traverseDAEVarAttr<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut iextraArg: Type_a,
) -> Result<(Option<metamodelica::Ref<DAE::VariableAttributes>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut traversedDaeList: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut oextraArg: Type_a;
    (traversedDaeList, oextraArg) = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity, unit, displayUnit, min, max, start, fixed, nominal, stateSelectOption: stateSelect, uncertainOption: uncertainty, distributionOption: distribution, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut extraArg = iextraArg;
            let mut quantity = (*quantity).clone();
            let mut unit = (*unit).clone();
            let mut displayUnit = (*displayUnit).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut nominal = (*nominal).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func, extraArg)?;
            (unit, extraArg) = traverseDAEOptExp(unit.clone(), func, extraArg)?;
            (displayUnit, extraArg) = traverseDAEOptExp(displayUnit.clone(), func, extraArg)?;
            (min, extraArg) = traverseDAEOptExp(min.clone(), func, extraArg)?;
            (max, extraArg) = traverseDAEOptExp(max.clone(), func, extraArg)?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func, extraArg)?;
            (fixed, extraArg) = traverseDAEOptExp(fixed.clone(), func, extraArg)?;
            (nominal, extraArg) = traverseDAEOptExp(nominal.clone(), func, extraArg)?;
            (Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), nominal: nominal.clone(), stateSelectOption: stateSelect.clone(), uncertainOption: uncertainty.clone(), distributionOption: distribution.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg)
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity, min, max, start, fixed, uncertainOption: uncertainty, distributionOption: distribution, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut extraArg = iextraArg;
            let mut quantity = (*quantity).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func, extraArg)?;
            (min, extraArg) = traverseDAEOptExp(min.clone(), func, extraArg)?;
            (max, extraArg) = traverseDAEOptExp(max.clone(), func, extraArg)?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func, extraArg)?;
            (fixed, extraArg) = traverseDAEOptExp(fixed.clone(), func, extraArg)?;
            (Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), uncertainOption: uncertainty.clone(), distributionOption: distribution.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg)
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity, start, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut extraArg = iextraArg;
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func, extraArg)?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func, extraArg)?;
            (fixed, extraArg) = traverseDAEOptExp(fixed.clone(), func, extraArg)?;
            (Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg)
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_CLOCK { isProtected: _, finalPrefix: _ }) => {
            let mut extraArg = iextraArg;
            (attr, extraArg)
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity, start, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut extraArg = iextraArg;
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func, extraArg)?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func, extraArg)?;
            (fixed, extraArg) = traverseDAEOptExp(fixed.clone(), func, extraArg)?;
            (Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg)
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity, min, max, start, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut extraArg = iextraArg;
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            (quantity, extraArg) = traverseDAEOptExp(quantity.clone(), func, extraArg)?;
            (start, extraArg) = traverseDAEOptExp(start.clone(), func, extraArg)?;
            (Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() })), extraArg)
        },
        None => {
            let mut extraArg = iextraArg;
            (None, extraArg)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((traversedDaeList, oextraArg))
}

pub(crate) fn addComponentTypeOpt(
    mut inDae: DAE::DAElist,
    mut inPath: Option<metamodelica::Ref<Absyn::Path>>,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &(inPath) {
        Some(p) => {
            let mut dae = inDae;
            dae = addComponentType(dae, p.clone())?;
            dae
        },
        None => {
            let mut dae = inDae;
            dae
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDae)
}

pub fn addComponentType(mut dae: DAE::DAElist, mut newtype: metamodelica::Ref<Absyn::Path>) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist = dae;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(dae);
    }
    dae = (match dae {
        DAE::DAElist { elementLst: ref elts } => {
            let mut elts = elts.clone();
            elts = List::map1(elts.clone(), &addComponentType2, newtype)?;
            DAE::DAElist {
                elementLst: elts.clone(),
            }
        }
    });
    Ok(dae)
}

fn addComponentType2(
    mut elt: metamodelica::Ref<DAE::Element>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut elt: metamodelica::Ref<DAE::Element> = elt;
    elt = (match &*elt {
        DAE::Element::VAR {
            source: __elt_source, ..
        } => {
            assign_variant_field!(elt => DAE::Element::VAR; source = ElementSource::addElementSourceType(__elt_source.clone(), inPath)?);
            elt
        }
        _ => elt,
    });
    Ok(elt)
}

pub fn isExtFunction(mut elt: &DAE::Function) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(elt) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn functionName(mut elt: &DAE::Function) -> metamodelica::Ref<Absyn::Path> {
    let mut name: metamodelica::Ref<Absyn::Path>;
    name = (match elt.clone() {
        DAE::Function::FUNCTION {
            path: ref __esc_name, ..
        } => {
            name = __esc_name.clone();
            name.clone()
        }
        DAE::Function::RECORD_CONSTRUCTOR {
            path: ref __esc_name, ..
        } => {
            name = __esc_name.clone();
            name.clone()
        }
    });
    name
}

pub(crate) fn convertInlineTypeToBool(mut it: DAE::InlineType) -> bool {
    let mut b: bool;
    b = (match it {
        DAE::InlineType::NO_INLINE { .. } => false,
        _ => true,
    });
    b
}

pub fn inlineTypeEqual(mut it1: DAE::InlineType, mut it2: DAE::InlineType) -> bool {
    let mut b: bool;
    b = (match (it1, it2) {
        (DAE::InlineType::NORM_INLINE { .. }, DAE::InlineType::NORM_INLINE { .. }) => true,
        (DAE::InlineType::BUILTIN_EARLY_INLINE { .. }, DAE::InlineType::BUILTIN_EARLY_INLINE { .. }) => true,
        (DAE::InlineType::EARLY_INLINE { .. }, DAE::InlineType::EARLY_INLINE { .. }) => true,
        (DAE::InlineType::DEFAULT_INLINE { .. }, DAE::InlineType::DEFAULT_INLINE { .. }) => true,
        (DAE::InlineType::NO_INLINE { .. }, DAE::InlineType::NO_INLINE { .. }) => true,
        (DAE::InlineType::AFTER_INDEX_RED_INLINE { .. }, DAE::InlineType::AFTER_INDEX_RED_INLINE { .. }) => true,
        _ => false,
    });
    b
}

pub fn daeElements(mut dae: &DAE::DAElist) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    let mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    elts = (match dae.clone() {
        DAE::DAElist {
            elementLst: ref __esc_elts,
        } => {
            elts = __esc_elts.clone();
            elts.clone()
        }
    });
    elts
}

pub fn joinDaes(mut dae1: &DAE::DAElist, mut dae2: &DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (match (dae1.clone(), dae2.clone()) {
        (DAE::DAElist { elementLst: ref elts1 }, DAE::DAElist { elementLst: ref elts2 }) => {
            let mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            elts = listAppend(elts1.clone(), elts2.clone());
            DAE::DAElist { elementLst: elts }
        }
    });
    Ok(outDae)
}

pub fn joinDaeLst(mut idaeLst: &metamodelica::List<DAE::DAElist>) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match idaeLst {
        Deref @ metamodelica::ListNode::Cons { head: dae, tail: Deref @ metamodelica::ListNode::Nil } => {
            dae.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: dae, tail: daeLst } => {
            let mut dae1: DAE::DAElist;
            let mut dae = (*dae).clone();
            dae1 = joinDaeLst(daeLst)?;
            dae = joinDaes(metamodelica::AsArg::as_arg(&dae), &dae1)?;
            dae.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outDae)
}

pub fn splitElements(
    mut elements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>>,
    metamodelica::List<metamodelica::Ref<SCode::Comment>>,
)> {
    let mut variables: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut initialEquations: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut initialAlgorithms: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut equations: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut algorithms: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut classAttributes: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut constraints: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut externalObjects: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut stateMachineComps: metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>> =
        metamodelica::nil();
    let mut comments: metamodelica::List<metamodelica::Ref<SCode::Comment>> = metamodelica::nil();
    let mut split_comp: metamodelica::Ref<DAEDumpTypes::compWithSplitElements>;
    for mut e in &**elements {
        let () = (match &*e.clone() {
            DAE::Element::VAR { .. } => {
                variables = metamodelica::cons(e.clone(), variables);
                ()
            }
            DAE::Element::INITIALEQUATION { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIALDEFINE { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIAL_IF_EQUATION { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIAL_FOR_EQUATION { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIAL_ASSERT { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIAL_TERMINATE { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIAL_NORETCALL { .. } => {
                initialEquations = metamodelica::cons(e.clone(), initialEquations);
                ()
            }
            DAE::Element::INITIALALGORITHM { .. } => {
                initialAlgorithms = metamodelica::cons(e.clone(), initialAlgorithms);
                ()
            }
            DAE::Element::EQUATION { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::EQUEQUATION { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::ARRAY_EQUATION { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::COMPLEX_EQUATION { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::DEFINE { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::ASSERT { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::TERMINATE { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::IF_EQUATION { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::FOR_EQUATION { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::WHEN_EQUATION { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::REINIT { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::NORETCALL { .. } => {
                equations = metamodelica::cons(e.clone(), equations);
                ()
            }
            DAE::Element::ALGORITHM { .. } => {
                algorithms = metamodelica::cons(e.clone(), algorithms);
                ()
            }
            DAE::Element::CONSTRAINT { .. } => {
                constraints = metamodelica::cons(e.clone(), constraints);
                ()
            }
            DAE::Element::CLASS_ATTRIBUTES { .. } => {
                classAttributes = metamodelica::cons(e.clone(), classAttributes);
                ()
            }
            DAE::Element::EXTOBJECTCLASS { .. } => {
                externalObjects = metamodelica::cons(e.clone(), externalObjects);
                ()
            }
            DAE::Element::COMP {
                dAElist: __e_dAElist, ..
            } => {
                variables = listAppend(__e_dAElist.clone(), variables);
                ()
            }
            DAE::Element::FLAT_SM {
                dAElist: __e_dAElist,
                ident: __e_ident,
            } => {
                split_comp = splitComponent(
                    &(metamodelica::Ref::new(DAE::Element::COMP {
                        ident: __e_ident.clone(),
                        dAElist: __e_dAElist.clone(),
                        source: DAE::emptyElementSource().clone(),
                        comment: Some(metamodelica::Ref::new(SCode::Comment {
                            annotation_: None,
                            comment: Some(literal!("stateMachine")),
                        })),
                    })),
                )?;
                stateMachineComps = metamodelica::cons(split_comp, stateMachineComps);
                ()
            }
            DAE::Element::SM_COMP {
                componentRef: __e_componentRef,
                dAElist: __e_dAElist,
            } => {
                split_comp = splitComponent(
                    &(metamodelica::Ref::new(DAE::Element::COMP {
                        ident: ComponentReference::crefStr(metamodelica::AsArg::as_arg(&__e_componentRef))?,
                        dAElist: __e_dAElist.clone(),
                        source: DAE::emptyElementSource().clone(),
                        comment: Some(metamodelica::Ref::new(SCode::Comment {
                            annotation_: None,
                            comment: Some(literal!("state")),
                        })),
                    })),
                )?;
                stateMachineComps = metamodelica::cons(split_comp, stateMachineComps);
                ()
            }
            DAE::Element::COMMENT { cmt: __e_cmt } => {
                comments = metamodelica::cons(__e_cmt.clone(), comments);
                ()
            }
            _ => {
                Error::addInternalError(
                    literal!("DAEUtil.splitElements got unknown element."),
                    Absyn::dummyInfo.clone(),
                )?;
                return Err("fail");
            }
        });
    }
    variables = variables.reverse();
    initialEquations = initialEquations.reverse();
    initialAlgorithms = initialAlgorithms.reverse();
    equations = equations.reverse();
    algorithms = algorithms.reverse();
    classAttributes = classAttributes.reverse();
    constraints = constraints.reverse();
    externalObjects = externalObjects.reverse();
    stateMachineComps = stateMachineComps.reverse();
    Ok((
        variables,
        initialEquations,
        initialAlgorithms,
        equations,
        algorithms,
        classAttributes,
        constraints,
        externalObjects,
        stateMachineComps,
        comments,
    ))
}

pub(crate) fn splitComponent(
    mut component: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>> {
    let mut splitComponent: metamodelica::Ref<DAEDumpTypes::compWithSplitElements>;
    let mut v: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut ie: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut ia: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut e: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut a: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut co: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut o: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut ca: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut sm: metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>>;
    let mut split_el: metamodelica::Ref<DAEDumpTypes::splitElements>;
    splitComponent = (match &**component {
        DAE::Element::COMP {
            comment: __component_comment,
            dAElist: __component_dAElist,
            ident: __component_ident,
            ..
        } => {
            (v, ie, ia, e, a, co, o, ca, sm, _) = splitElements(metamodelica::AsArg::as_arg(&__component_dAElist))?;
            split_el = metamodelica::Ref::new(DAEDumpTypes::splitElements {
                v: v,
                ie: ie,
                ia: ia,
                e: e,
                a: a,
                co: co,
                o: o,
                ca: ca,
                sm: sm,
            });
            metamodelica::Ref::new(DAEDumpTypes::compWithSplitElements {
                name: __component_ident.clone(),
                spltElems: split_el,
                comment: __component_comment.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(splitComponent)
}

fn isIfEquation(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<()> {
    let () = (match &**inElement {
        DAE::Element::IF_EQUATION { .. } => (),
        DAE::Element::INITIAL_IF_EQUATION { .. } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn collectLocalDecls(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut inElements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outElements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    (outExp, outElements) = (match &*e {
        DAE::Exp::MATCHEXPRESSION { localDecls: ld1, .. } => {
            let mut ld2 = inElements.clone();
            let mut ld: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            ld = listAppend(ld1.clone(), ld2);
            (e, ld)
        }
        _ => (e, inElements),
    });
    (outExp, outElements)
}

pub fn getUniontypePaths(
    mut funcs: metamodelica::List<DAE::Function>,
    mut els: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    let mut paths1: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    let mut paths2: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    outPaths = 'mc: {
        let __mc_input = &**els;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = outPaths.clone();
                    let mut paths1: metamodelica::List<metamodelica::Ref<Absyn::Path>> = paths1.clone();
                    let mut paths2: metamodelica::List<metamodelica::Ref<Absyn::Path>> = paths2.clone();
                    paths1 = getUniontypePathsFunctions(funcs.clone())?;
                    paths2 = getUniontypePathsElements(els, metamodelica::nil())?;
                    outPaths = listAppend(paths1.clone(), paths2.clone());
                    Ok((outPaths.clone(), outPaths.clone(), paths1.clone(), paths2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outPaths = __wb0;
            paths1 = __wb1;
            paths2 = __wb2;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outPaths)
}

fn getUniontypePathsFunctions(
    mut elements: metamodelica::List<DAE::Function>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outPaths = (::match_deref::match_deref! { match &(elements.clone()) {
        _ => {
            let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut els1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut els2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let (_, (_, __pa0)) = traverseDAEFunctions(elements.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(collectLocalDecls, metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::Element>>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::Element>>)> + 'static>), metamodelica::nil()))?;
            els1 = metamodelica::Own::own(__pa0);
            els2 = getFunctionsElements(elements)?;
            els = listAppend(els1, els2);
            outPaths = getUniontypePathsElements(&els, metamodelica::nil())?;
            outPaths
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPaths)
}

fn getUniontypePathsElements<'__b>(
    mut elements: &'__b metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match elements {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(List::applyAndFold(&acc, &fnptr!(listAppend, metamodelica::List<metamodelica::Ref<Absyn::Path>>, _), &move |__a0: metamodelica::Ref<DAE::Type>| Types::getUniontypePaths(&__a0), metamodelica::nil())?)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { ty: ft, .. }, tail: rest } => {
                let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                tys = Types::getAllInnerTypesOfType(ft.clone(), &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::uniontypeFilter(&__a0)) })?;
                { (elements, acc) = (rest, listAppend(tys, acc)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (elements, acc) = (rest, acc); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getDAEDeclsFromValueblocks(
    mut exps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    let mut outEls: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    for mut ex in &**exps {
        let () = (match &*ex.clone() {
            DAE::Exp::MATCHEXPRESSION { localDecls: els1, .. } => {
                outEls = List::append_reverse(metamodelica::AsArg::as_arg(&els1), outEls);
                ()
            }
            _ => (),
        });
    }
    outEls = metamodelica::Dangerous::listReverseInPlace(outEls);
    outEls
}

// protected function transformDerInline "This is not used.
//   Simple euler inline of the equation system; only does explicit euler, and only der(cref)"
//   input DAE.DAElist dae;
//   output DAE.DAElist d;
// algorithm
//   d := matchcontinue (dae)
//     local
//       HashTable.HashTable ht;
//     case _
//       equation
//         false = Flags.isSet(Flags.FRONTEND_INLINE_EULER);
//       then dae;
//     case _
//       equation
//         ht = HashTable.emptyHashTable();
//         (d,_,ht) = traverseDAE(dae,AvlTreePathFunction.Tree.EMPTY(),simpleInlineDerEuler,ht);
//       then d;
//   end matchcontinue;
// end transformDerInline;
//
// protected function simpleInlineDerEuler "This is not used.
//   Helper function of transformDerInline."
//   input tuple<DAE.Exp,HashTable.HashTable> itpl;
//   output tuple<DAE.Exp,HashTable.HashTable> otpl;
// algorithm
//   otpl := matchcontinue (itpl)
//     local
//       DAE.ComponentRef cr,cref_1,cref_2;
//       HashTable.HashTable crs0,crs1;
//       DAE.Exp exp,e1,e2;
//
//     case ((DAE.CALL(path=Absyn.IDENT("der"),expLst={exp as DAE.CREF(componentRef = cr, ty = DAE.T_REAL(varLst = _))}),crs0))
//       equation
//         cref_1 = ComponentReferenceBasics.makeCrefQual("$old",DAE.T_REAL_DEFAULT,{},cr);
//         cref_2 = ComponentReferenceBasics.makeCrefIdent("$current_step_size",DAE.T_REAL_DEFAULT,{});
//         e1 = Expression.makeCrefExp(cref_1,DAE.T_REAL_DEFAULT);
//         e2 = Expression.makeCrefExp(cref_2,DAE.T_REAL_DEFAULT);
//         exp = DAE.BINARY(
//                 DAE.BINARY(exp, DAE.SUB(DAE.T_REAL_DEFAULT), e1),
//                 DAE.DIV(DAE.T_REAL_DEFAULT),
//                 e2);
//         crs1 = BaseHashTable.add((cr,0),crs0);
//       then
//         ((exp,crs1));
//
//     case ((exp,crs0)) then ((exp,crs0));
//
//   end matchcontinue;
// end simpleInlineDerEuler;
pub fn transformationsBeforeBackend(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut inDAElist: DAE::DAElist,
    mut stateMachineToDataFlow: &dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, DAE::DAElist) -> Result<DAE::DAElist>,
) -> Result<DAE::DAElist> {
    pub type StateMachineToDataFlowFunc =
        std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, DAE::DAElist) -> Result<DAE::DAElist> + 'static>;

    let mut outDAElist: DAE::DAElist;
    let mut dAElist: DAE::DAElist;
    let mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut ht: metamodelica::Ref<AvlSetCR::Tree>;
    dAElist = stateMachineToDataFlow(cache.clone(), env, inDAElist)?;
    if Flags::isSet(Flags::SCODE_INST.clone())? {
        let DAE::DAE { elementLst: __pa0 } = dAElist;
        elts = metamodelica::Own::own(__pa0);
        outDAElist = DAE::DAElist { elementLst: elts };
    } else {
        let DAE::DAE { elementLst: __pa1 } = dAElist;
        elts = metamodelica::Own::own(__pa1);
        ht = FCore::getEvaluatedParams(cache)?;
        elts = List::map1(elts, &makeEvaluatedParamFinal, ht.clone())?;
        if Flags::isSet(Flags::PRINT_STRUCTURAL.clone())? {
            transformationsBeforeBackendNotification(&ht)?;
        }
        outDAElist = DAE::DAElist { elementLst: elts };
    }
    Ok(outDAElist)
}

fn transformationsBeforeBackendNotification(mut ht: &metamodelica::Ref<AvlSetCR::Tree>) -> Result<()> {
    let mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut strs: metamodelica::List<ArcStr>;
    let mut r#str: ArcStr;
    crs = AvlSetCR::listKeys(ht, metamodelica::nil());
    if !((crs).is_empty()) {
        strs = List::map(crs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
            ComponentReferenceBasics::printComponentRefStr(&__a0)
        })?;
        r#str = stringDelimitList(strs, literal!(", "));
        Error::addMessage(Error::NOTIFY_FRONTEND_STRUCTURAL_PARAMETERS.clone(), list![r#str])?;
    }
    Ok(())
}

fn makeEvaluatedParamFinal(
    mut inElement: metamodelica::Ref<DAE::Element>,
    mut ht: metamodelica::Ref<AvlSetCR::Tree>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outElement: metamodelica::Ref<DAE::Element>;
    outElement = (match &*inElement.clone() {
        DAE::Element::VAR {
            componentRef: cr,
            kind: DAE::VarKind::PARAM { .. },
            variableAttributesOption: varOpt,
            ..
        } => {
            let mut elt: metamodelica::Ref<DAE::Element>;
            elt = if (AvlSetCR::hasKey(ht, cr.clone())?) {
                setVariableAttributes(inElement, setFinalAttr(varOpt.clone(), true)?)?
            } else {
                inElement
            };
            elt
        }
        DAE::Element::COMP {
            ident: id,
            dAElist: elts,
            source,
            comment: cmt,
        } => {
            let mut elts = (*elts).clone();
            elts = List::map1(elts.clone(), &makeEvaluatedParamFinal, ht)?;
            metamodelica::Ref::new(DAE::Element::COMP {
                ident: id.clone(),
                dAElist: elts.clone(),
                source: source.clone(),
                comment: cmt.clone(),
            })
        }
        _ => inElement,
    });
    Ok(outElement)
}

pub fn setBindingSource(
    mut inBinding: metamodelica::Ref<DAE::Binding>,
    mut bindingSource: DAE::BindingSource,
) -> metamodelica::Ref<DAE::Binding> {
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    outBinding = (match &*inBinding {
        DAE::Binding::UNBOUND { .. } => inBinding,
        DAE::Binding::EQBOUND {
            exp,
            evaluatedExp,
            constant_: cnst,
            source: _,
        } => metamodelica::Ref::new(DAE::Binding::EQBOUND {
            exp: exp.clone(),
            evaluatedExp: evaluatedExp.clone(),
            constant_: cnst.clone(),
            source: bindingSource,
        }),
        DAE::Binding::VALBOUND { valBound, source: _ } => metamodelica::Ref::new(DAE::Binding::VALBOUND {
            valBound: valBound.clone(),
            source: bindingSource,
        }),
    });
    outBinding
}

pub(crate) fn printBindingExpStr(mut binding: &metamodelica::Ref<DAE::Binding>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**binding {
        DAE::Binding::UNBOUND { .. } => {
            literal!("")
        }
        DAE::Binding::EQBOUND { exp: e, .. } => {
            r#str = ExpressionBasics::printExpStr(e.clone())?;
            r#str
        }
        DAE::Binding::VALBOUND { valBound: v, .. } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ValuesDump::valString(v)?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
    });
    Ok(r#str)
}

pub fn collectValueblockFunctionRefVars(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut acc: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<Absyn::Path>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outAcc: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    (outExp, outAcc) = (match &*exp {
        DAE::Exp::MATCHEXPRESSION { localDecls: decls, .. } => {
            outAcc = List::fold(
                metamodelica::AsArg::as_arg(&decls),
                &move |__a0: metamodelica::Ref<DAE::Element>,
                       __a1: metamodelica::List<metamodelica::Ref<Absyn::Path>>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(collectFunctionRefVarPaths(&__a0, __a1))
                },
                acc,
            )?;
            (exp, outAcc)
        }
        _ => (exp, acc),
    });
    Ok((outExp, outAcc))
}

pub fn collectFunctionRefVarPaths(
    mut inElem: &metamodelica::Ref<DAE::Element>,
    mut acc: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut outAcc: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outAcc = (::match_deref::match_deref! { match inElem {
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_FUNCTION { path, .. }, .. } => {
            metamodelica::cons(path.clone(), acc)
        },
        _ => {
            acc
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAcc
}

pub fn addFunctionDefinition(mut ifunc: DAE::Function, mut iFuncDef: DAE::FunctionDefinition) -> DAE::Function {
    let mut func: DAE::Function = ifunc;
    let () = (match func.clone() {
        DAE::Function::FUNCTION { .. } => {
            let __owned_variant_functions_0 =
                List::appendElt(iFuncDef, var_field!(func.functions, DAE::Function::FUNCTION).clone());
            if let DAE::Function::FUNCTION { functions, .. } = &mut func {
                *functions = __owned_variant_functions_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION");
            }
            ()
        }
        _ => (),
    });
    func
}

pub(crate) fn getFunctionsInfo(
    mut ft: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut strs: metamodelica::List<ArcStr>;
    strs = (match &**ft {
        _ => {
            let mut lst: metamodelica::List<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)>;
            lst = AvlTreePathFunction::toList(ft, metamodelica::nil());
            strs = List::map(lst, &move |__a0: (
                metamodelica::Ref<Absyn::Path>,
                Option<DAE::Function>,
            )| getInfo(&__a0))?;
            strs = List::sort(
                strs,
                (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
                }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            )?;
            strs
        }
    });
    Ok(strs)
}

pub(crate) fn getInfo(mut tpl: &(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match &(tpl) {
        (p, None) => {
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" [invalid]")); ArcStr::from(__mm_s) };
            r#str
        },
        (p, Some(_)) => {
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" [valid]  ")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

fn showCacheFuncs(mut tree: &metamodelica::Ref<AvlTreePathFunction::Tree>) -> Result<()> {
    let () = (match &**tree {
        _ => {
            let mut msg: ArcStr;
            msg = stringDelimitList(getFunctionsInfo(tree)?, literal!("\n  "));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Cache has: \n  "));
                __mm_s.push_str(&*msg);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            ()
        }
    });
    Ok(())
}

pub fn setAttrVariability(
    mut attr: metamodelica::Ref<DAE::Attributes>,
    mut var: SCode::Variability,
) -> metamodelica::Ref<DAE::Attributes> {
    let mut attr: metamodelica::Ref<DAE::Attributes> = attr;
    assign_field!(attr.variability = var);
    attr
}

pub fn getAttrVariability(mut attr: &metamodelica::Ref<DAE::Attributes>) -> SCode::Variability {
    let mut var: SCode::Variability = attr.variability.clone();
    var
}

pub(crate) fn setAttrDirection(
    mut attr: metamodelica::Ref<DAE::Attributes>,
    mut dir: Absyn::Direction,
) -> metamodelica::Ref<DAE::Attributes> {
    let mut attr: metamodelica::Ref<DAE::Attributes> = attr;
    assign_field!(attr.direction = dir);
    attr
}

pub fn getAttrDirection(mut attr: &metamodelica::Ref<DAE::Attributes>) -> Absyn::Direction {
    let mut dir: Absyn::Direction = attr.direction.clone();
    dir
}

pub fn setAttrInnerOuter(
    mut attr: metamodelica::Ref<DAE::Attributes>,
    mut io: Absyn::InnerOuter,
) -> metamodelica::Ref<DAE::Attributes> {
    let mut attr: metamodelica::Ref<DAE::Attributes> = attr;
    assign_field!(attr.innerOuter = io);
    attr
}

pub fn getAttrInnerOuter(mut attr: &metamodelica::Ref<DAE::Attributes>) -> Absyn::InnerOuter {
    let mut io: Absyn::InnerOuter = attr.innerOuter.clone();
    io
}

pub fn translateSCodeAttrToDAEAttr(
    mut inAttributes: SCode::Attributes,
    mut inPrefixes: &metamodelica::Ref<SCode::Prefixes>,
) -> metamodelica::Ref<DAE::Attributes> {
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut ct: SCode::ConnectorType;
    let mut prl: SCode::Parallelism;
    let mut var: SCode::Variability;
    let mut dir: Absyn::Direction;
    let mut io: Absyn::InnerOuter;
    let mut vis: SCode::Visibility;
    let SCode::ATTR {
        connectorType: __pa0,
        parallelism: __pa1,
        variability: __pa2,
        direction: __pa3,
        ..
    } = inAttributes;
    ct = metamodelica::Own::own(__pa0);
    prl = metamodelica::Own::own(__pa1);
    var = metamodelica::Own::own(__pa2);
    dir = metamodelica::Own::own(__pa3);
    let __arc6 = &(*inPrefixes);
    let SCode::PREFIXES {
        innerOuter: __pa4,
        visibility: __pa5,
        ..
    } = &**__arc6;
    io = metamodelica::Own::own(__pa4);
    vis = metamodelica::Own::own(__pa5);
    outAttributes = metamodelica::Ref::new(DAE::Attributes {
        connectorType: toConnectorTypeNoState(ct, None),
        parallelism: prl,
        variability: var,
        direction: dir,
        innerOuter: io,
        visibility: vis,
    });
    outAttributes
}

pub fn varName(mut var: &metamodelica::Ref<DAE::Element>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*var)) {
        Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    Ok(name)
}

pub fn typeVarIdent(mut var: &metamodelica::Ref<DAE::Var>) -> ArcStr {
    let mut name: ArcStr;
    let __arc1 = &(*var);
    let DAE::TYPES_VAR { name: __pa0, .. } = &**__arc1;
    name = metamodelica::Own::own(__pa0);
    name
}

pub(crate) fn typeVarIdentEqual(mut var: &metamodelica::Ref<DAE::Var>, mut name: &ArcStr) -> bool {
    let mut b: bool;
    let mut name2: ArcStr;
    let __arc1 = &(*var);
    let DAE::TYPES_VAR { name: __pa0, .. } = &**__arc1;
    name2 = metamodelica::Own::own(__pa0);
    b = stringEq(&name, &name2);
    b
}

pub fn varType(mut var: &metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::Type> {
    let mut type_: metamodelica::Ref<DAE::Type>;
    let __arc1 = &(*var);
    let DAE::TYPES_VAR { ty: __pa0, .. } = &**__arc1;
    type_ = metamodelica::Own::own(__pa0);
    type_
}

pub fn bindingExp(mut bind: &metamodelica::Ref<DAE::Binding>) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut exp: Option<metamodelica::Ref<DAE::Exp>>;
    exp = (::match_deref::match_deref! { match bind {
        Deref @ DAE::Binding::UNBOUND { .. } => {
            None
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(v), .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = ValuesUtil::valueExp(v.clone(), None)?;
            Some(e)
        },
        Deref @ DAE::Binding::EQBOUND { exp: e, .. } => {
            Some(e.clone())
        },
        Deref @ DAE::Binding::VALBOUND { valBound: v, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = ValuesUtil::valueExp(v.clone(), None)?;
            Some(e)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn isBound(mut inBinding: &metamodelica::Ref<DAE::Binding>) -> bool {
    let mut outIsBound: bool;
    outIsBound = (match &**inBinding {
        DAE::Binding::UNBOUND { .. } => false,
        _ => true,
    });
    outIsBound
}

pub(crate) fn isCompleteFunction(mut f: &DAE::Function) -> bool {
    let mut isComplete: bool;
    isComplete = (match f.clone() {
        DAE::Function::RECORD_CONSTRUCTOR { .. } => true,
        DAE::Function::FUNCTION {
            functions: mut functions,
            ..
        } => isCompleteFunctionBody(metamodelica::AsArg::as_arg(&functions)),
        _ => false,
    });
    isComplete
}

pub(crate) fn isCompleteFunctionBody(mut functions: &metamodelica::List<DAE::FunctionDefinition>) -> bool {
    let mut isComplete: bool;
    isComplete = 'mc: {
        let __mc_input = &**functions;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: _ } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: els }, tail: _ } => {
                    let mut a: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    (_, _, _, _, a, _, _, _, _, _) = splitElements(metamodelica::AsArg::as_arg(&els))?;
                    let false = ((a).is_empty()) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. }, tail: rest } => {
                    Ok(isCompleteFunctionBody(metamodelica::AsArg::as_arg(&rest)))
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
    isComplete
}

pub(crate) fn isNotCompleteFunction(mut f: &DAE::Function) -> bool {
    let mut isNotComplete: bool;
    isNotComplete = !(isCompleteFunction(f));
    isNotComplete
}

pub(crate) fn setAttributeDirection(
    mut inDirection: Absyn::Direction,
    mut inAttributes: &metamodelica::Ref<DAE::Attributes>,
) -> metamodelica::Ref<DAE::Attributes> {
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut ct: metamodelica::Ref<DAE::ConnectorType>;
    let mut p: SCode::Parallelism;
    let mut var: SCode::Variability;
    let mut io: Absyn::InnerOuter;
    let mut vis: SCode::Visibility;
    let __arc5 = &(*inAttributes);
    let DAE::ATTR {
        connectorType: __pa0,
        parallelism: __pa1,
        variability: __pa2,
        direction: _,
        innerOuter: __pa3,
        visibility: __pa4,
    } = &**__arc5;
    ct = metamodelica::Own::own(__pa0);
    p = metamodelica::Own::own(__pa1);
    var = metamodelica::Own::own(__pa2);
    io = metamodelica::Own::own(__pa3);
    vis = metamodelica::Own::own(__pa4);
    outAttributes = metamodelica::Ref::new(DAE::Attributes {
        connectorType: ct,
        parallelism: p,
        variability: var,
        direction: inDirection,
        innerOuter: io,
        visibility: vis,
    });
    outAttributes
}

pub(crate) fn varKindEqual(mut inVariability1: DAE::VarKind, mut inVariability2: DAE::VarKind) -> Result<bool> {
    let mut outIsEqual: bool;
    outIsEqual = (match (inVariability1, inVariability2) {
        (DAE::VarKind::VARIABLE { .. }, DAE::VarKind::VARIABLE { .. }) => true,
        (DAE::VarKind::DISCRETE { .. }, DAE::VarKind::DISCRETE { .. }) => true,
        (DAE::VarKind::CONST { .. }, DAE::VarKind::CONST { .. }) => true,
        (DAE::VarKind::PARAM { .. }, DAE::VarKind::PARAM { .. }) => true,
        _ => return Err("match: no arm matched"),
    });
    Ok(outIsEqual)
}

pub fn varDirectionEqual(mut inDirection1: DAE::VarDirection, mut inDirection2: DAE::VarDirection) -> bool {
    let mut outIsEqual: bool;
    outIsEqual = (match (inDirection1, inDirection2) {
        (DAE::VarDirection::BIDIR { .. }, DAE::VarDirection::BIDIR { .. }) => true,
        (DAE::VarDirection::INPUT { .. }, DAE::VarDirection::INPUT { .. }) => true,
        (DAE::VarDirection::OUTPUT { .. }, DAE::VarDirection::OUTPUT { .. }) => true,
        _ => false,
    });
    outIsEqual
}

pub(crate) fn isComplexVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut outIsComplex: bool;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __arc1 = &(*inVar);
    let DAE::TYPES_VAR { ty: __pa0, .. } = &**__arc1;
    ty = metamodelica::Own::own(__pa0);
    outIsComplex = Types::isComplexType(&ty);
    outIsComplex
}

pub(crate) fn getElements(mut inDAE: DAE::DAElist) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let DAE::DAE { elementLst: __pa0 } = inDAE;
    outElements = metamodelica::Own::own(__pa0);
    outElements
}

pub(crate) fn mkEmptyVar(mut name: ArcStr) -> metamodelica::Ref<DAE::Var> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    outVar = metamodelica::Ref::new(DAE::Var {
        name: name,
        attributes: DAE::dummyAttrVar().clone(),
        ty: DAE::T_UNKNOWN_DEFAULT().clone(),
        binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        bind_from_outside: false,
        constOfForIteratorRange: None,
    });
    outVar
}

pub(crate) fn sortDAEInModelicaCodeOrder(
    mut inShouldSort: bool,
    mut inElements: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inDae: DAE::DAElist,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (::match_deref::match_deref! { match &((inShouldSort, inElements.clone(), inDae.clone())) {
        (false, _, _) => {
            inDae
        },
        (true, Deref @ metamodelica::ListNode::Nil, _) => {
            inDae
        },
        (true, _, DAE::DAElist { elementLst: els }) => {
            let mut els = (*els).clone();
            els = sortDAEElementsInModelicaCodeOrder(inElements, els.clone())?;
            DAE::DAElist { elementLst: els.clone() }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outDae)
}

fn sortDAEElementsInModelicaCodeOrder(
    mut inElements: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inDaeEls: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outDaeEls: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Element>> = inDaeEls.clone();
    for mut e in &**inElements {
        let () = (::match_deref::match_deref! { match &(e.clone()) {
            (Deref @ SCode::Element::COMPONENT { name, .. }, _) => {
                let mut named: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                (named, rest) = splitVariableNamed(rest, metamodelica::AsArg::as_arg(&name), metamodelica::nil(), metamodelica::nil())?;
                outDaeEls = List::append_reverse(&named, outDaeEls);
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outDaeEls = List::append_reverse(&inDaeEls, outDaeEls);
    outDaeEls = metamodelica::Dangerous::listReverseInPlace(outDaeEls);
    Ok(outDaeEls)
}

fn splitVariableNamed<'__b>(
    mut inElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inName: &'__b ArcStr,
    mut inAccNamed: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inAccRest: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElementLst, inAccNamed.clone(), inAccRest.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok((inAccNamed.reverse(), inAccRest.reverse()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: x @ Deref @ DAE::Element::VAR { componentRef: cr, .. }, tail: lst }, accNamed, accRest) => {
                let mut equal: bool;
                let mut accNamed = (*accNamed).clone();
                let mut accRest = (*accRest).clone();
                equal = stringEq(&(ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?), &inName);
                accNamed = List::consOnTrue(equal, x.clone(), accNamed.clone());
                accRest = List::consOnTrue(boolNot(equal), x.clone(), accRest.clone());
                { (inElementLst, inName, inAccNamed, inAccRest) = (lst.clone(), inName, accNamed.clone(), accRest.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: x, tail: lst }, accNamed, accRest) => {
                let mut accNamed = (*accNamed).clone();
                let mut accRest = (*accRest).clone();
                { (inElementLst, inName, inAccNamed, inAccRest) = (lst.clone(), inName, accNamed.clone(), metamodelica::cons(x.clone(), accRest.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn daeDescription(mut inDAE: &DAE::DAElist) -> ArcStr {
    let mut comment: ArcStr;
    comment = (::match_deref::match_deref! { match &(inDAE) {
        DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { comment: Some(Deref @ SCode::Comment { comment: Some(__esc_comment), .. }), .. }, tail: _ } } => {
            comment = (*__esc_comment).clone();
            comment.clone()
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    comment
}

pub fn replaceCallAttrType(
    mut caIn: metamodelica::Ref<DAE::CallAttributes>,
    mut typeIn: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::CallAttributes> {
    let mut caOut: metamodelica::Ref<DAE::CallAttributes>;
    caOut = caIn;
    assign_field!(caOut.ty = typeIn.clone());
    if Types::isTuple(&typeIn) {
        assign_field!(caOut.tuple_ = true);
    }
    caOut
}

pub fn funcIsRecord(mut func: &DAE::Function) -> bool {
    let mut isRec: bool;
    isRec = (match func.clone() {
        DAE::Function::RECORD_CONSTRUCTOR { .. } => true,
        _ => false,
    });
    isRec
}

pub(crate) fn funcArgDim(mut argIn: &metamodelica::Ref<DAE::FuncArg>) -> Result<i32> {
    let mut dim: i32;
    dim = (::match_deref::match_deref! { match argIn {
        Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_ARRAY { dims: arrayDims, .. }, .. } => {
            List::applyAndFold(metamodelica::AsArg::as_arg(&arrayDims), &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::dimensionSize(&__a0), 0)?
        },
        Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_ENUMERATION { names, .. }, .. } => {
            ((names).len() as i32)
        },
        _ => {
            1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub fn toDAEInnerOuter(mut ioIn: Absyn::InnerOuter) -> DAE::VarInnerOuter {
    let mut ioOut: DAE::VarInnerOuter;
    ioOut = (match ioIn {
        Absyn::InnerOuter::INNER { .. } => openmodelica_frontend_types::DAE::VarInnerOuter::INNER,
        Absyn::InnerOuter::OUTER { .. } => openmodelica_frontend_types::DAE::VarInnerOuter::OUTER,
        Absyn::InnerOuter::INNER_OUTER { .. } => openmodelica_frontend_types::DAE::VarInnerOuter::INNER_OUTER,
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
    });
    ioOut
}

pub fn getAssertConditionCrefs(
    mut stmt: &metamodelica::Ref<DAE::Statement>,
    mut crefsIn: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut crefsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    crefsOut = (match &**stmt {
        DAE::Statement::STMT_ASSERT { cond, .. } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crefs = Expression::extractCrefsFromExp(cond.clone())?;
            listAppend(crefsIn, crefs)
        }
        _ => crefsIn,
    });
    Ok(crefsOut)
}

pub fn getSubscriptIndex(mut iSubscript: &metamodelica::Ref<DAE::Subscript>) -> i32 {
    let mut oIndex: i32;
    let mut index: i32;
    oIndex = (::match_deref::match_deref! { match iSubscript {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __esc_index } } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { index: __esc_index, .. } } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        _ => -1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oIndex
}

pub(crate) fn bindingValue(
    mut inBinding: &metamodelica::Ref<DAE::Binding>,
) -> Option<metamodelica::Ref<Values::Value>> {
    let mut outValue: Option<metamodelica::Ref<Values::Value>>;
    outValue = (match &**inBinding {
        DAE::Binding::EQBOUND {
            evaluatedExp: __inBinding_evaluatedExp,
            ..
        } => __inBinding_evaluatedExp.clone(),
        DAE::Binding::VALBOUND {
            valBound: __inBinding_valBound,
            ..
        } => Some(__inBinding_valBound.clone()),
        _ => None,
    });
    outValue
}

pub fn statementsContainReturn(mut stmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>) -> Result<bool> {
    let mut b: bool;
    (_, b) = traverseDAEStmts(
        stmts,
        &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Statement>, __a2: bool| {
            statementsContainReturn2(__a0, &__a1, __a2)
        },
        false,
    )?;
    Ok(b)
}

pub fn statementsContainTryBlock(mut stmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>) -> Result<bool> {
    let mut b: bool;
    (_, b) = traverseDAEStmts(
        stmts,
        &move |__a0: metamodelica::Ref<DAE::Exp>,
               __a1: metamodelica::Ref<DAE::Statement>,
               __a2: bool|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(statementsContainTryBlock2(__a0, &__a1, __a2))
        },
        false,
    )?;
    Ok(b)
}

fn statementsContainReturn2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inStmt: &metamodelica::Ref<DAE::Statement>,
    mut b: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut ob: bool = b;
    if !(b) {
        ob = (match &**inStmt {
            DAE::Statement::STMT_RETURN { .. } => true,
            _ => {
                (match &*inExp {
                    DAE::Exp::MATCHEXPRESSION { cases, .. } => {
                        let mut body: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                        for mut c in &*cases.clone() {
                            if !(ob) {
                                let __arc1 = c.clone();
                                let DAE::CASE { body: __pa0, .. } = &*__arc1;
                                body = metamodelica::Own::own(__pa0);
                                ob = statementsContainReturn(&body)?;
                            }
                        }
                        ob
                    }
                    _ => false,
                })
            }
        });
    }
    Ok((outExp, ob))
}

fn statementsContainTryBlock2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inStmt: &metamodelica::Ref<DAE::Statement>,
    mut b: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut ob: bool = b;
    if !(b) {
        ob = (match &*inExp {
            DAE::Exp::MATCHEXPRESSION {
                matchType: DAE::MatchType::MATCHCONTINUE { .. },
                ..
            } => true,
            _ => false,
        });
    }
    (outExp, ob)
}

pub(crate) fn getVarBinding(
    mut iels: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut icr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut obnd: Option<metamodelica::Ref<DAE::Exp>>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    obnd = None;
    for mut i in &**iels {
        obnd = (::match_deref::match_deref! { match &(i.clone()) {
            Deref @ DAE::Element::VAR { componentRef: __esc_cr, binding: __esc_obnd, .. } => {
                obnd = (*__esc_obnd).clone();
                cr = (*__esc_cr).clone();
                if ComponentReferenceBasics::crefEqualNoStringCompare(icr, metamodelica::AsArg::as_arg(&cr))? {
                    return Ok(obnd.clone());
                }
                obnd.clone()
            },
            Deref @ DAE::Element::DEFINE { componentRef: __esc_cr, exp: __esc_e, .. } => {
                cr = (*__esc_cr).clone();
                e = (*__esc_e).clone();
                obnd = Some(e.clone());
                if ComponentReferenceBasics::crefEqualNoStringCompare(icr, metamodelica::AsArg::as_arg(&cr))? {
                    return Ok(obnd);
                }
                obnd
            },
            Deref @ DAE::Element::INITIALDEFINE { componentRef: __esc_cr, exp: __esc_e, .. } => {
                cr = (*__esc_cr).clone();
                e = (*__esc_e).clone();
                obnd = Some(e.clone());
                if ComponentReferenceBasics::crefEqualNoStringCompare(icr, metamodelica::AsArg::as_arg(&cr))? {
                    return Ok(obnd);
                }
                obnd
            },
            Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, scalar: __esc_e, .. } => {
                cr = (*__esc_cr).clone();
                e = (*__esc_e).clone();
                obnd = Some(e.clone());
                if ComponentReferenceBasics::crefEqualNoStringCompare(icr, metamodelica::AsArg::as_arg(&cr))? {
                    return Ok(obnd);
                }
                obnd
            },
            Deref @ DAE::Element::EQUATION { exp: __esc_e, scalar: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, .. } => {
                e = (*__esc_e).clone();
                cr = (*__esc_cr).clone();
                obnd = Some(e.clone());
                if ComponentReferenceBasics::crefEqualNoStringCompare(icr, metamodelica::AsArg::as_arg(&cr))? {
                    return Ok(obnd);
                }
                obnd
            },
            Deref @ DAE::Element::INITIALEQUATION { exp1: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, exp2: __esc_e, .. } => {
                cr = (*__esc_cr).clone();
                e = (*__esc_e).clone();
                obnd = Some(e.clone());
                if ComponentReferenceBasics::crefEqualNoStringCompare(icr, metamodelica::AsArg::as_arg(&cr))? {
                    return Ok(obnd);
                }
                obnd
            },
            Deref @ DAE::Element::INITIALEQUATION { exp1: __esc_e, exp2: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, .. } => {
                e = (*__esc_e).clone();
                cr = (*__esc_cr).clone();
                obnd = Some(e.clone());
                if ComponentReferenceBasics::crefEqualNoStringCompare(icr, metamodelica::AsArg::as_arg(&cr))? {
                    return Ok(obnd);
                }
                obnd
            },
            _ => obnd,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(obnd)
}

pub(crate) fn evaluateCref(
    mut icr: &metamodelica::Ref<DAE::ComponentRef>,
    mut iels: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut oexp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut ee: metamodelica::Ref<DAE::Exp>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut oexps: metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>;
    oexp = getVarBinding(&iels, icr)?;
    if (oexp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(oexp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        (e, _) = ExpressionSimplify::simplify(e)?;
        if Expression::isConst(e.clone())? {
            oexp = Some(e);
            return Ok(oexp);
        }
        crefs = Expression::getAllCrefs(e.clone())?;
        oexps = List::map1(
            crefs.clone(),
            &move |__a0: metamodelica::Ref<DAE::ComponentRef>,
                   __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>| evaluateCref(&__a0, __a1),
            iels,
        )?;
        for mut c in &*crefs {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(oexps) {
                Deref @ metamodelica::ListNode::Cons { head: Some(__pa1), tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ee = metamodelica::Own::own(__pa1);
            oexps = metamodelica::Own::own(__pa2);
            (e, _) = Expression::replaceCref(e, (c.clone(), ee))?;
            (e, _) = ExpressionSimplify::simplify(e)?;
        }
        oexp = Some(e);
    }
    Ok(oexp)
}

pub(crate) fn evaluateExp(
    mut iexp: metamodelica::Ref<DAE::Exp>,
    mut iels: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut oexp: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut ee: metamodelica::Ref<DAE::Exp>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut oexps: metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>;
    if Expression::isConst(iexp.clone())? {
        oexp = Some(iexp);
        return Ok(oexp);
    }
    match '__try0: {
        e = iexp.clone();
        crefs = unwrap_break_err!(Expression::getAllCrefs(e.clone()), '__try0);
        oexps = unwrap_break_err!(List::map1(crefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>| evaluateCref(&__a0, __a1), iels.clone()), '__try0);
        for mut c in &*crefs {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(oexps.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: Some(__pa1), tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            ee = metamodelica::Own::own(__pa1);
            oexps = metamodelica::Own::own(__pa2);
            e = unwrap_break_err!(Expression::replaceCrefBottomUp(e.clone(), c.clone(), ee.clone()), '__try0);
            (e, _) = unwrap_break_err!(ExpressionSimplify::simplify(e.clone()), '__try0);
        }
        oexp = Some(e.clone());
        Ok::<_, &'static str>((oexp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            oexp = __try0_o0;
        }
        Err(_) => {
            oexp = None;
        }
    }
    Ok(oexp)
}

pub(crate) fn replaceCrefInDAEElements(
    mut inElements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut repl: VarTransform::VariableReplacements;
    repl = VarTransform::emptyReplacements();
    repl = VarTransform::addReplacement(repl, inCref, inExp)?;
    (outElements, _) = traverseDAEElementList(
        inElements,
        (std::sync::Arc::new(replaceCrefBottomUp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        VarTransform::VariableReplacements,
                    )
                        -> Result<(metamodelica::Ref<DAE::Exp>, VarTransform::VariableReplacements)>
                    + 'static,
            >),
        repl,
    )?;
    Ok(outElements)
}

pub(crate) fn replaceCrefBottomUp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut replIn: VarTransform::VariableReplacements,
) -> Result<(metamodelica::Ref<DAE::Exp>, VarTransform::VariableReplacements)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut replOut: VarTransform::VariableReplacements;
    replOut = replIn.clone();
    (outExp, _) = Expression::traverseExpBottomUp(inExp, &replaceCompRef, replIn)?;
    Ok((outExp, replOut))
}

fn replaceCompRef(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut replIn: VarTransform::VariableReplacements,
) -> Result<(metamodelica::Ref<DAE::Exp>, VarTransform::VariableReplacements)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut replOut: VarTransform::VariableReplacements;
    replOut = replIn.clone();
    (outExp, _) = VarTransform::replaceExp(inExp, &replIn, None)?;
    Ok((outExp, replOut))
}

pub fn connectorTypeStr(mut connectorType: &metamodelica::Ref<DAE::ConnectorType>) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (::match_deref::match_deref! { match connectorType {
        Deref @ DAE::ConnectorType::POTENTIAL { .. } => {
            literal!("")
        },
        Deref @ DAE::ConnectorType::FLOW { .. } => {
            literal!("flow")
        },
        Deref @ DAE::ConnectorType::STREAM { associatedFlow: None } => {
            literal!("stream()")
        },
        Deref @ DAE::ConnectorType::STREAM { associatedFlow: Some(cref) } => {
            let mut cref_str: ArcStr;
            cref_str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?;
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("stream(")); __mm_s.push_str(&*cref_str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!("non connector")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(string)
}

pub fn streamBool(mut inStream: &metamodelica::Ref<DAE::ConnectorType>) -> bool {
    let mut bStream: bool;
    bStream = (match &**inStream {
        DAE::ConnectorType::STREAM { .. } => true,
        _ => false,
    });
    bStream
}

pub fn potentialBool(mut inConnectorType: &metamodelica::Ref<DAE::ConnectorType>) -> bool {
    let mut outPotential: bool;
    outPotential = (match &**inConnectorType {
        DAE::ConnectorType::POTENTIAL { .. } => true,
        _ => false,
    });
    outPotential
}

pub fn connectorTypeEqual(
    mut inConnectorType1: &metamodelica::Ref<DAE::ConnectorType>,
    mut inConnectorType2: &metamodelica::Ref<DAE::ConnectorType>,
) -> Result<bool> {
    let mut outEqual: bool;
    outEqual = (::match_deref::match_deref! { match (inConnectorType1, inConnectorType2) {
        (Deref @ DAE::ConnectorType::POTENTIAL { .. }, Deref @ DAE::ConnectorType::POTENTIAL { .. }) => true,
        (Deref @ DAE::ConnectorType::FLOW { .. }, Deref @ DAE::ConnectorType::FLOW { .. }) => true,
        (Deref @ DAE::ConnectorType::STREAM { associatedFlow: _ }, Deref @ DAE::ConnectorType::STREAM { associatedFlow: _ }) => true,
        (Deref @ DAE::ConnectorType::NON_CONNECTOR { .. }, Deref @ DAE::ConnectorType::NON_CONNECTOR { .. }) => true,
        _ => return Err("match: no arm matched"),
    } });
    Ok(outEqual)
}

pub fn toSCodeConnectorType(mut daeConnectorType: &metamodelica::Ref<DAE::ConnectorType>) -> SCode::ConnectorType {
    let mut scodeConnectorType: SCode::ConnectorType;
    scodeConnectorType = (match &**daeConnectorType {
        DAE::ConnectorType::FLOW { .. } => openmodelica_frontend_types::SCode::ConnectorType::FLOW,
        DAE::ConnectorType::STREAM { associatedFlow: _ } => openmodelica_frontend_types::SCode::ConnectorType::STREAM,
        DAE::ConnectorType::POTENTIAL { .. } => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
        DAE::ConnectorType::NON_CONNECTOR { .. } => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
    });
    scodeConnectorType
}

pub fn mergeAlgorithmSections(mut inDae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut newEls: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut dAElist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut istmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut s: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut source: metamodelica::Ref<DAE::ElementSource> = DAE::emptyElementSource().clone();
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut ident: ArcStr;
    let mut comment: Option<metamodelica::Ref<SCode::Comment>>;
    if !(Flags::isSet(Flags::MERGE_ALGORITHM_SECTIONS.clone())?) {
        outDae = inDae;
        return Ok(outDae);
    }
    let DAE::DAE { elementLst: __pa0 } = inDae;
    els = metamodelica::Own::own(__pa0);
    for mut e in &*els {
        let () = (::match_deref::match_deref! { match &(e.clone()) {
            Deref @ DAE::Element::COMP { ident: __esc_ident, dAElist: __esc_dAElist, source: __esc_src, comment: __esc_comment } => {
                ident = (*__esc_ident).clone();
                dAElist = (*__esc_dAElist).clone();
                src = (*__esc_src).clone();
                comment = (*__esc_comment).clone();
                let DAE::DAE { elementLst: __pa0 } = mergeAlgorithmSections(DAE::DAElist { elementLst: dAElist.clone() })?;
                dAElist = metamodelica::Own::own(__pa0);
                newEls = metamodelica::cons(metamodelica::Ref::new(DAE::Element::COMP { ident: ident.clone(), dAElist: dAElist.clone(), source: src.clone(), comment: comment.clone() }), newEls);
                ()
            },
            Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: __esc_s }, source: __esc_source } => {
                s = (*__esc_s).clone();
                source = (*__esc_source).clone();
                stmts = List::append_reverse(metamodelica::AsArg::as_arg(&s), stmts);
                ()
            },
            Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: __esc_s }, source: __esc_source } => {
                s = (*__esc_s).clone();
                source = (*__esc_source).clone();
                istmts = List::append_reverse(metamodelica::AsArg::as_arg(&s), istmts);
                ()
            },
            _ => {
                newEls = metamodelica::cons(e.clone(), newEls);
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    if !((istmts).is_empty()) {
        newEls = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::INITIALALGORITHM {
                algorithm_: metamodelica::Ref::new(DAE::Algorithm {
                    statementLst: istmts.reverse(),
                }),
                source: source.clone(),
            }),
            newEls,
        );
    }
    if !((stmts).is_empty()) {
        newEls = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::ALGORITHM {
                algorithm_: metamodelica::Ref::new(DAE::Algorithm {
                    statementLst: stmts.reverse(),
                }),
                source: source,
            }),
            newEls,
        );
    }
    newEls = newEls.reverse();
    outDae = DAE::DAElist { elementLst: newEls };
    Ok(outDae)
}

pub fn moveElementToInitialSection(mut elt: metamodelica::Ref<DAE::Element>) -> metamodelica::Ref<DAE::Element> {
    let mut elt: metamodelica::Ref<DAE::Element> = elt;
    elt = (match &*elt {
        DAE::Element::EQUATION {
            exp: __elt_exp,
            scalar: __elt_scalar,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIALEQUATION {
            exp1: __elt_exp.clone(),
            exp2: __elt_scalar.clone(),
            source: __elt_source.clone(),
        }),
        DAE::Element::DEFINE {
            componentRef: __elt_componentRef,
            exp: __elt_exp,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIALDEFINE {
            componentRef: __elt_componentRef.clone(),
            exp: __elt_exp.clone(),
            source: __elt_source.clone(),
        }),
        DAE::Element::ARRAY_EQUATION {
            array: __elt_array,
            dimension: __elt_dimension,
            exp: __elt_exp,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION {
            dimension: __elt_dimension.clone(),
            exp: __elt_exp.clone(),
            array: __elt_array.clone(),
            source: __elt_source.clone(),
        }),
        DAE::Element::COMPLEX_EQUATION {
            lhs: __elt_lhs,
            rhs: __elt_rhs,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIAL_COMPLEX_EQUATION {
            lhs: __elt_lhs.clone(),
            rhs: __elt_rhs.clone(),
            source: __elt_source.clone(),
        }),
        DAE::Element::IF_EQUATION {
            condition1: __elt_condition1,
            equations2: __elt_equations2,
            equations3: __elt_equations3,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIAL_IF_EQUATION {
            condition1: __elt_condition1.clone(),
            equations2: __elt_equations2.clone(),
            equations3: __elt_equations3.clone(),
            source: __elt_source.clone(),
        }),
        DAE::Element::ALGORITHM {
            algorithm_: __elt_algorithm_,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIALALGORITHM {
            algorithm_: __elt_algorithm_.clone(),
            source: __elt_source.clone(),
        }),
        DAE::Element::ASSERT {
            condition: __elt_condition,
            level: __elt_level,
            message: __elt_message,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIAL_ASSERT {
            condition: __elt_condition.clone(),
            message: __elt_message.clone(),
            level: __elt_level.clone(),
            source: __elt_source.clone(),
        }),
        DAE::Element::TERMINATE {
            message: __elt_message,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIAL_TERMINATE {
            message: __elt_message.clone(),
            source: __elt_source.clone(),
        }),
        DAE::Element::NORETCALL {
            exp: __elt_exp,
            source: __elt_source,
        } => metamodelica::Ref::new(DAE::Element::INITIAL_NORETCALL {
            exp: __elt_exp.clone(),
            source: __elt_source.clone(),
        }),
        _ => elt,
    });
    elt
}

pub fn getParameters(
    mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(elts) {
            Deref @ metamodelica::ListNode::Nil => {
                return acc
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { dAElist: celts, .. }, tail: rest } => {
                let mut a: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                a = getParameters(celts.clone(), acc);
                { (elts, acc) = (rest.clone(), a); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Element::VAR { .. }, tail: rest } => {
                if (isParameterOrConstant(metamodelica::AsArg::as_arg(&e))) {return metamodelica::cons(e.clone(), getParameters(rest.clone(), acc))} else {{ (elts, acc) = (rest.clone(), acc); continue '__tco; }}
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (elts, acc) = (rest.clone(), acc); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getInteger(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<i32> {
    let mut i: i32;
    i = (match &*exp {
        DAE::Exp::ICONST { integer: __esc_i } => {
            i = (*__esc_i).clone();
            i.clone()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("DAEUtil.getInteger"));
                    __mm_s.push_str(&*literal!(" failed because expression is not an ICONST: "));
                    __mm_s.push_str(&*ExpressionBasics::printExpStr(exp)?);
                    __mm_s.push_str(&*literal!(".\n"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(i)
}

pub fn optimizeMetaRecordFieldAssigns(
    mut inStmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = tempVars;
    let mut s2: metamodelica::Ref<DAE::Statement>;
    outStmts = metamodelica::nil();
    for mut s in &**inStmts {
        (s2, tempVars) = optMRFAInStmt(s.clone(), tempVars)?;
        outStmts = metamodelica::cons(s2, outStmts);
    }
    outStmts = outStmts.reverse();
    (outStmts, tempVars) = optMRFAMergeList(outStmts, tempVars)?;
    Ok((outStmts, tempVars))
}

fn optMRFAInStmt(
    mut stmt: metamodelica::Ref<DAE::Statement>,
    mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::Ref<DAE::Statement>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut stmt: metamodelica::Ref<DAE::Statement> = stmt;
    let mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = tempVars;
    (stmt, tempVars) = (match &*stmt {
        DAE::Statement::STMT_IF {
            statementLst: __stmt_statementLst,
            ..
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut els: metamodelica::Ref<DAE::Else>;
            (stmts, tempVars) =
                optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&__stmt_statementLst), tempVars)?;
            assign_variant_field!(stmt => DAE::Statement::STMT_IF; statementLst = stmts);
            (els, tempVars) = optMRFAInElse(var_field!((*stmt).else_, DAE::Statement::STMT_IF).clone(), tempVars)?;
            assign_variant_field!(stmt => DAE::Statement::STMT_IF; else_ = els);
            (stmt, tempVars)
        }
        DAE::Statement::STMT_FOR {
            statementLst: __stmt_statementLst,
            ..
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (stmts, tempVars) =
                optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&__stmt_statementLst), tempVars)?;
            assign_variant_field!(stmt => DAE::Statement::STMT_FOR; statementLst = stmts);
            (stmt, tempVars)
        }
        DAE::Statement::STMT_PARFOR {
            statementLst: __stmt_statementLst,
            ..
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (stmts, tempVars) =
                optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&__stmt_statementLst), tempVars)?;
            assign_variant_field!(stmt => DAE::Statement::STMT_PARFOR; statementLst = stmts);
            (stmt, tempVars)
        }
        DAE::Statement::STMT_WHILE {
            statementLst: __stmt_statementLst,
            ..
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (stmts, tempVars) =
                optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&__stmt_statementLst), tempVars)?;
            assign_variant_field!(stmt => DAE::Statement::STMT_WHILE; statementLst = stmts);
            (stmt, tempVars)
        }
        DAE::Statement::STMT_WHEN {
            statementLst: __stmt_statementLst,
            ..
        } => {
            let mut ew: metamodelica::Ref<DAE::Statement>;
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (stmts, tempVars) =
                optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&__stmt_statementLst), tempVars)?;
            assign_variant_field!(stmt => DAE::Statement::STMT_WHEN; statementLst = stmts);
            assign_variant_field!(stmt => DAE::Statement::STMT_WHEN; elseWhen = (::match_deref::match_deref! { match &(var_field!((*stmt).elseWhen, DAE::Statement::STMT_WHEN).clone()) {
        Some(__esc_ew) => {
            ew = (*__esc_ew).clone();
            (ew, tempVars) = optMRFAInStmt(ew.clone(), tempVars)?;
            Some(ew.clone())
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }));
            (stmt, tempVars)
        }
        DAE::Statement::STMT_FAILURE { body: __stmt_body, .. } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (stmts, tempVars) = optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&__stmt_body), tempVars)?;
            assign_variant_field!(stmt => DAE::Statement::STMT_FAILURE; body = stmts);
            (stmt, tempVars)
        }
        _ => (stmt, tempVars),
    });
    Ok((stmt, tempVars))
}

fn optMRFAInElse(
    mut els: metamodelica::Ref<DAE::Else>,
    mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::Ref<DAE::Else>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut els: metamodelica::Ref<DAE::Else> = els;
    let mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = tempVars;
    (els, tempVars) = (match &*els {
        DAE::Else::ELSEIF {
            statementLst: __els_statementLst,
            ..
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut nestedEls: metamodelica::Ref<DAE::Else>;
            (stmts, tempVars) =
                optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&__els_statementLst), tempVars)?;
            assign_variant_field!(els => DAE::Else::ELSEIF; statementLst = stmts);
            (nestedEls, tempVars) = optMRFAInElse(var_field!((*els).else_, DAE::Else::ELSEIF).clone(), tempVars)?;
            assign_variant_field!(els => DAE::Else::ELSEIF; else_ = nestedEls);
            (els, tempVars)
        }
        DAE::Else::ELSE {
            statementLst: __els_statementLst,
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (stmts, tempVars) =
                optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&__els_statementLst), tempVars)?;
            assign_variant_field!(els => DAE::Else::ELSE; statementLst = stmts);
            (els, tempVars)
        }
        _ => (els, tempVars),
    });
    Ok((els, tempVars))
}

// (field name, rhs expression, originating statement -- kept so we can reuse
// its ElementSource when the group is merged into a single statement).
pub type MRFAUpdate = (ArcStr, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Statement>);

fn optMRFAMergeList(
    mut inStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = tempVars;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Statement>> = inStmts;
    let mut stmt: metamodelica::Ref<DAE::Statement>;
    let mut m: Option<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<DAE::Type>,
        ArcStr,
        metamodelica::Ref<DAE::Exp>,
    )>;
    let mut base: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut mrecTy: metamodelica::Ref<DAE::Type>;
    let mut field: ArcStr;
    let mut group: metamodelica::List<(ArcStr, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Statement>)>;
    let mut writtenFields: metamodelica::List<ArcStr>;
    let mut extended: bool;
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        stmt = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        m = optMRFAMatch(&stmt);
        (outStmts, rest, tempVars) = (::match_deref::match_deref! { match &(m) {
            Some((__esc_base, __esc_mrecTy, __esc_field, __esc_rhs)) => {
                base = (*__esc_base).clone();
                mrecTy = (*__esc_mrecTy).clone();
                field = (*__esc_field).clone();
                rhs = (*__esc_rhs).clone();
                group = list![(field.clone(), rhs.clone(), stmt)];
                writtenFields = list![field.clone()];
                extended = true;
                while extended {
                    (writtenFields, group, extended) = optMRFATryExtend(&rest, base.clone(), metamodelica::AsArg::as_arg(&mrecTy), writtenFields, group)?;
                    if extended {
                        rest = (rest).rest()?;
                    }
                }
                (outStmts, tempVars) = optMRFACommitGroup(&(group.reverse()), base.clone(), mrecTy.clone(), outStmts, tempVars)?;
                (outStmts, rest, tempVars)
            },
            _ => (metamodelica::cons(stmt, outStmts), rest, tempVars),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outStmts = outStmts.reverse();
    Ok((outStmts, tempVars))
}

fn optMRFACommitGroup(
    mut group: &metamodelica::List<(ArcStr, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Statement>)>,
    mut baseExp: metamodelica::Ref<DAE::Exp>,
    mut mrecTy: metamodelica::Ref<DAE::Type>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outAcc: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = tempVars;
    let mut original: metamodelica::Ref<DAE::Statement>;
    let mut mergedStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    (outAcc, tempVars) = (::match_deref::match_deref! { match group {
        Deref @ metamodelica::ListNode::Cons { head: (_, _, __esc_original), tail: Deref @ metamodelica::ListNode::Nil } => {
            original = (*__esc_original).clone();
            (metamodelica::cons(original.clone(), acc), tempVars)
        },
        _ => {
            (mergedStmts, tempVars) = optMRFABuildMerged(group, baseExp, mrecTy, tempVars)?;
            (List::append_reverse(&mergedStmts, acc), tempVars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outAcc, tempVars))
}

fn optMRFABuildMerged(
    mut group: &metamodelica::List<(ArcStr, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Statement>)>,
    mut baseExp: metamodelica::Ref<DAE::Exp>,
    mut mrecTy: metamodelica::Ref<DAE::Type>,
    mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = tempVars;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut index: i32;
    let mut typeVars: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut fields: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut fieldNames: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut fv: metamodelica::Ref<DAE::Var> = <metamodelica::Ref<DAE::Var> as ::std::default::Default>::default();
    let mut fname: ArcStr;
    let mut gname: ArcStr;
    let mut tname: ArcStr;
    let mut fty: metamodelica::Ref<DAE::Type>;
    let mut gty: metamodelica::Ref<DAE::Type>;
    let mut grhs: metamodelica::Ref<DAE::Exp>;
    let mut pos: i32 = 1;
    let mut arg: metamodelica::Ref<DAE::Exp>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut baseTy: metamodelica::Ref<DAE::Type>;
    let mut tempAssigns: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut fieldRefs: metamodelica::List<(ArcStr, metamodelica::Ref<DAE::Exp>)> = metamodelica::nil();
    let mut tcref: metamodelica::Ref<DAE::ComponentRef>;
    let mut tref: metamodelica::Ref<DAE::Exp>;
    let mut tvar: metamodelica::Ref<DAE::Element>;
    (path, index, typeVars) = optMRFAMetaRecordInfo(&mrecTy)?;
    fields = Types::getMetaRecordFields(mrecTy)?;
    let __pa0 = ::match_deref::match_deref! { match &(Util::tuple33((group).head().cloned()?)) {
        Deref @ DAE::Statement::STMT_ASSIGN { source: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    src = metamodelica::Own::own(__pa0);
    for mut upd in &**group {
        (gname, grhs, _) = upd.clone();
        gty = optMRFAFieldType(&fields, &gname)?;
        tname = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$mrfa_"));
            __mm_s.push_str(&*Util::tickStr());
            ArcStr::from(__mm_s)
        };
        tcref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
            ident: tname,
            identType: gty.clone(),
            subscriptLst: metamodelica::nil(),
        });
        tref = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: tcref.clone(),
            ty: gty.clone(),
        });
        tempAssigns = metamodelica::cons(
            metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
                type_: gty.clone(),
                exp1: tref.clone(),
                exp: grhs,
                source: src.clone(),
            }),
            tempAssigns,
        );
        tvar = metamodelica::Ref::new(DAE::Element::VAR {
            componentRef: tcref,
            kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE,
            direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
            parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
            protection: openmodelica_frontend_types::DAE::VarVisibility::PROTECTED,
            ty: gty,
            binding: None,
            dims: metamodelica::nil(),
            connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
            source: src.clone(),
            variableAttributesOption: None,
            comment: None,
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            encrypted: false,
        });
        tempVars = metamodelica::cons(tvar, tempVars);
        fieldRefs = metamodelica::cons((gname, tref), fieldRefs);
    }
    for mut fv in &*fields {
        let mut fv = fv.clone();
        let __arc3 = fv;
        let DAE::TYPES_VAR {
            name: __pa1, ty: __pa2, ..
        } = &*__arc3;
        fname = metamodelica::Own::own(__pa1);
        fty = metamodelica::Own::own(__pa2);
        arg = (::match_deref::match_deref! { match &(optMRFALookupRef(&fieldRefs, &fname)) {
            Some(__esc_arg) => {
                arg = (*__esc_arg).clone();
                arg.clone()
            },
            _ => metamodelica::Ref::new(DAE::Exp::RSUB { exp: baseExp.clone(), ix: pos, fieldName: fname.clone(), ty: fty }),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        args = metamodelica::cons(arg.clone(), args);
        fieldNames = metamodelica::cons(fname, fieldNames);
        pos = pos + 1;
    }
    args = args.reverse();
    fieldNames = fieldNames.reverse();
    baseTy = Expression::r#typeof(baseExp.clone())?;
    tempAssigns = metamodelica::cons(
        metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
            type_: baseTy,
            exp1: baseExp,
            exp: metamodelica::Ref::new(DAE::Exp::METARECORDCALL {
                path: path,
                args: args,
                fieldNames: fieldNames,
                index: index,
                typeVars: typeVars,
            }),
            source: src,
        }),
        tempAssigns,
    );
    outStmts = tempAssigns.reverse();
    Ok((outStmts, tempVars))
}

fn optMRFAFieldType(
    mut fields: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut fname: &ArcStr,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut n: ArcStr;
    for mut fv in &**fields {
        let __arc2 = fv.clone();
        let DAE::TYPES_VAR {
            name: __pa0, ty: __pa1, ..
        } = &*__arc2;
        n = metamodelica::Own::own(__pa0);
        ty = metamodelica::Own::own(__pa1);
        if stringEq(&n, &fname) {
            return Ok(ty);
        }
    }
    Error::addInternalError(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("optMRFAFieldType: field "));
            __mm_s.push_str(&*fname);
            __mm_s.push_str(&*literal!(" not found in metarecord"));
            ArcStr::from(__mm_s)
        },
        metamodelica::sourceInfo!("FrontEnd/DAEUtil.mo"),
    )?;
    return Err("fail");
    Ok(ty)
}

fn optMRFALookupRef(
    mut fieldRefs: &metamodelica::List<(ArcStr, metamodelica::Ref<DAE::Exp>)>,
    mut fname: &ArcStr,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut outRef: Option<metamodelica::Ref<DAE::Exp>>;
    let mut n: ArcStr;
    let mut r: metamodelica::Ref<DAE::Exp>;
    outRef = None;
    for mut fr in &**fieldRefs {
        (n, r) = fr.clone();
        if stringEq(&n, &fname) {
            outRef = Some(r);
            return outRef;
        }
    }
    outRef
}

fn optMRFAMetaRecordInfo(
    mut ty: &metamodelica::Ref<DAE::Type>,
) -> Result<(
    metamodelica::Ref<Absyn::Path>,
    i32,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut index: i32;
    let mut typeVars: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (path, index, typeVars) = (::match_deref::match_deref! { match ty {
        Deref @ DAE::Type::T_METARECORD { index: __ty_index, path: __ty_path, typeVars: __ty_typeVars, .. } => {
            (__ty_path.clone(), __ty_index.clone(), __ty_typeVars.clone())
        },
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: rec }, .. } => {
            optMRFAMetaRecordPieces(metamodelica::AsArg::as_arg(&rec))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((path, index, typeVars))
}

fn optMRFAMetaRecordPieces(
    mut ty: &metamodelica::Ref<DAE::Type>,
) -> Result<(
    metamodelica::Ref<Absyn::Path>,
    i32,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut index: i32;
    let mut typeVars: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ DAE::Type::T_METARECORD { path: __pa0, index: __pa1, typeVars: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa0);
    index = metamodelica::Own::own(__pa1);
    typeVars = metamodelica::Own::own(__pa2);
    Ok((path, index, typeVars))
}

fn optMRFAMatch(
    mut stmt: &metamodelica::Ref<DAE::Statement>,
) -> Option<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    ArcStr,
    metamodelica::Ref<DAE::Exp>,
)> {
    let mut outMatch: Option<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<DAE::Type>,
        ArcStr,
        metamodelica::Ref<DAE::Exp>,
    )>;
    outMatch = (::match_deref::match_deref! { match stmt {
        Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::RSUB { exp: baseExp @ Deref @ DAE::Exp::CREF { ty: t1 @ Deref @ DAE::Type::T_METARECORD { .. }, .. }, fieldName: fname, .. }, exp: rhs, .. } => {
            Some((baseExp.clone(), t1.clone(), fname.clone(), rhs.clone()))
        },
        Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::RSUB { exp: baseExp @ Deref @ DAE::Exp::CREF { ty: t1 @ Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: true, .. }, .. }, fieldName: fname, .. }, exp: rhs, .. } if (optMRFAResolvableSingleton(metamodelica::AsArg::as_arg(&t1))) => {
            Some((baseExp.clone(), t1.clone(), fname.clone(), rhs.clone()))
        },
        Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: identTy @ Deref @ DAE::Type::T_METATYPE { ty: t1 @ Deref @ DAE::Type::T_METARECORD { .. } }, subscriptLst: subs, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: fname, .. } }, .. }, exp: rhs, .. } => {
            let mut baseExp: metamodelica::Ref<DAE::Exp>;
            let mut topCref: metamodelica::Ref<DAE::ComponentRef>;
            topCref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident.clone(), identType: identTy.clone(), subscriptLst: subs.clone() });
            baseExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: topCref, ty: identTy.clone() });
            Some((baseExp, t1.clone(), fname.clone(), rhs.clone()))
        },
        Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: identTy @ Deref @ DAE::Type::T_METATYPE { ty: t1 @ Deref @ DAE::Type::T_METAUNIONTYPE { knownSingleton: true, .. } }, subscriptLst: subs, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: fname, .. } }, .. }, exp: rhs, .. } if (optMRFAResolvableSingleton(metamodelica::AsArg::as_arg(&t1))) => {
            let mut baseExp: metamodelica::Ref<DAE::Exp>;
            let mut topCref: metamodelica::Ref<DAE::ComponentRef>;
            topCref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident.clone(), identType: identTy.clone(), subscriptLst: subs.clone() });
            baseExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: topCref, ty: identTy.clone() });
            Some((baseExp, t1.clone(), fname.clone(), rhs.clone()))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMatch
}

fn optMRFAResolvableSingleton(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut ok: bool;
    ok = (::match_deref::match_deref! { match ty {
        Deref @ DAE::Type::T_METAUNIONTYPE { singletonType: Deref @ DAE::EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: Deref @ DAE::Type::T_METARECORD { .. } }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ok
}

fn optMRFATryExtend(
    mut rest: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut baseExp: metamodelica::Ref<DAE::Exp>,
    mut mrecTy: &metamodelica::Ref<DAE::Type>,
    mut writtenFields: metamodelica::List<ArcStr>,
    mut group: metamodelica::List<(ArcStr, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Statement>)>,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<(ArcStr, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Statement>)>,
    bool,
)> {
    let mut writtenFields: metamodelica::List<ArcStr> = writtenFields;
    let mut group: metamodelica::List<(ArcStr, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Statement>)> = group;
    let mut extended: bool;
    let mut stmt: metamodelica::Ref<DAE::Statement>;
    let mut m: Option<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<DAE::Type>,
        ArcStr,
        metamodelica::Ref<DAE::Exp>,
    )>;
    let mut base2: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut t2: metamodelica::Ref<DAE::Type>;
    let mut f: ArcStr;
    if (rest).is_empty() {
        extended = false;
        return Ok((writtenFields, group, extended));
    }
    stmt = (rest).head().cloned()?;
    m = optMRFAMatch(&stmt);
    extended = (::match_deref::match_deref! { match &(&m) {
        Some((__esc_base2, __esc_t2, __esc_f, __esc_rhs)) => {
            base2 = (*__esc_base2).clone();
            t2 = (*__esc_t2).clone();
            f = (*__esc_f).clone();
            rhs = (*__esc_rhs).clone();
            optMRFACheckExtend(metamodelica::AsArg::as_arg(&base2), metamodelica::AsArg::as_arg(&t2), f.clone(), rhs.clone(), baseExp, mrecTy, writtenFields.clone())?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if extended {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(m) {
            Some((_, _, __pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        f = metamodelica::Own::own(__pa0);
        rhs = metamodelica::Own::own(__pa1);
        group = metamodelica::cons((f.clone(), rhs, stmt), group);
        writtenFields = metamodelica::cons(f, writtenFields);
    }
    Ok((writtenFields, group, extended))
}

fn optMRFACheckExtend(
    mut newBase: &metamodelica::Ref<DAE::Exp>,
    mut newMrecTy: &metamodelica::Ref<DAE::Type>,
    mut newField: ArcStr,
    mut newRhs: metamodelica::Ref<DAE::Exp>,
    mut baseExp: metamodelica::Ref<DAE::Exp>,
    mut mrecTy: &metamodelica::Ref<DAE::Type>,
    mut writtenFields: metamodelica::List<ArcStr>,
) -> Result<bool> {
    let mut ok: bool;
    ok = ExpressionBasics::expEqual(newBase, baseExp.clone())?
        && !(listMember(newField, writtenFields.clone()))
        && optMRFARhsSafe(newRhs, baseExp, writtenFields)?;
    Ok(ok)
}

fn optMRFARhsSafe(
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut baseExp: metamodelica::Ref<DAE::Exp>,
    mut writtenFields: metamodelica::List<ArcStr>,
) -> Result<bool> {
    let mut safe: bool;
    let (_, (_, _, __pa0)) = Expression::traverseExpTopDown(rhs, &optMRFARhsCheck, (baseExp, writtenFields, true))?;
    safe = metamodelica::Own::own(__pa0);
    Ok(safe)
}

fn optMRFARhsCheck(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inAcc: (metamodelica::Ref<DAE::Exp>, metamodelica::List<ArcStr>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::Exp>, metamodelica::List<ArcStr>, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut cont: bool;
    let mut outAcc: (metamodelica::Ref<DAE::Exp>, metamodelica::List<ArcStr>, bool);
    let mut baseExp: metamodelica::Ref<DAE::Exp>;
    let mut innerExp: metamodelica::Ref<DAE::Exp>;
    let mut writtenFields: metamodelica::List<ArcStr>;
    let mut safe: bool;
    let mut handled: bool;
    let mut fname: ArcStr;
    (baseExp, writtenFields, safe) = inAcc.clone();
    if !(safe) {
        cont = false;
        outAcc = inAcc;
        return Ok((outExp, cont, outAcc));
    }
    handled = false;
    (safe, handled) = (match &*inExp {
        DAE::Exp::RSUB {
            exp: innerExp,
            fieldName: __esc_fname,
            ..
        } if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&innerExp), baseExp.clone())?) => {
            fname = (*__esc_fname).clone();
            (!(listMember(fname.clone(), writtenFields.clone())), true)
        }
        DAE::Exp::CREF { .. } => (optMRFACheckCrefRead(&inExp, &baseExp, writtenFields.clone())?, false),
        _ => (safe, false),
    });
    cont = safe && !(handled);
    outAcc = (baseExp, writtenFields, safe);
    Ok((outExp, cont, outAcc))
}

fn optMRFACheckCrefRead(
    mut crefExp: &metamodelica::Ref<DAE::Exp>,
    mut baseExp: &metamodelica::Ref<DAE::Exp>,
    mut writtenFields: metamodelica::List<ArcStr>,
) -> Result<bool> {
    let mut safe: bool;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut baseCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut baseIdent: ArcStr;
    let mut headIdent: ArcStr;
    let mut fname: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*crefExp)) {
        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cref = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &((*baseExp)) {
        Deref @ DAE::Exp::CREF { componentRef: __pa1, .. } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    baseCref = metamodelica::Own::own(__pa1);
    safe = (::match_deref::match_deref! { match &((cref, baseCref)) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_headIdent, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_baseIdent, .. }) => {
            headIdent = (*__esc_headIdent).clone();
            baseIdent = (*__esc_baseIdent).clone();
            !(stringEq(&headIdent, &baseIdent) && !((writtenFields).is_empty()))
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_headIdent, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_fname, .. }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_baseIdent, .. }) => {
            headIdent = (*__esc_headIdent).clone();
            fname = (*__esc_fname).clone();
            baseIdent = (*__esc_baseIdent).clone();
            !(stringEq(&headIdent, &baseIdent) && listMember(fname.clone(), writtenFields))
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_headIdent, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_baseIdent, .. }) => {
            headIdent = (*__esc_headIdent).clone();
            baseIdent = (*__esc_baseIdent).clone();
            !(stringEq(&headIdent, &baseIdent) && !((writtenFields).is_empty()))
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(safe)
}
