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

use crate::NFCall as Call;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInst as Inst;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFFunctionInverse {
    pub inputParam: metamodelica::Ref<ComponentRef::NFComponentRef>,
    pub inverseCall: metamodelica::Ref<Expression::NFExpression>,
    pub info: SourceInfo,
}

impl metamodelica::gc::MMTrace for NFFunctionInverse {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.inputParam, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inverseCall, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
        Ok(())
    }
}
impl Default for NFFunctionInverse {
    fn default() -> Self {
        Self {
            inputParam: Default::default(),
            inverseCall: Default::default(),
            info: Default::default(),
        }
    }
}

pub type FUNCTION_INV = NFFunctionInverse;

pub(crate) fn instInverses(
    mut fnNode: metamodelica::Ref<InstNode::InstNode>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
) -> Result<metamodelica::Array<metamodelica::Ref<NFFunctionInverse>>> {
    let mut inverses: metamodelica::Array<metamodelica::Ref<NFFunctionInverse>>;
    let mut inv_mods: metamodelica::List<metamodelica::Ref<SCode::Mod>>;
    let mut invs: metamodelica::List<metamodelica::Ref<NFFunctionInverse>> = metamodelica::nil();
    inv_mods = getInverseAnnotations(&(InstNode::definition(fnNode.clone())?));
    if !((inv_mods).is_empty()) && !(((r#fn.outputs).len() as i32) == 1) {
        Error::addSourceMessage(
            &(Error::FUNCTION_INVALID_OUTPUTS_FOR_INVERSE.clone()),
            list![AbsynUtil::pathString(Function::name(r#fn), literal!("."), true, false)?],
            &(SCodeUtil::getModifierInfo(&((inv_mods).head().cloned()?))),
        )?;
        return Err("fail");
    }
    for mut m in &*inv_mods {
        invs = instInverseMod(metamodelica::AsArg::as_arg(&m), fnNode.clone(), r#fn, invs)?;
    }
    inverses = metamodelica::arrayFromVec(invs.into_iter().cloned().collect());
    Ok(inverses)
}

pub(crate) fn typeInverse(
    mut fnInv: metamodelica::Ref<NFFunctionInverse>,
) -> Result<metamodelica::Ref<NFFunctionInverse>> {
    let mut fnInv: metamodelica::Ref<NFFunctionInverse> = fnInv;
    assign_field!(
        fnInv.inputParam = Typing::typeCref(
            fnInv.inputParam.clone(),
            NFInstContext::RELAXED.clone(),
            &(fnInv.info.clone())
        )?
        .0,
        fnInv.inverseCall = Typing::typeExp(
            fnInv.inverseCall.clone(),
            NFInstContext::RELAXED.clone(),
            &(fnInv.info.clone()),
            false
        )?
        .0
    );
    Ok(fnInv)
}

pub(crate) fn toDAE(mut fnInv: &metamodelica::Ref<NFFunctionInverse>) -> Result<DAE::FunctionDefinition> {
    let mut invDef: DAE::FunctionDefinition;
    invDef = DAE::FunctionDefinition::FUNCTION_INVERSE {
        inputParam: ComponentRef::toDAE(&fnInv.inputParam)?,
        inverseCall: Expression::toDAE(fnInv.inverseCall.clone(), false)?,
    };
    Ok(invDef)
}

pub(crate) fn toSubMod(mut fnInv: &metamodelica::Ref<NFFunctionInverse>) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut subMod: metamodelica::Ref<SCode::SubMod>;
    let mut inv_mod: metamodelica::Ref<SCode::SubMod>;
    let mut call_exp: metamodelica::Ref<Absyn::Exp>;
    call_exp = Expression::toAbsyn(fnInv.inverseCall.clone())?;
    inv_mod = metamodelica::Ref::new(SCode::SubMod {
        ident: ComponentRef::firstName(&fnInv.inputParam, false)?,
        r#mod: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: Some(call_exp),
            comment: None,
            info: fnInv.info.clone(),
        }),
    });
    subMod = metamodelica::Ref::new(SCode::SubMod {
        ident: literal!("inverse"),
        r#mod: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: list![inv_mod],
            binding: None,
            comment: None,
            info: fnInv.info.clone(),
        }),
    });
    Ok(subMod)
}

pub(crate) fn getFunction(
    mut fnInv: &metamodelica::Ref<NFFunctionInverse>,
) -> Result<metamodelica::Ref<Function::Function>> {
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &(fnInv.inverseCall.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    call = metamodelica::Own::own(__pa0);
    r#fn = Call::typedFunction(&call)?;
    Ok(r#fn)
}

fn getInverseAnnotations(
    mut definition: &metamodelica::Ref<SCode::Element>,
) -> metamodelica::List<metamodelica::Ref<SCode::Mod>> {
    let mut invMods: metamodelica::List<metamodelica::Ref<SCode::Mod>>;
    invMods = (::match_deref::match_deref! { match definition {
        Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
            SCodeUtil::lookupAnnotations(metamodelica::AsArg::as_arg(&ann), &(literal!("inverse")))
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    invMods
}

fn instInverseMod(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut fnNode: metamodelica::Ref<InstNode::InstNode>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut fnInvs: metamodelica::List<metamodelica::Ref<NFFunctionInverse>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFFunctionInverse>>> {
    let mut fnInvs: metamodelica::List<metamodelica::Ref<NFFunctionInverse>> = fnInvs;
    fnInvs = (match &**r#mod {
        SCode::Mod::MOD {
            info: __mod_info,
            subModLst: __mod_subModLst,
            ..
        } => {
            for mut s in &*__mod_subModLst.clone() {
                fnInvs = instInverseSubMod(
                    metamodelica::AsArg::as_arg(&s),
                    fnNode.clone(),
                    r#fn,
                    __mod_info.clone(),
                    fnInvs,
                )?;
            }
            fnInvs
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFFunctionInverse.instInverseMod"));
                    __mm_s.push_str(&*literal!(" got invalid modifier"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFFunctionInverse.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(fnInvs)
}

fn instInverseSubMod(
    mut submod: &metamodelica::Ref<SCode::SubMod>,
    mut fnNode: metamodelica::Ref<InstNode::InstNode>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut info: SourceInfo,
    mut fnInvs: metamodelica::List<metamodelica::Ref<NFFunctionInverse>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFFunctionInverse>>> {
    let mut fnInvs: metamodelica::List<metamodelica::Ref<NFFunctionInverse>> = fnInvs;
    let mut name: ArcStr;
    let mut aparam: metamodelica::Ref<Absyn::ComponentRef>;
    let mut param: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut call_aexp: metamodelica::Ref<Absyn::Exp>;
    let mut call_exp: metamodelica::Ref<Expression::NFExpression>;
    fnInvs = (::match_deref::match_deref! { match submod {
        Deref @ SCode::SubMod { ident: __esc_name, r#mod: Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, binding: Some(__esc_call_aexp @ Deref @ Absyn::Exp::CALL { .. }), .. } } => {
            name = (*__esc_name).clone();
            call_aexp = (*__esc_call_aexp).clone();
            aparam = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() });
            match '__try0: {
                (param, _, _) = unwrap_break_err!(Lookup::lookupLocalCref(&aparam, fnNode.clone(), NFInstContext::RELAXED.clone(), &info), '__try0);
                let true = (InstNode::isInput(&(unwrap_break_err!(ComponentRef::node(&param), '__try0)))) else { break '__try0 Err::<_, _>("pattern mismatch") };
                Ok::<_, &'static str>((param.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    param = __try0_o0;
                }
                Err(__try0_err) => {
                    Error::addSourceMessage(&(Error::INVALID_FUNCTION_ANNOTATION_INPUT.clone()), list![name.clone(), AbsynUtil::pathString(Function::name(r#fn), literal!("."), true, false)?], &info)?;
                    return Err(__try0_err);
                }
            }
            call_exp = Inst::instExp(call_aexp.clone(), &fnNode, NFInstContext::RELAXED.clone(), &info)?;
            metamodelica::cons(metamodelica::Ref::new(NFFunctionInverse { inputParam: param, inverseCall: call_exp, info: info }), fnInvs)
        },
        Deref @ SCode::SubMod { .. } => {
            Error::addStrictMessage(Error::INVALID_FUNCTION_ANNOTATION_ATTR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*submod.ident); __mm_s.push_str(&*SCodeDump::printModStr(submod.r#mod.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }, literal!("inverse")], &info)?;
            fnInvs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(fnInvs)
}
