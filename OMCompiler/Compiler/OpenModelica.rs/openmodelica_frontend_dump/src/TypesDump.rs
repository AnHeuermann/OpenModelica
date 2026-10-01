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

use crate::AbsynUtil;
use crate::ClassInfUtil;
use crate::Dump;
use crate::ExpressionBasics;
use crate::SCodeDump;
use crate::ValuesDump;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Print;
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

pub fn unparseEqMod(mut eq: &DAE::EqMod) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match eq.clone() {
        DAE::EqMod::TYPED {
            modifierAsExp: ref e, ..
        } => {
            r#str = ExpressionBasics::printExpStr(e.clone())?;
            r#str
        }
        DAE::EqMod::UNTYPED { exp: ref e2 } => {
            r#str = Dump::printExpStr(e2.clone())?;
            r#str
        }
    });
    Ok(r#str)
}

pub fn unparseOptionEqMod(mut eq: Option<DAE::EqMod>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match eq {
        None => {
            literal!("NONE()")
        }
        Some(mut e) => unparseEqMod(&e)?,
    });
    Ok(r#str)
}

pub fn unparseType(mut inType: metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inType) {
            Deref @ DAE::Type::T_INTEGER { varLst: Deref @ metamodelica::ListNode::Nil } => {
                return Ok(literal!("Integer"))
            },
            Deref @ DAE::Type::T_REAL { varLst: Deref @ metamodelica::ListNode::Nil } => {
                return Ok(literal!("Real"))
            },
            Deref @ DAE::Type::T_STRING { varLst: Deref @ metamodelica::ListNode::Nil } => {
                return Ok(literal!("String"))
            },
            Deref @ DAE::Type::T_BOOL { varLst: Deref @ metamodelica::ListNode::Nil } => {
                return Ok(literal!("Boolean"))
            },
            Deref @ DAE::Type::T_CLOCK { .. } => {
                return Ok(literal!("Clock"))
            },
            Deref @ DAE::Type::T_INTEGER { varLst: vs } => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                s1 = stringDelimitList(List::map(vs.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(unparseVarAttr(&__a0)) })?, literal!(", "));
                return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Integer(")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
            },
            Deref @ DAE::Type::T_REAL { varLst: vs } => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                s1 = stringDelimitList(List::map(vs.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(unparseVarAttr(&__a0)) })?, literal!(", "));
                return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Real(")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
            },
            Deref @ DAE::Type::T_STRING { varLst: vs } => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                s1 = stringDelimitList(List::map(vs.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(unparseVarAttr(&__a0)) })?, literal!(", "));
                return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("String(")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
            },
            Deref @ DAE::Type::T_BOOL { varLst: vs } => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                s1 = stringDelimitList(List::map(vs.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(unparseVarAttr(&__a0)) })?, literal!(", "));
                return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Boolean(")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
            },
            Deref @ DAE::Type::T_ENUMERATION { path, names: l, .. } => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                let mut r#str: ArcStr;
                s1 = if (Config::typeinfo()?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" /*")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("*/ (")); ArcStr::from(__mm_s) }} else {literal!("(")};
                s2 = stringDelimitList(l.clone(), literal!(", "));
                return Ok(stringAppendList(list![literal!("enumeration"), s1, s2, literal!(")")]))
            },
            ty @ Deref @ DAE::Type::T_ARRAY { .. } => {
                let mut dims: ArcStr;
                let mut res: ArcStr;
                let mut tystr: ArcStr;
                let mut dimlst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                let mut ty = (*ty).clone();
                (ty, dimlst) = flattenArrayType(metamodelica::AsArg::as_arg(&ty));
                tystr = unparseType(ty.clone())?;
                dims = printDimensionsStr(dimlst)?;
                return Ok(stringAppendList(list![tystr, literal!("["), dims, literal!("]")]))
            },
            Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: vs, .. } => {
                let mut res: ArcStr;
                let mut vstr: ArcStr;
                let mut name: ArcStr;
                let mut vars: metamodelica::List<ArcStr>;
                name = AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?;
                vars = List::map(vs.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| unparseVar(&__a0))?;
                vstr = stringAppendList(vars);
                return Ok(stringAppendList(list![literal!("record "), name.clone(), literal!("\n"), vstr, literal!("end "), name, literal!(";")]))
            },
            Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path, isExpandable: b }, varLst: vs, .. } => {
                let mut r#str: ArcStr;
                let mut res: ArcStr;
                let mut vstr: ArcStr;
                let mut name: ArcStr;
                let mut vars: metamodelica::List<ArcStr>;
                name = AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?;
                vars = List::map(vs.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| unparseVar(&__a0))?;
                vstr = stringAppendList(vars);
                r#str = if (b.clone()) {literal!("expandable ")} else {literal!("")};
                return Ok(stringAppendList(list![r#str, literal!("connector "), name.clone(), literal!("\n"), vstr, literal!("end "), name, literal!(";")]))
            },
            Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ci_state, complexType: bc_tp, .. } => {
                let mut res: ArcStr;
                let mut st_str: ArcStr;
                let mut bc_tp_str: ArcStr;
                st_str = AbsynUtil::pathString(ClassInfUtil::getStateName(metamodelica::AsArg::as_arg(&ci_state)), literal!("."), true, false)?;
                res = ClassInfUtil::printStateStr(metamodelica::AsArg::as_arg(&ci_state));
                bc_tp_str = unparseType(bc_tp.clone())?;
                return Ok(stringAppendList(list![literal!("("), res, literal!(" "), st_str, literal!(" bc:"), bc_tp_str, literal!(")")]))
            },
            Deref @ DAE::Type::T_COMPLEX { complexClassType: ci_state, .. } => {
                let mut res: ArcStr;
                let mut st_str: ArcStr;
                st_str = AbsynUtil::pathString(ClassInfUtil::getStateName(metamodelica::AsArg::as_arg(&ci_state)), literal!("."), true, false)?;
                res = ClassInfUtil::printStateStr(metamodelica::AsArg::as_arg(&ci_state));
                return Ok(stringAppendList(list![res, literal!(" "), st_str]))
            },
            Deref @ DAE::Type::T_FUNCTION { funcArg: params, funcResultType: restype, path, .. } => {
                let mut res: ArcStr;
                let mut paramstr: ArcStr;
                let mut restypestr: ArcStr;
                let mut funcstr: ArcStr;
                let mut paramstrs: metamodelica::List<ArcStr>;
                funcstr = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                paramstrs = List::map(params.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| unparseParam(&__a0))?;
                paramstr = stringDelimitList(paramstrs, literal!(", "));
                restypestr = unparseType(restype.clone())?;
                return Ok(stringAppendList(list![funcstr, literal!("<function>("), paramstr, literal!(") => "), restypestr]))
            },
            Deref @ DAE::Type::T_TUPLE { types: tys, names: __inType_names } => {
                let mut res: ArcStr;
                let mut tystr: ArcStr;
                let mut tystrs: metamodelica::List<ArcStr>;
                tystrs = (::match_deref::match_deref! { match &(__inType_names.clone()) {
            Some(names) => {
                ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            let __thr_src0 = tys.clone();
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = names.clone();
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(t), Some(n)) => {
                        let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*unparseType(t.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*n); ArcStr::from(__mm_s) };
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => return Err("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        })
            },
            _ => {
                ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut t in (tys.clone()).into_iter().cloned() {
                let __x = unparseType(t.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } });
                tystr = stringDelimitList(tystrs, literal!(", "));
                return Ok(stringAppendList(list![literal!("("), tystr, literal!(")")]))
            },
            Deref @ DAE::Type::T_METATUPLE { types: tys } => {
                let mut res: ArcStr;
                let mut tystr: ArcStr;
                let mut tystrs: metamodelica::List<ArcStr>;
                tystrs = List::map(tys.clone(), &unparseType)?;
                tystr = stringDelimitList(tystrs, literal!(", "));
                return Ok(stringAppendList(list![literal!("tuple<"), tystr, literal!(">")]))
            },
            Deref @ DAE::Type::T_METALIST { ty } => {
                let mut res: ArcStr;
                let mut tystr: ArcStr;
                tystr = unparseType(ty.clone())?;
                return Ok(stringAppendList(list![literal!("list<"), tystr, literal!(">")]))
            },
            Deref @ DAE::Type::T_METAARRAY { ty } => {
                let mut res: ArcStr;
                let mut tystr: ArcStr;
                tystr = unparseType(ty.clone())?;
                return Ok(stringAppendList(list![literal!("array<"), tystr, literal!(">")]))
            },
            Deref @ DAE::Type::T_METAPOLYMORPHIC { name: tystr } => {
                let mut res: ArcStr;
                return Ok(stringAppendList(list![literal!("polymorphic<"), tystr.clone(), literal!(">")]))
            },
            Deref @ DAE::Type::T_METAUNIONTYPE { path: __inType_path, typeVars: __inType_typeVars, .. } => {
                let mut res: ArcStr;
                res = AbsynUtil::pathStringNoQual(__inType_path.clone(), literal!("."), false, false)?;
                if ((__inType_typeVars).is_empty()) {return Ok(res)} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*res); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut tv in (__inType_typeVars.clone()).into_iter().cloned() {
                let __x = unparseType(tv.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!(","))); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) })}
            },
            Deref @ DAE::Type::T_METARECORD { path: __inType_path, typeVars: __inType_typeVars, .. } => {
                let mut res: ArcStr;
                res = AbsynUtil::pathStringNoQual(__inType_path.clone(), literal!("."), false, false)?;
                if ((__inType_typeVars).is_empty()) {return Ok(res)} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*res); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut tv in (__inType_typeVars.clone()).into_iter().cloned() {
                let __x = unparseType(tv.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!(","))); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) })}
            },
            Deref @ DAE::Type::T_METABOXED { ty } => {
                let mut res: ArcStr;
                res = unparseType(ty.clone())?;
                return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#")); __mm_s.push_str(&*res); ArcStr::from(__mm_s) })
            },
            Deref @ DAE::Type::T_METAOPTION { ty: Deref @ DAE::Type::T_UNKNOWN { .. } } => {
                return Ok(literal!("Option<Any>"))
            },
            Deref @ DAE::Type::T_METAOPTION { ty } => {
                let mut res: ArcStr;
                let mut tystr: ArcStr;
                tystr = unparseType(ty.clone())?;
                return Ok(stringAppendList(list![literal!("Option<"), tystr, literal!(">")]))
            },
            Deref @ DAE::Type::T_METATYPE { ty } => {
                { inType = ty.clone(); continue '__tco; }
            },
            Deref @ DAE::Type::T_NORETCALL { .. } => {
                return Ok(literal!("#NORETCALL#"))
            },
            Deref @ DAE::Type::T_UNKNOWN { .. } => {
                return Ok(literal!("#T_UNKNOWN#"))
            },
            Deref @ DAE::Type::T_ANYTYPE { .. } => {
                return Ok(literal!("#ANYTYPE#"))
            },
            Deref @ DAE::Type::T_CODE { ty: codeType } => {
                return Ok(printCodeTypeStr(codeType.clone()))
            },
            Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: ty } => {
                return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#FUNCTION_REFERENCE_VAR#")); __mm_s.push_str(&*unparseType(ty.clone())?); ArcStr::from(__mm_s) })
            },
            Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: ty, .. } => {
                return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#FUNCTION_REFERENCE_FUNC#")); __mm_s.push_str(&*unparseType(ty.clone())?); ArcStr::from(__mm_s) })
            },
            _ => {
                return Ok(literal!("Internal error TypesDump.unparseType: not implemented yet\n"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn unparseTypeNoAttr(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    (ty, _) = stripTypeVars(inType);
    outString = unparseType(ty)?;
    Ok(outString)
}

pub fn unparsePropTypeNoAttr(mut inProps: &DAE::Properties) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inProps.clone() {
        DAE::Properties::PROP { type_: ref ty, .. } => unparseTypeNoAttr(metamodelica::AsArg::as_arg(&ty))?,
        DAE::Properties::PROP_TUPLE { type_: ref ty, .. } => unparseTypeNoAttr(metamodelica::AsArg::as_arg(&ty))?,
    });
    Ok(outString)
}

pub fn unparseConst(mut inConst: DAE::Const) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inConst {
        DAE::Const::C_CONST { .. } => literal!("constant"),
        DAE::Const::C_PARAM { .. } => literal!("parameter"),
        DAE::Const::C_VAR { .. } => literal!("continuous"),
        DAE::Const::C_UNKNOWN { .. } => literal!("unknown"),
    });
    outString
}

pub fn printConstStr(mut inConst: DAE::Const) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inConst {
        DAE::Const::C_CONST { .. } => literal!("C_CONST"),
        DAE::Const::C_PARAM { .. } => literal!("C_PARAM"),
        DAE::Const::C_VAR { .. } => literal!("C_VAR"),
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("TypesDump.printConstStr"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("FrontEnd/TypesDump.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(outString)
}

pub fn printTupleConstStr(mut inTupleConst: &metamodelica::Ref<DAE::TupleConst>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inTupleConst {
        DAE::TupleConst::SINGLE_CONST { r#const: c } => {
            let mut cstr: ArcStr;
            cstr = printConstStr(c.clone())?;
            cstr
        }
        DAE::TupleConst::TUPLE_CONST {
            tupleConstLst: constlist,
        } => {
            let mut res: ArcStr;
            let mut res_1: ArcStr;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = List::map(constlist.clone(), &move |__a0: metamodelica::Ref<DAE::TupleConst>| {
                printTupleConstStr(&__a0)
            })?;
            res = stringDelimitList(strlist, literal!(", "));
            res_1 = stringAppendList(list![literal!("("), res, literal!(")")]);
            res_1
        }
    });
    Ok(outString)
}

pub fn printTypeStr(mut inType: metamodelica::Ref<DAE::Type>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { varLst: vars } => {
                    Ok(List::toStringCustom(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(printVarStr(&__a0)) }, literal!("Integer"), literal!("("), literal!(", "), literal!(")"), false, 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { varLst: vars } => {
                    Ok(List::toStringCustom(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(printVarStr(&__a0)) }, literal!("Real"), literal!("("), literal!(", "), literal!(")"), false, 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { varLst: vars } => {
                    Ok(List::toStringCustom(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(printVarStr(&__a0)) }, literal!("String"), literal!("("), literal!(", "), literal!(")"), false, 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { varLst: vars } => {
                    Ok(List::toStringCustom(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(printVarStr(&__a0)) }, literal!("Boolean"), literal!("("), literal!(", "), literal!(")"), false, 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { varLst: vars } => {
                    Ok(List::toStringCustom(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(printVarStr(&__a0)) }, literal!("Clock"), literal!("("), literal!(", "), literal!(")"), false, 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { literalVarLst: vars, .. } => {
                    Ok(List::toStringCustom(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(printVarStr(&__a0)) }, literal!("Enumeration"), literal!("("), literal!(", "), literal!(")"), false, 0)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: st, complexType: t, varLst: vars, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut compType: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    compType = printTypeStr(t.clone());
                    s1 = ClassInfUtil::printStateStr(metamodelica::AsArg::as_arg(&st));
                    s2 = stringDelimitList(List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(printVarStr(&__a0)) })?, literal!(", "));
                    r#str = stringAppendList(list![literal!("composite("), s1.clone(), literal!("{"), s2.clone(), literal!("}, derived from "), compType.clone(), literal!(")")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: st, varLst: vars, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = ClassInfUtil::printStateStr(metamodelica::AsArg::as_arg(&st));
                    s2 = stringDelimitList(List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(printVarStr(&__a0)) })?, literal!(", "));
                    r#str = stringAppendList(list![literal!("composite("), s1.clone(), literal!("{"), s2.clone(), literal!("})")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims, ty: t } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = stringDelimitList(List::map(dims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| ExpressionBasics::dimensionString(&__a0))?, literal!(", "));
                    s2 = printTypeStr(t.clone());
                    r#str = stringAppendList(list![literal!("array("), s2.clone(), literal!(")["), s1.clone(), literal!("]")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { funcArg: params, funcResultType: restype, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = printParamsStr(metamodelica::AsArg::as_arg(&params))?;
                    s2 = printTypeStr(restype.clone());
                    r#str = stringAppendList(list![literal!("function("), s1.clone(), literal!(") => "), s2.clone()]);
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*inType).path, DAE::Type::T_FUNCTION).clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) };
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = stringDelimitList(List::map(tys.clone(), &fnptr!(printTypeStr, metamodelica::Ref<DAE::Type>))?, literal!(", "));
                    r#str = stringAppendList(list![literal!("("), s1.clone(), literal!(")")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATUPLE { types: tys } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = printTypeStr(metamodelica::Ref::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }));
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { ty } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = printTypeStr(ty.clone());
                    r#str = stringAppendList(list![literal!("list<"), s1.clone(), literal!(">")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAOPTION { ty } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = printTypeStr(ty.clone());
                    r#str = stringAppendList(list![literal!("Option<"), s1.clone(), literal!(">")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAARRAY { ty } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = printTypeStr(ty.clone());
                    r#str = stringAppendList(list![literal!("array<"), s1.clone(), literal!(">")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METABOXED { ty } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = printTypeStr(ty.clone());
                    r#str = stringAppendList(list![literal!("boxed<"), s1.clone(), literal!(">")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METAPOLYMORPHIC { name: s1 } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = stringAppendList(list![literal!("polymorphic<"), s1.clone(), literal!(">")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN { .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = literal!("T_UNKNOWN");
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ANYTYPE { anyClassType: None } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = literal!("ANYTYPE()");
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ANYTYPE { anyClassType: Some(st) } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = ClassInfUtil::printStateStr(metamodelica::AsArg::as_arg(&st));
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ANYTYPE(")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_NORETCALL { .. } => {
                    Ok(literal!("()"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATYPE { ty: t } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = printTypeStr(t.clone());
                    r#str = stringAppendList(list![literal!("METATYPE("), s1.clone(), literal!(")")]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                t @ Deref @ DAE::Type::T_METARECORD { .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = AbsynUtil::pathStringNoQual(var_field!((**t).path, DAE::Type::T_METARECORD).clone(), literal!("."), false, false)?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("#")); ArcStr::from(__mm_s) };
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                t @ Deref @ DAE::Type::T_METAUNIONTYPE { .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = AbsynUtil::pathStringNoQual(var_field!((**t).path, DAE::Type::T_METAUNIONTYPE).clone(), literal!("."), false, false)?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("#")); ArcStr::from(__mm_s) };
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_EXPRESSION { .. } } => {
                    Ok(literal!("$Code(Expression)"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_EXPRESSION_OR_MODIFICATION { .. } } => {
                    Ok(literal!("$Code(ExpressionOrModification)"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME { .. } } => {
                    Ok(literal!("$Code(TypeName)"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_VARIABLENAME { .. } } => {
                    Ok(literal!("$Code(VariableName)"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_VARIABLENAMES { .. } } => {
                    Ok(literal!("$Code(VariableName[:])"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = literal!("TypesDump.printTypeStr failed");
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    r#str
}

pub fn printConnectorTypeStr(mut it: metamodelica::Ref<DAE::Type>) -> Result<(ArcStr, ArcStr)> {
    let mut s: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    (s, s2) = 'mc: {
        let __mc_input = &*it;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: connectorName, isExpandable }, varLst: vars, .. } => {
                    let mut varNames: metamodelica::List<ArcStr>;
                    let mut isExpandableStr: ArcStr;
                    let mut s: ArcStr = s.clone();
                    let mut s2: ArcStr = s2.clone();
                    varNames = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getVarName(&__a0)) })?;
                    isExpandableStr = if (isExpandable.clone()) {literal!("/* expandable */ ")} else {literal!("")};
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*isExpandableStr); __mm_s.push_str(&*AbsynUtil::pathString(connectorName.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) };
                    s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(varNames.clone(), literal!(", "))); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
                    Ok(((s.clone(), s2.clone()), s.clone(), s2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            s = __wb0;
            s2 = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: ClassInf::State::CONNECTOR { path: connectorName, isExpandable }, varLst: vars, complexType: t, .. } => {
                    let mut varNames: metamodelica::List<ArcStr>;
                    let mut isExpandableStr: ArcStr;
                    let mut s: ArcStr = s.clone();
                    let mut s2: ArcStr = s2.clone();
                    varNames = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getVarName(&__a0)) })?;
                    isExpandableStr = if (isExpandable.clone()) {literal!("/* expandable */ ")} else {literal!("")};
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*isExpandableStr); __mm_s.push_str(&*AbsynUtil::pathString(connectorName.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) };
                    s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(varNames.clone(), literal!(", "))); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!(" subtype of: ")); __mm_s.push_str(&*printTypeStr(t.clone())); ArcStr::from(__mm_s) };
                    Ok(((s.clone(), s2.clone()), s.clone(), s2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            s = __wb0;
            s2 = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((literal!(""), unparseType(it.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((s, s2))
}

pub(crate) fn printParamsStr(mut inFuncArgLst: &metamodelica::List<metamodelica::Ref<DAE::FuncArg>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match inFuncArgLst {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { name: n, ty: t, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut s1: ArcStr;
            s1 = printTypeStr(t.clone());
            r#str = stringAppendList(list![n.clone(), literal!(" :: "), s1]);
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { name: n, ty: t, .. }, tail: params } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printTypeStr(t.clone());
            s2 = printParamsStr(params)?;
            r#str = stringAppendList(list![n.clone(), literal!(" :: "), s1, literal!(" * "), s2]);
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

pub fn unparseVarAttr(mut inVar: &metamodelica::Ref<DAE::Var>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inVar;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name: n, binding: Deref @ DAE::Binding::EQBOUND { exp: e, .. }, .. } => {
                    let mut res: ArcStr;
                    let mut bindStr: ArcStr;
                    bindStr = ExpressionBasics::printExpStr(e.clone())?;
                    res = stringAppendList(list![n.clone(), literal!(" = "), bindStr.clone()]);
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name: n, binding: Deref @ DAE::Binding::VALBOUND { valBound: value, .. }, .. } => {
                    let mut res: ArcStr;
                    let mut valStr: ArcStr;
                    valStr = ValuesDump::valString(metamodelica::AsArg::as_arg(&value))?;
                    res = stringAppendList(list![n.clone(), literal!(" = "), valStr.clone()]);
                    Ok(res.clone())
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

pub fn unparseVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inVar {
        Deref @ DAE::Var { name: n, ty: typ, attributes: Deref @ DAE::Attributes { connectorType: ct, .. }, .. } => {
            let mut t: ArcStr;
            let mut res: ArcStr;
            let mut s: ArcStr;
            s = connectorTypeStr(metamodelica::AsArg::as_arg(&ct));
            t = unparseType(typ.clone())?;
            res = stringAppendList(list![literal!("  "), s, t, literal!(" "), n.clone(), literal!(";\n")]);
            res
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub(crate) fn connectorTypeStr(mut ct: &metamodelica::Ref<DAE::ConnectorType>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match &**ct {
        DAE::ConnectorType::POTENTIAL { .. } => literal!(""),
        DAE::ConnectorType::FLOW { .. } => literal!("flow "),
        DAE::ConnectorType::STREAM { associatedFlow: _ } => literal!("stream "),
        _ => literal!(""),
    });
    r#str
}

fn unparseParam(mut inFuncArg: &metamodelica::Ref<DAE::FuncArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inFuncArg {
        Deref @ DAE::FuncArg { name: id, ty, r#const: c, par: p, defaultBinding: None } => {
            let mut tstr: ArcStr;
            let mut res: ArcStr;
            let mut cstr: ArcStr;
            let mut pstr: ArcStr;
            tstr = unparseType(ty.clone())?;
            cstr = constStrFriendly(c.clone())?;
            pstr = dumpVarParallelismStr(p.clone());
            res = stringAppendList(list![tstr, literal!(" "), cstr, pstr, id.clone()]);
            res
        },
        Deref @ DAE::FuncArg { name: id, ty, r#const: c, par: p, defaultBinding: Some(exp) } => {
            let mut tstr: ArcStr;
            let mut res: ArcStr;
            let mut cstr: ArcStr;
            let mut estr: ArcStr;
            let mut pstr: ArcStr;
            tstr = unparseType(ty.clone())?;
            cstr = constStrFriendly(c.clone())?;
            estr = ExpressionBasics::printExpStr(exp.clone())?;
            pstr = dumpVarParallelismStr(p.clone());
            res = stringAppendList(list![tstr, literal!(" "), cstr, pstr, id.clone(), literal!(" := "), estr]);
            res
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub fn printVarStr(mut inVar: &metamodelica::Ref<DAE::Var>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = 'mc: {
        let __mc_input = &**inVar;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name: n, attributes: Deref @ DAE::Attributes { variability: var, .. }, ty: typ, binding: bind, .. } => {
                    let mut vs: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    s1 = printTypeStr(typ.clone());
                    vs = SCodeDump::variabilityString(var.clone());
                    s2 = printBindingStr(metamodelica::AsArg::as_arg(&bind))?;
                    r#str = stringAppendList(list![s1.clone(), literal!(" "), n.clone(), literal!(" "), vs.clone(), literal!(" "), s2.clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name: n, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = stringAppendList(list![n.clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    r#str
}

pub fn printBindingStr(mut inBinding: &metamodelica::Ref<DAE::Binding>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inBinding {
        Deref @ DAE::Binding::UNBOUND { .. } => {
            literal!("UNBOUND")
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: None, constant_: __inBinding_constant_, exp: __inBinding_exp, source: __inBinding_source } => {
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut res: ArcStr;
            let mut str3: ArcStr;
            r#str = ExpressionBasics::printExpStr(__inBinding_exp.clone())?;
            str2 = printConstStr(__inBinding_constant_.clone())?;
            str3 = printBindingSourceStr(__inBinding_source.clone());
            res = stringAppendList(list![literal!("DAE.EQBOUND("), r#str, literal!(", NONE(), "), str2, literal!(", "), str3, literal!(")")]);
            res
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(v), constant_: __inBinding_constant_, exp: __inBinding_exp, source: __inBinding_source } => {
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut res: ArcStr;
            let mut v_str: ArcStr;
            let mut str3: ArcStr;
            r#str = ExpressionBasics::printExpStr(__inBinding_exp.clone())?;
            str2 = printConstStr(__inBinding_constant_.clone())?;
            v_str = ValuesDump::valString(metamodelica::AsArg::as_arg(&v))?;
            str3 = printBindingSourceStr(__inBinding_source.clone());
            res = stringAppendList(list![literal!("DAE.EQBOUND("), r#str, literal!(", SOME("), v_str, literal!("), "), str2, literal!(", "), str3, literal!(")")]);
            res
        },
        Deref @ DAE::Binding::VALBOUND { valBound: v, source: __inBinding_source } => {
            let mut res: ArcStr;
            let mut s: ArcStr;
            let mut str3: ArcStr;
            s = ValuesDump::unparseValues(&(list![v.clone()]))?;
            str3 = printBindingSourceStr(__inBinding_source.clone());
            res = stringAppendList(list![literal!("DAE.VALBOUND("), s, literal!(", "), str3, literal!(")")]);
            res
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TypesDump.printBindingStr")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/TypesDump.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub(crate) fn printFarg(mut inFuncArg: &metamodelica::Ref<DAE::FuncArg>) -> Result<()> {
    let () = (match &**inFuncArg {
        DAE::FuncArg { name: n, ty, .. } => {
            Print::printErrorBuf(printTypeStr(ty.clone()))?;
            Print::printErrorBuf(literal!(" "))?;
            Print::printErrorBuf(n.clone())?;
            ()
        }
    });
    Ok(())
}

pub fn printFargStr(mut inFuncArg: &metamodelica::Ref<DAE::FuncArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inFuncArg {
        DAE::FuncArg {
            name: n,
            ty,
            r#const: c,
            par: _,
            defaultBinding: _,
        } => {
            let mut s: ArcStr;
            let mut res: ArcStr;
            let mut cs: ArcStr;
            s = unparseType(ty.clone())?;
            cs = constStrFriendly(c.clone())?;
            res = stringAppendList(list![cs, s, literal!(" "), n.clone()]);
            res
        }
    });
    Ok(outString)
}

pub fn getTypeName(mut inType: metamodelica::Ref<DAE::Type>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(literal!("Integer"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(literal!("Real"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(literal!("String"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(literal!("Boolean"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { .. } => {
                    Ok(literal!("Clock"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: st, .. } => {
                    let mut n: ArcStr;
                    n = AbsynUtil::pathString(ClassInfUtil::getStateName(metamodelica::AsArg::as_arg(&st)), literal!("."), true, false)?;
                    Ok(n.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: st, .. } => {
                    let mut n: ArcStr;
                    n = AbsynUtil::pathString(ClassInfUtil::getStateName(metamodelica::AsArg::as_arg(&st)), literal!("."), true, false)?;
                    Ok(n.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                arrayty @ Deref @ DAE::Type::T_ARRAY { .. } => {
                    let mut dimstr: ArcStr;
                    let mut tystr: ArcStr;
                    let mut r#str: ArcStr;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    (ty, dims) = flattenArrayType(metamodelica::AsArg::as_arg(&arrayty));
                    dimstr = ExpressionBasics::dimensionsString(dims.clone())?;
                    tystr = getTypeName(ty.clone());
                    r#str = stringAppendList(list![tystr.clone(), literal!("["), dimstr.clone(), literal!("]")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METALIST { ty } => {
                    let mut n: ArcStr;
                    n = getTypeName(ty.clone());
                    Ok(n.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Not nameable type or no type"))
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

pub(crate) fn constStrFriendly(mut r#const: DAE::Const) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match r#const {
        DAE::Const::C_VAR { .. } => literal!(""),
        DAE::Const::C_PARAM { .. } => literal!("parameter "),
        DAE::Const::C_CONST { .. } => literal!("constant "),
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

pub(crate) fn dumpVarParallelismStr(mut inVarParallelism: DAE::VarParallelism) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVarParallelism {
        DAE::VarParallelism::NON_PARALLEL { .. } => literal!(""),
        DAE::VarParallelism::PARGLOBAL { .. } => literal!("parglobal "),
        DAE::VarParallelism::PARLOCAL { .. } => literal!("parlocal "),
    });
    outString
}

pub(crate) fn printBindingSourceStr(mut bindingSource: DAE::BindingSource) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match bindingSource {
        DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE { .. } => literal!("[DEFAULT VALUE]"),
        DAE::BindingSource::BINDING_FROM_START_VALUE { .. } => literal!("[START VALUE]"),
        DAE::BindingSource::BINDING_FROM_RECORD_SUBMODS { .. } => literal!("[RECORD SUBMODS]"),
        DAE::BindingSource::BINDING_FROM_DERIVED_RECORD_DECL { .. } => literal!("[DERIVED RECORD]"),
    });
    r#str
}

pub fn flattenArrayType<'__b>(
    mut inType: &'__b metamodelica::Ref<DAE::Type>,
) -> (
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) {
    '__tco: loop {
        ::match_deref::match_deref! { match inType {
            Deref @ DAE::Type::T_ARRAY { .. } => {
                let mut ty: Type;
                let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                (ty, dims) = flattenArrayType(var_field!((**inType).ty, DAE::Type::T_ARRAY));
                dims = listAppend(var_field!((**inType).dims, DAE::Type::T_ARRAY).clone(), dims);
                return (ty, dims)
            },
            Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => {
                return (inType.clone(), metamodelica::nil())
            },
            Deref @ DAE::Type::T_SUBTYPE_BASIC { .. } => {
                { inType = var_field!((**inType).complexType, DAE::Type::T_SUBTYPE_BASIC); continue '__tco; }
            },
            _ => {
                return (inType.clone(), metamodelica::nil())
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getVarName(mut v: &metamodelica::Ref<DAE::Var>) -> ArcStr {
    let mut name: ArcStr;
    name = (match &**v {
        DAE::Var { name: __esc_name, .. } => {
            name = (*__esc_name).clone();
            name.clone()
        }
    });
    name
}

pub fn stripTypeVars(
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> (
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
) {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    (outType, outVars) = (match &**inType {
        DAE::Type::T_INTEGER { varLst: vars } => (DAE::T_INTEGER_DEFAULT().clone(), vars.clone()),
        DAE::Type::T_REAL { varLst: vars } => (DAE::T_REAL_DEFAULT().clone(), vars.clone()),
        DAE::Type::T_STRING { varLst: vars } => (DAE::T_STRING_DEFAULT().clone(), vars.clone()),
        DAE::Type::T_BOOL { varLst: vars } => (DAE::T_BOOL_DEFAULT().clone(), vars.clone()),
        DAE::Type::T_TUPLE { types: tys, names: _ } => (
            metamodelica::Ref::new(DAE::Type::T_TUPLE {
                types: tys.clone(),
                names: None,
            }),
            metamodelica::nil(),
        ),
        DAE::Type::T_ARRAY { ty, dims } => {
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            let mut ty = (*ty).clone();
            (ty, vars) = stripTypeVars(metamodelica::AsArg::as_arg(&ty));
            (
                metamodelica::Ref::new(DAE::Type::T_ARRAY {
                    ty: ty.clone(),
                    dims: dims.clone(),
                }),
                vars,
            )
        }
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType: state,
            varLst: sub_vars,
            complexType: ty,
            equalityConstraint: ec,
        } => {
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            let mut ty = (*ty).clone();
            (ty, vars) = stripTypeVars(metamodelica::AsArg::as_arg(&ty));
            (
                metamodelica::Ref::new(DAE::Type::T_SUBTYPE_BASIC {
                    complexClassType: state.clone(),
                    varLst: sub_vars.clone(),
                    complexType: ty.clone(),
                    equalityConstraint: ec.clone(),
                }),
                vars,
            )
        }
        _ => (inType.clone(), metamodelica::nil()),
    });
    (outType, outVars)
}

pub fn printDimensionsStr(mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = stringDelimitList(
        List::map(dims, &move |__a0: metamodelica::Ref<DAE::Dimension>| {
            ExpressionBasics::dimensionString(&__a0)
        })?,
        literal!(", "),
    );
    Ok(res)
}

pub fn printCodeTypeStr(mut ct: DAE::CodeType) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match ct {
        DAE::CodeType::C_EXPRESSION { .. } => literal!("OpenModelica.Code.Expression"),
        DAE::CodeType::C_EXPRESSION_OR_MODIFICATION { .. } => literal!("OpenModelica.Code.ExpressionOrModification"),
        DAE::CodeType::C_MODIFICATION { .. } => literal!("OpenModelica.Code.Modification"),
        DAE::CodeType::C_TYPENAME { .. } => literal!("OpenModelica.Code.TypeName"),
        DAE::CodeType::C_VARIABLENAME { .. } => literal!("OpenModelica.Code.VariableName"),
        DAE::CodeType::C_VARIABLENAMES { .. } => literal!("OpenModelica.Code.VariableNames"),
        _ => literal!("TypesDump.printCodeTypeStr failed"),
    });
    r#str
}

pub fn getDimensions<'__b>(
    mut inType: &'__b metamodelica::Ref<DAE::Type>,
) -> metamodelica::List<metamodelica::Ref<DAE::Dimension>> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_ARRAY { .. } => {
                return listAppend(
                    var_field!((**inType).dims, DAE::Type::T_ARRAY).clone(),
                    getDimensions(var_field!((**inType).ty, DAE::Type::T_ARRAY)),
                );
            }
            DAE::Type::T_METAARRAY { .. } => {
                return metamodelica::cons(
                    openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(),
                    getDimensions(var_field!((**inType).ty, DAE::Type::T_METAARRAY)),
                );
            }
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                inType = var_field!((**inType).complexType, DAE::Type::T_SUBTYPE_BASIC);
                continue '__tco;
            }
            DAE::Type::T_METATYPE { .. } => {
                inType = var_field!((**inType).ty, DAE::Type::T_METATYPE);
                continue '__tco;
            }
            _ => return metamodelica::nil(),
        }
    }
}
