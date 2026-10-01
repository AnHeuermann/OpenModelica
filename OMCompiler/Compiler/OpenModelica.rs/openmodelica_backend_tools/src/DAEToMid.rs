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

use crate::HashTableMidVar;
use crate::MidToMid;
use openmodelica_ast::Absyn;
use openmodelica_codegen_util::MidCode;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub fn DAEFunctionsToMid(
    mut simfuncs: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
) -> Result<metamodelica::List<MidCode::Function>> {
    let mut midfuncs: metamodelica::List<MidCode::Function>;
    midfuncs = ({
        let mut __acc: metamodelica::List<MidCode::Function> = metamodelica::nil();
        for mut simfunc in (simfuncs).into_iter().cloned() {
            let __x = DAEFunctionToMid(&(simfunc.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(midfuncs)
}

#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct State {
    pub locals: DoubleEnded::MutableList<MidCode::Var>,
    pub localBufs: DoubleEnded::MutableList<MidCode::VarBuf>,
    pub localBufPtrs: DoubleEnded::MutableList<MidCode::VarBufPtr>,
    pub blocks: DoubleEnded::MutableList<MidCode::Block>,
    pub stmts: DoubleEnded::MutableList<MidCode::Stmt>,
    pub blockid: Mutable::Mutable<i32>,
    pub continuejumps: Mutable::Mutable<metamodelica::List<i32>>,
    pub breakjumps: Mutable::Mutable<metamodelica::List<i32>>,
    pub vars: Mutable::Mutable<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, MidCode::Var)>>,
        ),
        i32,
        (
            HashTableMidVar::FuncHashCref,
            HashTableMidVar::FuncCrefEqual,
            HashTableMidVar::FuncCrefStr,
            HashTableMidVar::FuncExpStr,
        ),
    )>,
}

impl metamodelica::gc::MMTrace for State {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.locals, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.localBufs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.localBufPtrs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.blocks, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stmts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.blockid, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.continuejumps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.breakjumps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        Ok(())
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.locals == other.locals
            && self.localBufs == other.localBufs
            && self.localBufPtrs == other.localBufPtrs
            && self.blocks == other.blocks
            && self.stmts == other.stmts
            && self.blockid == other.blockid
            && self.continuejumps == other.continuejumps
            && self.breakjumps == other.breakjumps
            && {
                let __lmut = Mutable::access((&self.vars).clone());
                let __rmut = Mutable::access((&other.vars).clone());
                (match ((&__lmut), (&__rmut)) {
                    ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                        (__lt0 == __rt0)
                            && (__lt1 == __rt1)
                            && (__lt2 == __rt2)
                            && (match (__lt3, __rt3) {
                                ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                    std::sync::Arc::ptr_eq(__lt0, __rt0)
                                        && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                        && std::sync::Arc::ptr_eq(__lt2, __rt2)
                                        && std::sync::Arc::ptr_eq(__lt3, __rt3)
                                }
                            })
                    }
                })
            }
    }
}
impl Eq for State {}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.locals
            .cmp(&other.locals)
            .then_with(|| self.localBufs.cmp(&other.localBufs))
            .then_with(|| self.localBufPtrs.cmp(&other.localBufPtrs))
            .then_with(|| self.blocks.cmp(&other.blocks))
            .then_with(|| self.stmts.cmp(&other.stmts))
            .then_with(|| self.blockid.cmp(&other.blockid))
            .then_with(|| self.continuejumps.cmp(&other.continuejumps))
            .then_with(|| self.breakjumps.cmp(&other.breakjumps))
            .then_with(|| {
                let __lmut = Mutable::access((&self.vars).clone());
                let __rmut = Mutable::access((&other.vars).clone());
                (match ((&__lmut), (&__rmut)) {
                    ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0
                        .cmp(__rt0)
                        .then_with(|| __lt1.cmp(__rt1))
                        .then_with(|| __lt2.cmp(__rt2))
                        .then_with(|| {
                            (match (__lt3, __rt3) {
                                ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                    (std::sync::Arc::as_ptr(__lt0) as *const ())
                                        .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                                        .then_with(|| {
                                            (std::sync::Arc::as_ptr(__lt1) as *const ())
                                                .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                                        })
                                        .then_with(|| {
                                            (std::sync::Arc::as_ptr(__lt2) as *const ())
                                                .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                                        })
                                        .then_with(|| {
                                            (std::sync::Arc::as_ptr(__lt3) as *const ())
                                                .cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))
                                        })
                                }
                            })
                        }),
                })
            })
    }
}
impl std::fmt::Debug for State {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("State");
        __ds.field("locals", &self.locals);
        __ds.field("localBufs", &self.localBufs);
        __ds.field("localBufPtrs", &self.localBufPtrs);
        __ds.field("blocks", &self.blocks);
        __ds.field("stmts", &self.stmts);
        __ds.field("blockid", &self.blockid);
        __ds.field("continuejumps", &self.continuejumps);
        __ds.field("breakjumps", &self.breakjumps);
        __ds.field(
            "vars",
            &format_args!("<dyn-fn-container@{:p}>", (&self.vars) as *const _),
        );
        __ds.finish()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            locals: Default::default(),
            localBufs: Default::default(),
            localBufPtrs: Default::default(),
            blocks: Default::default(),
            stmts: Default::default(),
            blockid: Default::default(),
            continuejumps: Default::default(),
            breakjumps: Default::default(),
            vars: Mutable::create((
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTableMidVar::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableMidVar::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableMidVar::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableMidVar::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            )),
        }
    }
}

pub type STATE = State;

fn listZip<X: Clone + 'static + metamodelica::gc::MMTrace, Y: Clone + 'static + metamodelica::gc::MMTrace>(
    mut xs: &metamodelica::List<X>,
    mut ys: &metamodelica::List<Y>,
) -> Result<metamodelica::List<(X, Y)>> {
    let mut zs: metamodelica::List<(X, Y)>;
    let mut xs_: metamodelica::List<X>;
    let mut ys_: metamodelica::List<Y>;
    let mut x: X;
    let mut y: Y;
    zs = (::match_deref::match_deref! { match (xs, ys) {
        (Deref @ metamodelica::ListNode::Nil, _) => metamodelica::nil(),
        (_, Deref @ metamodelica::ListNode::Nil) => metamodelica::nil(),
        (Deref @ metamodelica::ListNode::Cons { head: __esc_x, tail: __esc_xs_ }, Deref @ metamodelica::ListNode::Cons { head: __esc_y, tail: __esc_ys_ }) => {
            x = (*__esc_x).clone();
            xs_ = (*__esc_xs_).clone();
            y = (*__esc_y).clone();
            ys_ = (*__esc_ys_).clone();
            metamodelica::cons((x.clone(), y.clone()), listZip(metamodelica::AsArg::as_arg(&xs_), metamodelica::AsArg::as_arg(&ys_))?)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(zs)
}

fn GenTmpVar(mut ty: metamodelica::Ref<DAE::Type>, mut state: &State) -> Result<MidCode::Var> {
    let mut var: MidCode::Var;
    var = MidCode::Var {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("_tmp_"));
            __mm_s.push_str(&*intString(System::tmpTickIndex(46)));
            ArcStr::from(__mm_s)
        },
        ty: ty,
        volatile: false,
    };
    DoubleEnded::push_back(state.locals.clone(), var.clone())?;
    Ok(var)
}

fn GenTmpVarVolatile(mut ty: metamodelica::Ref<DAE::Type>, mut state: &State) -> Result<MidCode::Var> {
    let mut var: MidCode::Var;
    var = MidCode::Var {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("_tmp_"));
            __mm_s.push_str(&*intString(System::tmpTickIndex(46)));
            ArcStr::from(__mm_s)
        },
        ty: ty,
        volatile: true,
    };
    DoubleEnded::push_back(state.locals.clone(), var.clone())?;
    Ok(var)
}

fn GenTmpVarBuf(mut state: &State) -> Result<MidCode::VarBuf> {
    let mut var: MidCode::VarBuf;
    var = MidCode::VarBuf {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("_jmpbuf_"));
            __mm_s.push_str(&*intString(System::tmpTickIndex(47)));
            ArcStr::from(__mm_s)
        },
    };
    DoubleEnded::push_back(state.localBufs.clone(), var.clone())?;
    Ok(var)
}

fn GenTmpVarBufPtr(mut state: &State) -> Result<MidCode::VarBufPtr> {
    let mut var: MidCode::VarBufPtr;
    var = MidCode::VarBufPtr {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("_tmp_"));
            __mm_s.push_str(&*intString(System::tmpTickIndex(46)));
            ArcStr::from(__mm_s)
        },
    };
    DoubleEnded::push_back(state.localBufPtrs.clone(), var.clone())?;
    Ok(var)
}

fn GenBlockId() -> i32 {
    let mut id: i32;
    id = System::tmpTickIndex(45);
    id
}

fn ConvertSimCodeVars(
    mut simcodevar: &metamodelica::Ref<SimCodeFunction::Variable::Variable>,
    mut state: &State,
) -> Result<MidCode::Var> {
    let mut var: MidCode::Var;
    var = (match &**simcodevar {
        SimCodeFunction::Variable::VARIABLE {
            name: __simcodevar_name,
            value: __simcodevar_value,
            ..
        } => {
            let mut midcodevar: MidCode::Var;
            midcodevar = CrefToMidVar(__simcodevar_name.clone(), state)?;
            let () = (::match_deref::match_deref! { match &(__simcodevar_value.clone()) {
                None => {
                    ()
                },
                Some(exp) => {
                    stateAddStmt(MidCode::Stmt::ASSIGN { dest: midcodevar.clone(), src: ExpToMid(exp.clone(), state)? }, state)?;
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            midcodevar
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(var)
}

fn GetCrefIndexVar(mut cref: &metamodelica::Ref<DAE::ComponentRef>, mut state: &State) -> Result<Option<MidCode::Var>> {
    let mut var: Option<MidCode::Var>;
    let mut subscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    subscripts = ComponentReference::crefLastSubs(cref)?;
    var = (::match_deref::match_deref! { match &(subscripts) {
        Deref @ metamodelica::ListNode::Nil => {
            None
        },
        Deref @ metamodelica::ListNode::Cons { head: subscript @ Deref @ DAE::Subscript::INDEX { exp: _ }, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut indexvar: MidCode::Var;
            indexvar = RValueToVar(ExpToMid(var_field!((**subscript).exp, DAE::Subscript::INDEX).clone(), state)?, state)?;
            Some(indexvar)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(var)
}

fn CrefToMidVar(mut cref: metamodelica::Ref<DAE::ComponentRef>, mut state: &State) -> Result<MidCode::Var> {
    let mut var: MidCode::Var;
    let mut ident: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    if !(BaseHashTable::hasKey(cref.clone(), &(Mutable::access(state.vars.clone())))?) {
        (ident, ty) = (match &*cref {
            DAE::ComponentRef::CREF_IDENT {
                ident: ident_,
                identType: ty_,
                subscriptLst: _,
            } => (ident_.clone(), ty_.clone()),
            _ => {
                Error::addInternalError(
                    literal!("CrefToMidVar error"),
                    metamodelica::sourceInfo!("MidCode/DAEToMid.mo"),
                )?;
                return Err("fail");
            }
        });
        Mutable::update(
            state.vars.clone(),
            BaseHashTable::add(
                (
                    cref.clone(),
                    MidCode::Var {
                        name: ident,
                        ty: Types::complicateType(ty)?,
                        volatile: false,
                    },
                ),
                Mutable::access(state.vars.clone()),
            )?,
        );
    }
    var = BaseHashTable::get(cref, &(Mutable::access(state.vars.clone())))?;
    Ok(var)
}

fn RValueType(mut rvalue: &MidCode::RValue) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = (match rvalue.clone() {
        MidCode::RValue::VARIABLE { src: _ } => var_field!(rvalue.src, MidCode::RValue::VARIABLE).ty.clone(),
        MidCode::RValue::BINARYOP { op: _, .. } => {
            (match var_field!(rvalue.op, MidCode::RValue::BINARYOP).clone() {
                MidCode::BinaryOp::LESS { .. } => DAE::T_BOOL_DEFAULT().clone(),
                MidCode::BinaryOp::LESSEQ { .. } => DAE::T_BOOL_DEFAULT().clone(),
                MidCode::BinaryOp::GREATER { .. } => DAE::T_BOOL_DEFAULT().clone(),
                MidCode::BinaryOp::GREATEREQ { .. } => DAE::T_BOOL_DEFAULT().clone(),
                MidCode::BinaryOp::EQUAL { .. } => DAE::T_BOOL_DEFAULT().clone(),
                MidCode::BinaryOp::NEQUAL { .. } => DAE::T_BOOL_DEFAULT().clone(),
                _ => var_field!(rvalue.lsrc, MidCode::RValue::BINARYOP).ty.clone(),
            })
        }
        MidCode::RValue::UNARYOP {
            op: MidCode::UnaryOp::BOX { .. },
            src: _,
        } => Types::boxIfUnboxedType(var_field!(rvalue.src, MidCode::RValue::UNARYOP).ty.clone()),
        MidCode::RValue::UNARYOP {
            op: MidCode::UnaryOp::UNBOX { .. },
            src: _,
        } => Types::unboxedType(var_field!(rvalue.src, MidCode::RValue::UNARYOP).ty.clone())?,
        MidCode::RValue::UNARYOP { op: _, .. } => var_field!(rvalue.src, MidCode::RValue::UNARYOP).ty.clone(),
        MidCode::RValue::LITERALINTEGER { value: _ } => DAE::T_INTEGER_DEFAULT().clone(),
        MidCode::RValue::LITERALREAL { value: _ } => DAE::T_REAL_DEFAULT().clone(),
        MidCode::RValue::LITERALBOOLEAN { value: _ } => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::RValue::LITERALSTRING { value: _ } => DAE::T_STRING_DEFAULT().clone(),
        MidCode::RValue::LITERALMETATYPE { elements: _, .. } => {
            var_field!(rvalue.ty, MidCode::RValue::LITERALMETATYPE).clone()
        }
        MidCode::RValue::METAFIELD { src: _, .. } => var_field!(rvalue.ty, MidCode::RValue::METAFIELD).clone(),
        MidCode::RValue::UNIONTYPEVARIANT { src: _ } => DAE::T_INTEGER_DEFAULT().clone(),
        MidCode::RValue::ISCONS { src: _ } => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::RValue::ISSOME { src: _ } => DAE::T_BOOL_DEFAULT().clone(),
        _ => {
            Error::addInternalError(
                literal!("Could not find the correct type of an RValue.\n"),
                metamodelica::sourceInfo!("MidCode/DAEToMid.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(ty)
}

fn RValueToVar(mut rvalue: MidCode::RValue, mut state: &State) -> Result<MidCode::Var> {
    let mut var: MidCode::Var;
    var = (match rvalue.clone() {
        MidCode::RValue::VARIABLE { src: _ } => var_field!(rvalue.src, MidCode::RValue::VARIABLE).clone(),
        _ => {
            let mut tmpvar: MidCode::Var;
            tmpvar = GenTmpVar(Types::complicateType(RValueType(&rvalue)?)?, state)?;
            DoubleEnded::push_back(
                state.stmts.clone(),
                MidCode::Stmt::ASSIGN {
                    dest: tmpvar.clone(),
                    src: rvalue,
                },
            )?;
            tmpvar
        }
    });
    Ok(var)
}

fn DAEFunctionToMid(mut simfunc: &metamodelica::Ref<SimCodeFunction::Function::Function>) -> Result<MidCode::Function> {
    let mut midfunc: MidCode::Function;
    let mut state: State = <State as ::std::default::Default>::default();
    let mut inputs: DoubleEnded::MutableList<MidCode::Var> =
        <DoubleEnded::MutableList<MidCode::Var> as ::std::default::Default>::default();
    let mut outputs: DoubleEnded::MutableList<MidCode::Var> =
        <DoubleEnded::MutableList<MidCode::Var> as ::std::default::Default>::default();
    let mut path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut labelFirst: i32 = 0;
    System::tmpTickReset(47);
    System::tmpTickReset(46);
    System::tmpTickReset(45);
    let () = (match &**simfunc {
        SimCodeFunction::Function::FUNCTION {
            name,
            outVars,
            functionArguments,
            variableDeclarations,
            body,
            visibility: _,
            info: _,
        } => {
            labelFirst = GenBlockId();
            path = name.clone();
            inputs = DoubleEnded::fromList(&(metamodelica::nil()))?;
            outputs = DoubleEnded::fromList(&(metamodelica::nil()))?;
            state = State {
                locals: DoubleEnded::fromList(&(metamodelica::nil()))?,
                localBufs: DoubleEnded::fromList(&(metamodelica::nil()))?,
                localBufPtrs: DoubleEnded::fromList(&(metamodelica::nil()))?,
                blocks: DoubleEnded::fromList(&(metamodelica::nil()))?,
                stmts: DoubleEnded::fromList(&(metamodelica::nil()))?,
                blockid: Mutable::create(labelFirst),
                continuejumps: Mutable::create(metamodelica::nil()),
                breakjumps: Mutable::create(metamodelica::nil()),
                vars: Mutable::create(HashTableMidVar::emptyHashTable()),
            };
            for mut simcodeVar in &*variableDeclarations.clone() {
                DoubleEnded::push_back(
                    state.locals.clone(),
                    ConvertSimCodeVars(metamodelica::AsArg::as_arg(&simcodeVar), &state)?,
                )?;
            }
            for mut simcodeVar in &*outVars.clone() {
                DoubleEnded::push_back(
                    outputs.clone(),
                    ConvertSimCodeVars(metamodelica::AsArg::as_arg(&simcodeVar), &state)?,
                )?;
            }
            for mut simcodeVar in &*functionArguments.clone() {
                DoubleEnded::push_back(
                    inputs.clone(),
                    ConvertSimCodeVars(metamodelica::AsArg::as_arg(&simcodeVar), &state)?,
                )?;
            }
            StmtsToMid(body, &state)?;
            ()
        }
        _ => {
            Error::addInternalError(
                literal!("Unsupported SimCodeFunction.Function type\n"),
                metamodelica::sourceInfo!("MidCode/DAEToMid.mo"),
            )?;
            return Err("fail");
            ()
        }
    });
    stateTerminate(-1, openmodelica_codegen_util::MidCode::Terminator::RETURN, &state)?;
    midfunc = MidCode::Function {
        name: path,
        locals: DoubleEnded::toListAndClear(state.locals.clone(), metamodelica::nil())?,
        localBufs: DoubleEnded::toListAndClear(state.localBufs.clone(), metamodelica::nil())?,
        localBufPtrs: DoubleEnded::toListAndClear(state.localBufPtrs.clone(), metamodelica::nil())?,
        inputs: DoubleEnded::toListAndClear(inputs, metamodelica::nil())?,
        outputs: DoubleEnded::toListAndClear(outputs, metamodelica::nil())?,
        body: DoubleEnded::toListAndClear(state.blocks.clone(), metamodelica::nil())?,
        entryId: labelFirst,
        exitId: GenBlockId(),
    };
    midfunc = MidToMid::longJmpGoto(&midfunc)?;
    Ok(midfunc)
}

fn StmtsToMid(mut daestmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>, mut state: &State) -> Result<()> {
    let () = (::match_deref::match_deref! { match daestmts {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: stmt, tail: tail } => {
            let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { type_: _, exp1: exp1 @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, exp, source: _ } => {
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            let mut varCref: MidCode::Var;
            cref = ComponentReferenceBasics::crefLastCref(var_field!((**exp1).componentRef, DAE::Exp::CREF))?;
            varCref = CrefToMidVar(cref, state)?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varCref, src: ExpToMid(exp.clone(), state)? }, state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_ASSIGN { type_: _, exp1: exp1 @ Deref @ DAE::Exp::ASUB { exp: _, .. }, exp, source: _ } => {
            let mut varArray: MidCode::Var;
            let mut varIndex: MidCode::Var;
            let mut varValue: MidCode::Var;
            let mut labelNext: i32;
            varArray = RValueToVar(ExpToMid(var_field!((**exp1).exp, DAE::Exp::ASUB).clone(), state)?, state)?;
            varIndex = (::match_deref::match_deref! { match &(var_field!((**exp1).sub, DAE::Exp::ASUB).clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: indexexp }, tail: Deref @ metamodelica::ListNode::Nil } => {
            RValueToVar(ExpToMid(indexexp.clone(), state)?, state)?
        },
        _ => return Err("match: no arm matched"),
    } });
            varValue = RValueToVar(ExpToMid(exp.clone(), state)?, state)?;
            labelNext = GenBlockId();
            stateTerminate(labelNext, MidCode::Terminator::CALL { func: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("arrayUpdate") }), builtin: true, inputs: list![varArray, varIndex, varValue], outputs: metamodelica::nil(), next: labelNext }, state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_ASSIGN { type_: _, exp1: Deref @ DAE::Exp::PATTERN { pattern }, exp, source: _ } => {
            let mut varRHS: MidCode::Var;
            varRHS = RValueToVar(ExpToMid(exp.clone(), state)?, state)?;
            patternToMidCode(&(list![(varRHS, pattern.clone())]), 1, state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_ASSIGN { exp1: __stmt_exp1, .. } => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE.STMT_ASSIGN to Mid conversion failed ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(__stmt_exp1.clone(), 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: _, expExpLst: expLst, exp, source: _ } => {
            let mut exp1: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
            let mut varCref: MidCode::Var;
            let mut outvars: DoubleEnded::MutableList<MidCode::OutVar>;
            outvars = DoubleEnded::fromList(&(metamodelica::nil()))?;
            for mut exp1 in &*expLst.clone() {
                let mut exp1 = exp1.clone();
                let () = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => {
            DoubleEnded::push_back(outvars.clone(), openmodelica_codegen_util::MidCode::OutVar::OUT_WILD)?;
            ()
        },
        Deref @ DAE::Exp::CREF { componentRef: __exp1_componentRef, .. } => {
            varCref = CrefToMidVar(__exp1_componentRef.clone(), state)?;
            DoubleEnded::push_back(outvars.clone(), MidCode::OutVar::OUT_VAR { var: varCref })?;
            ()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("outvars convertion failed ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(exp1, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            let () = (match &*exp.clone() {
        DAE::Exp::CALL { path: _, .. } => {
            CallToMid(metamodelica::AsArg::as_arg(&exp), DoubleEnded::toListAndClear(outvars, metamodelica::nil())?, state)?;
            ()
        },
        DAE::Exp::MATCHEXPRESSION { matchType: _, .. } => {
            MatchExpressionToMid(metamodelica::AsArg::as_arg(&exp), &(DoubleEnded::toListAndClear(outvars, metamodelica::nil())?), state)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    });
            ()
        },
        Deref @ DAE::Statement::STMT_IF { else_: __stmt_else_, exp: __stmt_exp, statementLst: __stmt_statementLst, .. } => {
            IfToMid(__stmt_exp.clone(), metamodelica::AsArg::as_arg(&__stmt_statementLst), metamodelica::AsArg::as_arg(&__stmt_else_), state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_WHILE { exp: __stmt_exp, statementLst: __stmt_statementLst, .. } => {
            let mut varCondition: MidCode::Var;
            let mut labelBody: i32;
            let mut labelNext: i32;
            let mut labelCondition: i32;
            labelCondition = GenBlockId();
            labelBody = GenBlockId();
            labelNext = GenBlockId();
            Mutable::update(state.continuejumps.clone(), metamodelica::cons(labelCondition, Mutable::access(state.continuejumps.clone())));
            Mutable::update(state.breakjumps.clone(), metamodelica::cons(labelNext, Mutable::access(state.breakjumps.clone())));
            stateTerminate(labelCondition, MidCode::Terminator::GOTO { next: labelCondition }, state)?;
            varCondition = RValueToVar(ExpToMid(__stmt_exp.clone(), state)?, state)?;
            stateTerminate(labelBody, MidCode::Terminator::BRANCH { condition: varCondition, onTrue: labelBody, onFalse: labelNext }, state)?;
            StmtsToMid(metamodelica::AsArg::as_arg(&__stmt_statementLst), state)?;
            stateTerminate(labelNext, MidCode::Terminator::GOTO { next: labelCondition }, state)?;
            Mutable::update(state.continuejumps.clone(), ((Mutable::access(state.continuejumps.clone()))).rest()?);
            Mutable::update(state.breakjumps.clone(), ((Mutable::access(state.breakjumps.clone()))).rest()?);
            ()
        },
        Deref @ DAE::Statement::STMT_FOR { iter: __stmt_iter, range: __stmt_range, statementLst: __stmt_statementLst, type_: __stmt_type_, .. } => {
            ForToMid(__stmt_type_.clone(), __stmt_iter.clone(), __stmt_range.clone(), metamodelica::AsArg::as_arg(&__stmt_statementLst), state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_BREAK { source: _ } => {
            let mut labelNext: i32;
            labelNext = GenBlockId();
            stateTerminate(labelNext, MidCode::Terminator::GOTO { next: ((Mutable::access(state.breakjumps.clone()))).head().cloned()? }, state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_CONTINUE { source: _ } => {
            let mut labelNext: i32;
            labelNext = GenBlockId();
            stateTerminate(labelNext, MidCode::Terminator::GOTO { next: ((Mutable::access(state.continuejumps.clone()))).head().cloned()? }, state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_RETURN { source: _ } => {
            let mut labelNext: i32;
            labelNext = GenBlockId();
            stateTerminate(labelNext, openmodelica_codegen_util::MidCode::Terminator::RETURN, state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_NORETCALL { exp: __stmt_exp, .. } => {
            let () = (match &*__stmt_exp.clone() {
        DAE::Exp::CALL { path: _, .. } => {
            CallToMid(metamodelica::AsArg::as_arg(&__stmt_exp), metamodelica::nil(), state)?;
            ()
        },
        DAE::Exp::MATCHEXPRESSION { matchType: _, .. } => {
            MatchExpressionToMid(metamodelica::AsArg::as_arg(&__stmt_exp), &(metamodelica::nil()), state)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    });
            ()
        },
        Deref @ DAE::Statement::STMT_ASSERT { cond: __stmt_cond, level: __stmt_level, msg: __stmt_msg, .. } => {
            let mut varCondition: MidCode::Var;
            let mut varMessage: MidCode::Var;
            let mut varLevel: MidCode::Var;
            let mut labelNext: i32;
            varCondition = RValueToVar(ExpToMid(__stmt_cond.clone(), state)?, state)?;
            varMessage = RValueToVar(ExpToMid(__stmt_msg.clone(), state)?, state)?;
            varLevel = RValueToVar(ExpToMid(__stmt_level.clone(), state)?, state)?;
            labelNext = GenBlockId();
            stateTerminate(labelNext, MidCode::Terminator::ASSERT { condition: varCondition, message: varMessage, level: varLevel, next: labelNext }, state)?;
            ()
        },
        Deref @ DAE::Statement::STMT_TERMINATE { msg: __stmt_msg, .. } => {
            let mut varMessage: MidCode::Var;
            let mut labelNext: i32;
            varMessage = RValueToVar(ExpToMid(__stmt_msg.clone(), state)?, state)?;
            labelNext = GenBlockId();
            stateTerminate(labelNext, MidCode::Terminator::TERMINATE { message: varMessage }, state)?;
            ()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE.Statement to Mid conversion failed ")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            StmtsToMid(tail, state)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn ExpToMid<'__b>(mut exp: metamodelica::Ref<DAE::Exp>, mut state: &'__b State) -> Result<MidCode::RValue> {
    let mut rval: MidCode::RValue;
    rval = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: __exp_integer } => {
            MidCode::RValue::LITERALINTEGER { value: __exp_integer.clone() }
        },
        Deref @ DAE::Exp::ENUM_LITERAL { index: __exp_index, .. } => {
            MidCode::RValue::LITERALINTEGER { value: __exp_index.clone() }
        },
        Deref @ DAE::Exp::RCONST { real: __exp_real } => {
            MidCode::RValue::LITERALREAL { value: __exp_real.clone() }
        },
        Deref @ DAE::Exp::SCONST { string: __exp_string } => {
            MidCode::RValue::LITERALSTRING { value: __exp_string.clone() }
        },
        Deref @ DAE::Exp::SHARED_LITERAL { exp: __exp_exp, .. } => {
            ExpToMid(__exp_exp.clone(), state)?
        },
        Deref @ DAE::Exp::BOX { exp: __exp_exp } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(__exp_exp.clone(), state)?, state)?;
            MidCode::RValue::UNARYOP { op: openmodelica_codegen_util::MidCode::UnaryOp::BOX, src: varExp }
        },
        Deref @ DAE::Exp::UNBOX { exp: __exp_exp, .. } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(__exp_exp.clone(), state)?, state)?;
            MidCode::RValue::UNARYOP { op: openmodelica_codegen_util::MidCode::UnaryOp::UNBOX, src: varExp }
        },
        Deref @ DAE::Exp::BCONST { bool: __exp_bool } => {
            MidCode::RValue::LITERALBOOLEAN { value: __exp_bool.clone() }
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(exp1) } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
            MidCode::RValue::LITERALMETATYPE { elements: list![varExp.clone()], ty: Types::complicateType(metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: varExp.ty.clone() }))? }
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            MidCode::RValue::LITERALMETATYPE { elements: metamodelica::nil(), ty: Types::complicateType(DAE::T_NONE_DEFAULT().clone())? }
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expLst } => {
            let mut varExp: MidCode::Var;
            let mut values: DoubleEnded::MutableList<MidCode::Var>;
            values = DoubleEnded::fromList(&(metamodelica::nil()))?;
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                varExp = RValueToVar(ExpToMid(exp.clone(), state)?, state)?;
                DoubleEnded::push_back(values.clone(), varExp)?;
            }
            MidCode::RValue::LITERALMETATYPE { elements: DoubleEnded::toListAndClear(values, metamodelica::nil())?, ty: Types::complicateType(Expression::r#typeof(exp)?)? }
        },
        Deref @ DAE::Exp::METARECORDCALL { path: _, args: expLst, fieldNames: _, index: _, typeVars: _ } => {
            let mut varExp: MidCode::Var;
            let mut values: DoubleEnded::MutableList<MidCode::Var>;
            values = DoubleEnded::fromList(&(metamodelica::nil()))?;
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                varExp = RValueToVar(ExpToMid(exp.clone(), state)?, state)?;
                DoubleEnded::push_back(values.clone(), varExp)?;
            }
            MidCode::RValue::LITERALMETATYPE { elements: DoubleEnded::toListAndClear(values, metamodelica::nil())?, ty: Types::complicateType(Expression::r#typeof(exp)?)? }
        },
        Deref @ DAE::Exp::CONS { car: __exp_car, cdr: __exp_cdr } => {
            let mut varCar: MidCode::Var;
            let mut varCdr: MidCode::Var;
            varCar = RValueToVar(ExpToMid(__exp_car.clone(), state)?, state)?;
            varCdr = RValueToVar(ExpToMid(__exp_cdr.clone(), state)?, state)?;
            MidCode::RValue::LITERALMETATYPE { elements: list![varCar.clone(), varCdr], ty: Types::complicateType(metamodelica::Ref::new(DAE::Type::T_METALIST { ty: varCar.ty.clone() }))? }
        },
        Deref @ DAE::Exp::LIST { valList: expLst } => {
            let mut varCar: MidCode::Var;
            let mut varCdr: MidCode::Var;
            let mut varTmp: MidCode::Var;
            let mut expLst = (*expLst).clone();
            expLst = expLst.clone().reverse();
            varCdr = GenTmpVar(DAE::T_METALIST_DEFAULT().clone(), state)?;
            DoubleEnded::push_back(state.stmts.clone(), MidCode::Stmt::ASSIGN { dest: varCdr.clone(), src: MidCode::RValue::LITERALMETATYPE { elements: metamodelica::nil(), ty: DAE::T_METALIST_DEFAULT().clone() } })?;
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                varCar = RValueToVar(ExpToMid(exp, state)?, state)?;
                varTmp = GenTmpVar(metamodelica::Ref::new(DAE::Type::T_METALIST { ty: Types::complicateType(varCar.ty.clone())? }), state)?;
                DoubleEnded::push_back(state.stmts.clone(), MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: MidCode::RValue::LITERALMETATYPE { elements: list![varCar.clone(), varCdr], ty: Types::complicateType(metamodelica::Ref::new(DAE::Type::T_METALIST { ty: varCar.ty.clone() }))? } })?;
                varCdr = varTmp;
            }
            MidCode::RValue::VARIABLE { src: varCdr }
        },
        Deref @ DAE::Exp::CREF { componentRef: cref, ty: _ } => {
            let mut varCref: MidCode::Var;
            let mut varTmp: MidCode::Var;
            let mut labelNext: i32;
            let mut rvalue: MidCode::RValue;
            varCref = CrefToMidVar(cref.clone(), state)?;
            rvalue = (match GetCrefIndexVar(metamodelica::AsArg::as_arg(&cref), state)? {
        None => {
            MidCode::RValue::VARIABLE { src: varCref }
        },
        Some(mut indexvar) => {
            labelNext = GenBlockId();
            varTmp = GenTmpVar(Types::complicateType(Expression::r#typeof(exp)?)?, state)?;
            stateTerminate(labelNext, MidCode::Terminator::CALL { func: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("arrayGet") }), builtin: true, inputs: list![varCref, indexvar], outputs: list![MidCode::OutVar::OUT_VAR { var: varTmp.clone() }], next: labelNext }, state)?;
            MidCode::RValue::VARIABLE { src: varTmp }
        },
    });
            rvalue
        },
        Deref @ DAE::Exp::ASUB { exp: exp1, sub: subscripts } => {
            let mut varExp: MidCode::Var;
            let mut varExp2: MidCode::Var;
            let mut varTmp: MidCode::Var;
            let mut labelNext: i32;
            let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut sub in (subscripts.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            varExp = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
            varExp2 = (::match_deref::match_deref! { match &(expLst) {
        Deref @ metamodelica::ListNode::Cons { head: indexexp, tail: Deref @ metamodelica::ListNode::Nil } => {
            RValueToVar(ExpToMid(indexexp.clone(), state)?, state)?
        },
        _ => return Err("match: no arm matched"),
    } });
            varTmp = GenTmpVar(Types::complicateType(Expression::r#typeof(exp)?)?, state)?;
            labelNext = GenBlockId();
            stateTerminate(labelNext, MidCode::Terminator::CALL { func: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("arrayGet") }), builtin: true, inputs: list![varExp, varExp2], outputs: list![MidCode::OutVar::OUT_VAR { var: varTmp.clone() }], next: labelNext }, state)?;
            MidCode::RValue::VARIABLE { src: varTmp }
        },
        Deref @ DAE::Exp::TSUB { exp: exp1 @ Deref @ DAE::Exp::CALL { path: _, expLst: _, attr: callattrs }, ix: 1, ty: _ } => {
            let mut varTmp: MidCode::Var;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut numTailTypes: i32;
            let mut outvars: metamodelica::List<MidCode::OutVar>;
            (ty, numTailTypes) = (::match_deref::match_deref! { match &(callattrs.ty.clone()) {
        Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: actualType, tail: tailTypes }, .. } => {
            (actualType.clone(), ((tailTypes).len() as i32))
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            varTmp = GenTmpVar(Types::complicateType(ty)?, state)?;
            outvars = metamodelica::nil();
            for mut i in 1..=numTailTypes {
                outvars = metamodelica::cons(openmodelica_codegen_util::MidCode::OutVar::OUT_WILD, outvars);
            }
            outvars = metamodelica::cons(MidCode::OutVar::OUT_VAR { var: varTmp.clone() }, outvars);
            CallToMid(metamodelica::AsArg::as_arg(&exp1), outvars, state)?;
            MidCode::RValue::VARIABLE { src: varTmp }
        },
        Deref @ DAE::Exp::TSUB { exp: __exp_exp, ix: __exp_ix, ty: __exp_ty } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(__exp_exp.clone(), state)?, state)?;
            MidCode::RValue::METAFIELD { src: varExp, index: __exp_ix.clone(), ty: Types::complicateType(__exp_ty.clone())? }
        },
        Deref @ DAE::Exp::RSUB { exp: __exp_exp, ix: __exp_ix, ty: __exp_ty, .. } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(__exp_exp.clone(), state)?, state)?;
            MidCode::RValue::METAFIELD { src: varExp, index: __exp_ix.clone(), ty: Types::complicateType(__exp_ty.clone())? }
        },
        Deref @ DAE::Exp::CAST { ty: _, exp: exp1 } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
            MidCode::RValue::UNARYOP { op: openmodelica_codegen_util::MidCode::UnaryOp::MOVE, src: varExp }
        },
        Deref @ DAE::Exp::LUNARY { operator: _, exp: exp1 } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
            MidCode::RValue::UNARYOP { op: openmodelica_codegen_util::MidCode::UnaryOp::NOT, src: varExp }
        },
        Deref @ DAE::Exp::LBINARY { exp1, operator, exp2 } => {
            let mut varTmp: MidCode::Var;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut labelElse: i32;
            let mut labelNext: i32;
            let mut terminator: MidCode::Terminator;
            labelElse = GenBlockId();
            labelNext = GenBlockId();
            ty = (match operator.clone() {
        DAE::Operator::AND { ty: _ } => var_field!(operator.ty, DAE::Operator::AND).clone(),
        DAE::Operator::OR { ty: _ } => var_field!(operator.ty, DAE::Operator::OR).clone(),
        _ => return Err("match: no arm matched"),
    });
            varTmp = GenTmpVar(ty, state)?;
            terminator = (match operator.clone() {
        DAE::Operator::AND { ty: _ } => MidCode::Terminator::BRANCH { condition: varTmp.clone(), onTrue: labelElse, onFalse: labelNext },
        DAE::Operator::OR { ty: _ } => MidCode::Terminator::BRANCH { condition: varTmp.clone(), onTrue: labelNext, onFalse: labelElse },
        _ => return Err("match: no arm matched"),
    });
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: ExpToMid(exp1.clone(), state)? }, state)?;
            stateTerminate(labelElse, terminator, state)?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: ExpToMid(exp2.clone(), state)? }, state)?;
            stateTerminate(labelNext, MidCode::Terminator::GOTO { next: labelNext }, state)?;
            MidCode::RValue::VARIABLE { src: varTmp }
        },
        Deref @ DAE::Exp::UNARY { operator, exp: exp1 } => {
            let mut varExp: MidCode::Var;
            let mut unop: MidCode::UnaryOp;
            unop = (match operator.clone() {
        DAE::Operator::UMINUS { ty: _ } => openmodelica_codegen_util::MidCode::UnaryOp::UMINUS,
        _ => return Err("match: no arm matched"),
    });
            varExp = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
            MidCode::RValue::UNARYOP { op: unop, src: varExp }
        },
        Deref @ DAE::Exp::BINARY { exp1, operator, exp2 } => {
            let mut varExp: MidCode::Var;
            let mut varExp2: MidCode::Var;
            let mut binop: MidCode::BinaryOp;
            binop = (match operator.clone() {
        DAE::Operator::ADD { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::ADD,
        DAE::Operator::SUB { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::SUB,
        DAE::Operator::MUL { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::MUL,
        DAE::Operator::DIV { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::DIV,
        DAE::Operator::POW { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::POW,
        _ => return Err("match: no arm matched"),
    });
            varExp = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
            varExp2 = RValueToVar(ExpToMid(exp2.clone(), state)?, state)?;
            MidCode::RValue::BINARYOP { op: binop, lsrc: varExp, rsrc: varExp2 }
        },
        Deref @ DAE::Exp::RELATION { exp1, operator, exp2, index: _, optionExpisASUB: _ } => {
            let mut varExp: MidCode::Var;
            let mut varExp2: MidCode::Var;
            let mut binop: MidCode::BinaryOp;
            binop = (match operator.clone() {
        DAE::Operator::LESS { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::LESS,
        DAE::Operator::LESSEQ { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::LESSEQ,
        DAE::Operator::GREATER { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::GREATER,
        DAE::Operator::GREATEREQ { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::GREATEREQ,
        DAE::Operator::EQUAL { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::EQUAL,
        DAE::Operator::NEQUAL { ty: _ } => openmodelica_codegen_util::MidCode::BinaryOp::NEQUAL,
        _ => return Err("match: no arm matched"),
    });
            varExp = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
            varExp2 = RValueToVar(ExpToMid(exp2.clone(), state)?, state)?;
            MidCode::RValue::BINARYOP { op: binop, lsrc: varExp, rsrc: varExp2 }
        },
        Deref @ DAE::Exp::IFEXP { expCond: exp1, expThen: exp2, expElse: exp3 } => {
            let mut varExp: MidCode::Var;
            let mut varTmp: MidCode::Var;
            let mut labelBody: i32;
            let mut labelElse: i32;
            let mut labelNext: i32;
            labelBody = GenBlockId();
            labelElse = GenBlockId();
            labelNext = GenBlockId();
            varExp = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
            varTmp = GenTmpVar(Types::complicateType(Expression::r#typeof(exp2.clone())?)?, state)?;
            stateTerminate(labelBody, MidCode::Terminator::BRANCH { condition: varExp, onTrue: labelBody, onFalse: labelElse }, state)?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: ExpToMid(exp2.clone(), state)? }, state)?;
            stateTerminate(labelElse, MidCode::Terminator::GOTO { next: labelNext }, state)?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: ExpToMid(exp3.clone(), state)? }, state)?;
            stateTerminate(labelNext, MidCode::Terminator::GOTO { next: labelNext }, state)?;
            MidCode::RValue::VARIABLE { src: varTmp }
        },
        Deref @ DAE::Exp::CALL { path: _, expLst: _, attr: callattrs } => {
            let mut varTmp: MidCode::Var;
            varTmp = GenTmpVar(Types::complicateType(callattrs.ty.clone())?, state)?;
            CallToMid(&exp, list![MidCode::OutVar::OUT_VAR { var: varTmp.clone() }], state)?;
            MidCode::RValue::VARIABLE { src: varTmp }
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { et: ty, .. } => {
            let mut varTmp: MidCode::Var;
            varTmp = GenTmpVar(Types::complicateType(ty.clone())?, state)?;
            let () = (match &*(Types::complicateType(ty.clone())?) {
        DAE::Type::T_TUPLE { types: _, .. } => {
            Error::addInternalError(literal!("Not supposed to get tuple here.\n"), metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        _ => (),
    });
            MatchExpressionToMid(&exp, &(list![MidCode::OutVar::OUT_VAR { var: varTmp.clone() }]), state)?;
            MidCode::RValue::VARIABLE { src: varTmp }
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE.Exp to Mid conversion failed:\n")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(exp, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rval)
}

fn CallToMid(
    mut call: &metamodelica::Ref<DAE::Exp>,
    mut outvars: metamodelica::List<MidCode::OutVar>,
    mut state: &State,
) -> Result<()> {
    let () = (match &**call {
        DAE::Exp::CALL {
            path,
            expLst,
            attr: callattr,
        } => {
            let mut labelNext: i32;
            let mut inputs: DoubleEnded::MutableList<MidCode::Var>;
            let mut var1: MidCode::Var;
            labelNext = GenBlockId();
            inputs = DoubleEnded::fromList(&(metamodelica::nil()))?;
            for mut exp1 in &*expLst.clone() {
                var1 = RValueToVar(ExpToMid(exp1.clone(), state)?, state)?;
                DoubleEnded::push_back(inputs.clone(), var1)?;
            }
            stateTerminate(
                labelNext,
                MidCode::Terminator::CALL {
                    func: path.clone(),
                    builtin: callattr.builtin.clone(),
                    inputs: DoubleEnded::toListAndClear(inputs, metamodelica::nil())?,
                    outputs: outvars,
                    next: labelNext,
                },
                state,
            )?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn ForToMid(
    mut type_: metamodelica::Ref<DAE::Type>,
    mut iter: ArcStr,
    mut range: metamodelica::Ref<DAE::Exp>,
    mut daestmtLst: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut state: &State,
) -> Result<()> {
    let mut varCref: MidCode::Var;
    let mut varCondition: MidCode::Var;
    let mut labelCondition: i32;
    let mut labelStep: i32;
    let mut labelBody: i32;
    let mut labelNext: i32;
    varCref = CrefToMidVar(
        metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
            ident: iter,
            identType: type_,
            subscriptLst: metamodelica::nil(),
        }),
        state,
    )?;
    DoubleEnded::push_back(state.locals.clone(), varCref.clone())?;
    labelCondition = GenBlockId();
    labelStep = GenBlockId();
    labelBody = GenBlockId();
    labelNext = GenBlockId();
    Mutable::update(
        state.continuejumps.clone(),
        metamodelica::cons(labelStep, Mutable::access(state.continuejumps.clone())),
    );
    Mutable::update(
        state.breakjumps.clone(),
        metamodelica::cons(labelNext, Mutable::access(state.breakjumps.clone())),
    );
    varCondition = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state)?;
    let () = (match &*range {
        DAE::Exp::RANGE {
            ty: _,
            start,
            step,
            stop,
        } => {
            let mut varFirst: MidCode::Var;
            let mut varIter: MidCode::Var;
            let mut varLast: MidCode::Var;
            let mut varStep: MidCode::Var;
            let mut labelCondition2: i32;
            let mut rvalueStep: MidCode::RValue;
            labelCondition2 = GenBlockId();
            varFirst = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state)?;
            varIter = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state)?;
            varLast = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state)?;
            varStep = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state)?;
            stateAddStmt(
                MidCode::Stmt::ASSIGN {
                    dest: varFirst.clone(),
                    src: ExpToMid(start.clone(), state)?,
                },
                state,
            )?;
            stateAddStmt(
                MidCode::Stmt::ASSIGN {
                    dest: varIter.clone(),
                    src: ExpToMid(start.clone(), state)?,
                },
                state,
            )?;
            stateAddStmt(
                MidCode::Stmt::ASSIGN {
                    dest: varLast.clone(),
                    src: ExpToMid(stop.clone(), state)?,
                },
                state,
            )?;
            rvalueStep = (::match_deref::match_deref! { match &(step.clone()) {
                None => {
                    MidCode::RValue::LITERALINTEGER { value: 1 }
                },
                Some(stepexp) => {
                    ExpToMid(stepexp.clone(), state)?
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            stateAddStmt(
                MidCode::Stmt::ASSIGN {
                    dest: varStep.clone(),
                    src: rvalueStep,
                },
                state,
            )?;
            stateTerminate(
                labelCondition,
                MidCode::Terminator::GOTO { next: labelCondition },
                state,
            )?;
            stateTerminate(
                labelCondition2,
                MidCode::Terminator::CALL {
                    func: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("in_range_integer"),
                    }),
                    builtin: true,
                    inputs: list![varIter.clone(), varFirst, varLast],
                    outputs: list![MidCode::OutVar::OUT_VAR {
                        var: varCondition.clone()
                    }],
                    next: labelCondition2,
                },
                state,
            )?;
            stateTerminate(
                labelBody,
                MidCode::Terminator::BRANCH {
                    condition: varCondition,
                    onTrue: labelBody,
                    onFalse: labelNext,
                },
                state,
            )?;
            stateAddStmt(
                MidCode::Stmt::ASSIGN {
                    dest: varCref,
                    src: MidCode::RValue::VARIABLE { src: varIter.clone() },
                },
                state,
            )?;
            StmtsToMid(daestmtLst, state)?;
            stateTerminate(labelStep, MidCode::Terminator::GOTO { next: labelStep }, state)?;
            stateAddStmt(
                MidCode::Stmt::ASSIGN {
                    dest: varIter.clone(),
                    src: MidCode::RValue::BINARYOP {
                        op: openmodelica_codegen_util::MidCode::BinaryOp::ADD,
                        lsrc: varIter,
                        rsrc: varStep,
                    },
                },
                state,
            )?;
            stateTerminate(labelNext, MidCode::Terminator::GOTO { next: labelCondition }, state)?;
            ()
        }
        _ => {
            let mut varRange: MidCode::Var;
            let mut varIter: MidCode::Var;
            let mut varLast: MidCode::Var;
            let mut varStep: MidCode::Var;
            let mut labelBody2: i32;
            varRange = RValueToVar(ExpToMid(range, state)?, state)?;
            let () = (match &*varRange.ty.clone() {
                DAE::Type::T_METATYPE { ty: _ } => {
                    Error::addInternalError(
                        literal!("metatype error"),
                        metamodelica::sourceInfo!("MidCode/DAEToMid.mo"),
                    )?;
                    return Err("fail");
                }
                DAE::Type::T_METAARRAY { ty: _ } => {
                    labelBody2 = GenBlockId();
                    varIter = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state)?;
                    varLast = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state)?;
                    varStep = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state)?;
                    stateAddStmt(
                        MidCode::Stmt::ASSIGN {
                            dest: varIter.clone(),
                            src: MidCode::RValue::LITERALINTEGER { value: 1 },
                        },
                        state,
                    )?;
                    stateAddStmt(
                        MidCode::Stmt::ASSIGN {
                            dest: varStep.clone(),
                            src: MidCode::RValue::LITERALINTEGER { value: 1 },
                        },
                        state,
                    )?;
                    stateTerminate(
                        labelCondition,
                        MidCode::Terminator::CALL {
                            func: metamodelica::Ref::new(Absyn::Path::IDENT {
                                name: literal!("arrayLength"),
                            }),
                            builtin: true,
                            inputs: list![varRange.clone()],
                            outputs: list![MidCode::OutVar::OUT_VAR { var: varLast.clone() }],
                            next: labelCondition,
                        },
                        state,
                    )?;
                    stateAddStmt(
                        MidCode::Stmt::ASSIGN {
                            dest: varCondition.clone(),
                            src: MidCode::RValue::BINARYOP {
                                op: openmodelica_codegen_util::MidCode::BinaryOp::LESSEQ,
                                lsrc: varIter.clone(),
                                rsrc: varLast,
                            },
                        },
                        state,
                    )?;
                    stateTerminate(
                        labelBody,
                        MidCode::Terminator::BRANCH {
                            condition: varCondition,
                            onTrue: labelBody,
                            onFalse: labelNext,
                        },
                        state,
                    )?;
                    stateTerminate(
                        labelBody2,
                        MidCode::Terminator::CALL {
                            func: metamodelica::Ref::new(Absyn::Path::IDENT {
                                name: literal!("arrayGet"),
                            }),
                            builtin: true,
                            inputs: list![varRange, varIter.clone()],
                            outputs: list![MidCode::OutVar::OUT_VAR { var: varCref }],
                            next: labelBody2,
                        },
                        state,
                    )?;
                    StmtsToMid(daestmtLst, state)?;
                    stateTerminate(labelStep, MidCode::Terminator::GOTO { next: labelStep }, state)?;
                    stateAddStmt(
                        MidCode::Stmt::ASSIGN {
                            dest: varIter.clone(),
                            src: MidCode::RValue::BINARYOP {
                                op: openmodelica_codegen_util::MidCode::BinaryOp::ADD,
                                lsrc: varIter,
                                rsrc: varStep,
                            },
                        },
                        state,
                    )?;
                    stateTerminate(labelNext, MidCode::Terminator::GOTO { next: labelCondition }, state)?;
                    ()
                }
                DAE::Type::T_METALIST { ty: _ } => {
                    labelBody2 = GenBlockId();
                    varIter = varRange;
                    stateTerminate(
                        labelCondition,
                        MidCode::Terminator::GOTO { next: labelCondition },
                        state,
                    )?;
                    stateAddStmt(
                        MidCode::Stmt::ASSIGN {
                            dest: varCondition.clone(),
                            src: MidCode::RValue::ISCONS { src: varIter.clone() },
                        },
                        state,
                    )?;
                    stateTerminate(
                        labelBody,
                        MidCode::Terminator::BRANCH {
                            condition: varCondition,
                            onTrue: labelBody,
                            onFalse: labelNext,
                        },
                        state,
                    )?;
                    stateTerminate(
                        labelBody2,
                        MidCode::Terminator::CALL {
                            func: metamodelica::Ref::new(Absyn::Path::IDENT {
                                name: literal!("listHead"),
                            }),
                            builtin: true,
                            inputs: list![varIter.clone()],
                            outputs: list![MidCode::OutVar::OUT_VAR { var: varCref }],
                            next: labelBody2,
                        },
                        state,
                    )?;
                    StmtsToMid(daestmtLst, state)?;
                    stateTerminate(labelStep, MidCode::Terminator::GOTO { next: labelStep }, state)?;
                    stateTerminate(
                        labelNext,
                        MidCode::Terminator::CALL {
                            func: metamodelica::Ref::new(Absyn::Path::IDENT {
                                name: literal!("listRest"),
                            }),
                            builtin: true,
                            inputs: list![varIter.clone()],
                            outputs: list![MidCode::OutVar::OUT_VAR { var: varIter }],
                            next: labelCondition,
                        },
                        state,
                    )?;
                    ()
                }
                _ => {
                    Error::addInternalError(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("unknown for type "));
                            __mm_s.push_str(&*DAEDump::daeTypeStr(&varRange.ty)?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        },
                        metamodelica::sourceInfo!("MidCode/DAEToMid.mo"),
                    )?;
                    return Err("fail");
                }
            });
            ()
        }
    });
    Mutable::update(
        state.continuejumps.clone(),
        (Mutable::access(state.continuejumps.clone())).rest()?,
    );
    Mutable::update(
        state.breakjumps.clone(),
        (Mutable::access(state.breakjumps.clone())).rest()?,
    );
    Ok(())
}

fn IfToMid(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut daestmtLst: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut else_: &metamodelica::Ref<DAE::Else>,
    mut state: &State,
) -> Result<()> {
    let mut labelBody: i32;
    let mut labelElse: i32;
    let mut labelNext: i32;
    let mut var1: MidCode::Var;
    labelBody = GenBlockId();
    labelElse = GenBlockId();
    labelNext = GenBlockId();
    var1 = RValueToVar(ExpToMid(exp, state)?, state)?;
    stateTerminate(
        labelBody,
        MidCode::Terminator::BRANCH {
            condition: var1,
            onTrue: labelBody,
            onFalse: labelElse,
        },
        state,
    )?;
    StmtsToMid(daestmtLst, state)?;
    stateTerminate(labelElse, MidCode::Terminator::GOTO { next: labelNext }, state)?;
    let () = (match &**else_ {
        DAE::Else::NOELSE { .. } => (),
        DAE::Else::ELSEIF {
            exp: subexp,
            statementLst: subdaestmtLst,
            else_: subelse,
        } => {
            IfToMid(subexp.clone(), subdaestmtLst, subelse, state)?;
            ()
        }
        DAE::Else::ELSE {
            statementLst: subdaestmtLst,
        } => {
            StmtsToMid(subdaestmtLst, state)?;
            ()
        }
    });
    stateTerminate(labelNext, MidCode::Terminator::GOTO { next: labelNext }, state)?;
    Ok(())
}

fn stateGetCurrentLabel(mut state: &State) -> i32 {
    let mut label: i32;
    label = Mutable::access(state.blockid.clone());
    label
}

fn stateSetCurrentLabel(mut label: i32, mut state: &State) -> () {
    Mutable::update(state.blockid.clone(), label);
    ()
}

fn stateAddStmt(mut stmt: MidCode::Stmt, mut state: &State) -> Result<()> {
    DoubleEnded::push_back(state.stmts.clone(), stmt)?;
    Ok(())
}

fn stateTerminate(mut newLabel: i32, mut terminator: MidCode::Terminator, mut state: &State) -> Result<()> {
    let mut block_: MidCode::Block;
    block_ = MidCode::Block {
        id: stateGetCurrentLabel(state),
        stmts: DoubleEnded::toListAndClear(state.stmts.clone(), metamodelica::nil())?,
        terminator: terminator,
    };
    DoubleEnded::push_back(state.blocks.clone(), block_)?;
    stateSetCurrentLabel(newLabel, state);
    Ok(())
}

// helper
fn stateAddBailOnFalse(mut var: MidCode::Var, mut labelBail: i32, mut state: &State) -> Result<()> {
    let mut labelTmp: i32;
    labelTmp = GenBlockId();
    stateTerminate(
        labelTmp,
        MidCode::Terminator::BRANCH {
            condition: var,
            onFalse: labelBail,
            onTrue: labelTmp,
        },
        state,
    )?;
    Ok(())
}

fn unpackCrefFromExp(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    cref = (match &**exp {
        DAE::Exp::CREF {
            componentRef: __esc_cref,
            ..
        } => {
            cref = (*__esc_cref).clone();
            cref.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cref)
}

//TODO: stuff needs to be volatile for setjmp.
//TODO: could handle match separately from matchcontinue and add more simplifications
/*
The term matchexpression is used to include both matchcontinue and match.
*/
fn MatchExpressionToMid(
    mut matchexpression: &metamodelica::Ref<DAE::Exp>,
    mut outvars: &metamodelica::List<MidCode::OutVar>,
    mut state: &State,
) -> Result<()> {
    let mut labelFin: i32;
    let mut labelMux: i32;
    let mut labelInit: i32;
    let mut labelFail: i32;
    let mut labelFin2: i32;
    let mut labelOut: i32;
    let mut caseLabel: i32;
    let mut caseLabels: metamodelica::List<i32>;
    let mut muxState: MidCode::Var;
    let mut one: MidCode::Var;
    let mut midvar: MidCode::Var;
    let mut midvar2: MidCode::Var;
    let mut muxOldBuf: MidCode::VarBufPtr = MidCode::VarBufPtr { name: literal!("") };
    let mut muxNewBuf: MidCode::VarBuf;
    let mut outvar: MidCode::OutVar;
    let mut matchContinue: bool;
    let mut matchType: DAE::MatchType;
    let mut cases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    let mut inputsCref: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut aliases: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut srcVar: MidCode::Var;
    let mut aliasVar: MidCode::Var;
    let mut aliasList: metamodelica::List<ArcStr>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut inputsMidVar: metamodelica::List<MidCode::Var>;
    let mut daeExp: metamodelica::Ref<DAE::Exp>;
    let mut caseLabelIterator: metamodelica::List<i32>;
    let () = (match &**matchexpression {
        DAE::Exp::MATCHEXPRESSION {
            matchType: __esc_matchType,
            cases: __esc_cases,
            inputs: __esc_inputsCref,
            aliases: __esc_aliases,
            ..
        } => {
            matchType = (*__esc_matchType).clone();
            cases = (*__esc_cases).clone();
            inputsCref = (*__esc_inputsCref).clone();
            aliases = (*__esc_aliases).clone();
            labelInit = stateGetCurrentLabel(state);
            labelMux = GenBlockId();
            labelFin = GenBlockId();
            matchContinue = (match matchType.clone() {
                DAE::MatchType::MATCHCONTINUE { .. } => true,
                DAE::MatchType::MATCH { .. } => false,
                _ => return Err("match: no arm matched"),
            });
            caseLabels = metamodelica::nil();
            for mut i in 1..=((cases).len() as i32) {
                caseLabels = metamodelica::cons(GenBlockId(), caseLabels);
            }
            assert!(
                ((inputsCref).len() as i32) == ((aliases).len() as i32),
                "{}",
                &*literal!("MatchExpressionToMid: incorrect input: listLength(inputs) != listLength(aliases)")
            );
            inputsMidVar = metamodelica::nil();
            for mut daeExp_aliasList in &*List::zip(inputsCref.clone(), aliases.clone()) {
                (daeExp, aliasList) = daeExp_aliasList.clone();
                srcVar = RValueToVar(ExpToMid(daeExp.clone(), state)?, state)?;
                ty = RValueType(&(MidCode::RValue::VARIABLE { src: srcVar.clone() }))?;
                inputsMidVar = metamodelica::cons(srcVar.clone(), inputsMidVar);
                for mut alias in &*aliasList {
                    aliasVar = MidCode::Var {
                        name: alias.clone(),
                        ty: ty.clone(),
                        volatile: false,
                    };
                    DoubleEnded::push_back(state.locals.clone(), aliasVar.clone())?;
                    stateAddStmt(
                        MidCode::Stmt::ASSIGN {
                            dest: aliasVar,
                            src: MidCode::RValue::VARIABLE { src: srcVar.clone() },
                        },
                        state,
                    )?;
                }
            }
            muxState = GenTmpVarVolatile(DAE::T_INTEGER_DEFAULT().clone(), state)?;
            stateAddStmt(
                MidCode::Stmt::ASSIGN {
                    dest: muxState.clone(),
                    src: MidCode::RValue::LITERALINTEGER { value: 0 },
                },
                state,
            )?;
            if matchContinue {
                muxOldBuf = GenTmpVarBufPtr(state)?;
                muxNewBuf = GenTmpVarBuf(state)?;
                stateTerminate(
                    labelMux,
                    MidCode::Terminator::PUSHJMP {
                        old_buf: muxOldBuf.clone(),
                        new_buf: muxNewBuf,
                        next: labelMux,
                    },
                    state,
                )?;
            } else {
                stateTerminate(labelMux, MidCode::Terminator::GOTO { next: labelMux }, state)?;
            }
            if matchContinue {
                one = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state)?;
                stateAddStmt(
                    MidCode::Stmt::ASSIGN {
                        dest: one.clone(),
                        src: MidCode::RValue::LITERALINTEGER { value: 1 },
                    },
                    state,
                )?;
                stateAddStmt(
                    MidCode::Stmt::ASSIGN {
                        dest: muxState.clone(),
                        src: MidCode::RValue::BINARYOP {
                            op: openmodelica_codegen_util::MidCode::BinaryOp::ADD,
                            lsrc: muxState.clone(),
                            rsrc: one,
                        },
                    },
                    state,
                )?;
                stateTerminate(
                    labelFin,
                    MidCode::Terminator::SWITCH {
                        condition: muxState.clone(),
                        cases: List::zip(
                            List::intRange(((cases).len() as i32) + 1),
                            listAppend(caseLabels.clone(), list![labelFin]),
                        ),
                    },
                    state,
                )?;
            } else {
                stateTerminate(
                    labelFin,
                    MidCode::Terminator::GOTO {
                        next: if (!((caseLabels).is_empty())) {
                            (caseLabels).head().cloned()?
                        } else {
                            labelFin
                        },
                    },
                    state,
                )?;
            }
            labelFail = GenBlockId();
            labelFin2 = GenBlockId();
            labelOut = GenBlockId();
            if matchContinue {
                stateTerminate(
                    labelFin2,
                    MidCode::Terminator::POPJMP {
                        old_buf: muxOldBuf,
                        next: labelFin2,
                    },
                    state,
                )?;
            } else {
                stateTerminate(labelFin2, MidCode::Terminator::GOTO { next: labelFin2 }, state)?;
            }
            midvar = RValueToVar(
                MidCode::RValue::LITERALINTEGER {
                    value: ((cases).len() as i32) + 1,
                },
                state,
            )?;
            midvar2 = RValueToVar(
                MidCode::RValue::BINARYOP {
                    op: openmodelica_codegen_util::MidCode::BinaryOp::EQUAL,
                    lsrc: muxState,
                    rsrc: midvar.clone(),
                },
                state,
            )?;
            stateTerminate(
                labelFail,
                MidCode::Terminator::BRANCH {
                    condition: midvar2,
                    onTrue: labelFail,
                    onFalse: labelOut,
                },
                state,
            )?;
            stateTerminate(labelOut, openmodelica_codegen_util::MidCode::Terminator::LONGJMP, state)?;
            caseLabelIterator = caseLabels;
            while !((caseLabelIterator).is_empty()) {
                caseLabel = (caseLabelIterator).head().cloned()?;
                caseLabelIterator = (caseLabelIterator).rest()?;
                stateSetCurrentLabel(caseLabel, state);
                let () = (::match_deref::match_deref! { match &(cases.clone()) {
                    Deref @ metamodelica::ListNode::Nil => {
                        ()
                    },
                    Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns, body: daeBody, patternGuard, result: caseResult, .. }, tail: cases } => {
                        if matchContinue {
                            patternToMidCode(&(List::zip(inputsMidVar.clone(), patterns.clone())), labelMux, state)?;
                        } else {
                            patternToMidCode(&(List::zip(inputsMidVar.clone(), patterns.clone())), if (!((caseLabelIterator).is_empty())) {(caseLabelIterator).head().cloned()?} else {labelFail}, state)?;
                        }
                        let () = (::match_deref::match_deref! { match &(patternGuard.clone()) {
                    None => (),
                    Some(__esc_daeExp) => {
                        daeExp = (*__esc_daeExp).clone();
                        midvar = RValueToVar(ExpToMid(daeExp.clone(), state)?, state)?;
                        if matchContinue {
                            stateAddBailOnFalse(midvar.clone(), labelMux, state)?;
                        } else {
                            stateAddBailOnFalse(midvar.clone(), if (!((caseLabelIterator).is_empty())) {(caseLabelIterator).head().cloned()?} else {labelFail}, state)?;
                        }
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                        StmtsToMid(metamodelica::AsArg::as_arg(&daeBody), state)?;
                        let () = (::match_deref::match_deref! { match &((caseResult.clone(), &**outvars)) {
                    (Some(Deref @ DAE::Exp::TUPLE { PR: expList }), _) => {
                        for mut outvarDaeExp in &*listZip(outvars, metamodelica::AsArg::as_arg(&expList))? {
                            (outvar, daeExp) = outvarDaeExp.clone();
                            let () = (match outvar {
                    MidCode::OutVar::OUT_VAR { var: mut var } => {
                        stateAddStmt(MidCode::Stmt::ASSIGN { dest: var.clone(), src: ExpToMid(daeExp.clone(), state)? }, state)?;
                        ()
                    },
                    MidCode::OutVar::OUT_WILD { .. } => {
                        ()
                    },
                });
                        }
                        ()
                    },
                    (Some(__esc_daeExp @ Deref @ DAE::Exp::CALL { path: _, .. }), _) => {
                        daeExp = (*__esc_daeExp).clone();
                        CallToMid(metamodelica::AsArg::as_arg(&daeExp), outvars.clone(), state)?;
                        ()
                    },
                    (Some(__esc_daeExp @ Deref @ DAE::Exp::MATCHEXPRESSION { matchType: _, .. }), _) => {
                        daeExp = (*__esc_daeExp).clone();
                        MatchExpressionToMid(metamodelica::AsArg::as_arg(&daeExp), outvars, state)?;
                        ()
                    },
                    (Some(__esc_daeExp), Deref @ metamodelica::ListNode::Cons { head: MidCode::OutVar::OUT_VAR { var: __esc_midvar }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                        daeExp = (*__esc_daeExp).clone();
                        midvar = (*__esc_midvar).clone();
                        stateAddStmt(MidCode::Stmt::ASSIGN { dest: midvar.clone(), src: ExpToMid(daeExp.clone(), state)? }, state)?;
                        ()
                    },
                    (Some(__esc_daeExp), _) => {
                        daeExp = (*__esc_daeExp).clone();
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Match expression output to Mid conversion failed:\n")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(daeExp.clone(), 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
                        ()
                    },
                    (None, Deref @ metamodelica::ListNode::Nil) => {
                        ()
                    },
                    (None, _) => {
                        Error::addInternalError(literal!("case fail"), metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
                        return Err("fail")
                    },
                    _ => return Err("match: no arm matched"),
                } });
                        stateTerminate(labelOut, MidCode::Terminator::GOTO { next: labelFin }, state)?;
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn patternToMidCode(
    mut matches: &metamodelica::List<(MidCode::Var, metamodelica::Ref<DAE::Pattern>)>,
    mut labelNoMatch: i32,
    mut state: &State,
) -> Result<metamodelica::Array<metamodelica::List<MidCode::Stmt>>> {
    let mut assignBlock: metamodelica::Array<metamodelica::List<MidCode::Stmt>>;
    assignBlock = arrayCreate(1, metamodelica::nil());
    patternToMidCode2(state, matches, labelNoMatch, assignBlock.clone())?;
    for mut stmt in &*metamodelica::arrayGet(assignBlock.clone(), 1)?.reverse() {
        stateAddStmt(stmt.clone(), state)?;
    }
    Ok(assignBlock)
}

fn patternToMidCode2(
    mut state: &State,
    mut matches: &metamodelica::List<(MidCode::Var, metamodelica::Ref<DAE::Pattern>)>,
    mut labelNoMatch: i32,
    mut assignBlock: metamodelica::Array<metamodelica::List<MidCode::Stmt>>,
) -> Result<()> {
    let mut index: i32;
    let mut morePatterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
    let mut iterator: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
    let mut fields: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut knownSingleton: bool;
    let mut fieldNr: i32;
    let () = (::match_deref::match_deref! { match matches {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (_, Deref @ DAE::Pattern::PAT_WILD { .. }), tail: restMatches } => {
            patternToMidCode2(state, restMatches, labelNoMatch, assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_AS { id, ty: None, pat: pattern, .. }), tail: restMatches } => {
            let mut midvar: MidCode::Var;
            let mut ty: metamodelica::Ref<DAE::Type>;
            ty = RValueType(&(MidCode::RValue::VARIABLE { src: scrutinee.clone() }))?;
            midvar = MidCode::Var { name: id.clone(), ty: ty, volatile: false };
            metamodelica::arrayUpdate(assignBlock.clone(), 1, metamodelica::cons(MidCode::Stmt::ASSIGN { dest: midvar, src: MidCode::RValue::VARIABLE { src: scrutinee.clone() } }, metamodelica::arrayGet(assignBlock.clone(), 1)?))?;
            patternToMidCode2(state, &(metamodelica::cons((scrutinee.clone(), pattern.clone()), restMatches.clone())), labelNoMatch, assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_AS { id, ty: Some(ty), pat: pattern, .. }), tail: restMatches } => {
            let mut midvar: MidCode::Var;
            midvar = MidCode::Var { name: id.clone(), ty: ty.clone(), volatile: false };
            metamodelica::arrayUpdate(assignBlock.clone(), 1, metamodelica::cons(MidCode::Stmt::ASSIGN { dest: midvar, src: MidCode::RValue::UNARYOP { op: openmodelica_codegen_util::MidCode::UnaryOp::UNBOX, src: scrutinee.clone() } }, metamodelica::arrayGet(assignBlock.clone(), 1)?))?;
            patternToMidCode2(state, &(metamodelica::cons((scrutinee.clone(), pattern.clone()), restMatches.clone())), labelNoMatch, assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_CONSTANT { ty: optType, exp }), tail: restMatches } => {
            let mut ok: MidCode::Var;
            let mut scrutineeCompareVar: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut patCompareVar: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut bool: bool;
            let mut integer: i32;
            let mut real: metamodelica::Real;
            let mut string: ArcStr;
            let mut scrutinee = (*scrutinee).clone();
            let mut exp = (*exp).clone();
            exp = (match &*exp.clone() {
        DAE::Exp::SHARED_LITERAL { exp, .. } => exp.clone(),
        _ => exp.clone(),
    });
            scrutinee = (::match_deref::match_deref! { match &(optType.clone()) {
        None => scrutinee.clone(),
        Some(_) => RValueToVar(MidCode::RValue::UNARYOP { op: openmodelica_codegen_util::MidCode::UnaryOp::UNBOX, src: scrutinee.clone() }, state)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::BCONST { bool: __esc_bool } => {
            bool = (*__esc_bool).clone();
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: bool.clone() }, state)?;
            ()
        },
        Deref @ DAE::Exp::ICONST { integer: __esc_integer } => {
            integer = (*__esc_integer).clone();
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALINTEGER { value: integer.clone() }, state)?;
            ()
        },
        Deref @ DAE::Exp::RCONST { real: __esc_real } => {
            real = (*__esc_real).clone();
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALREAL { value: real.clone() }, state)?;
            ()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { index: __esc_integer, .. } => {
            integer = (*__esc_integer).clone();
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALINTEGER { value: integer.clone() }, state)?;
            ()
        },
        Deref @ DAE::Exp::LIST { valList: Deref @ metamodelica::ListNode::Nil } => {
            scrutineeCompareVar = RValueToVar(MidCode::RValue::ISCONS { src: scrutinee.clone() }, state)?;
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: false }, state)?;
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            scrutineeCompareVar = RValueToVar(MidCode::RValue::ISSOME { src: scrutinee.clone() }, state)?;
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: false }, state)?;
            ()
        },
        Deref @ DAE::Exp::SCONST { string: __esc_string } => {
            string = (*__esc_string).clone();
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALSTRING { value: string.clone() }, state)?;
            ()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE.Exp to Mid conversion failed for pattern constant. Exp:")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(exp.clone(), 0)?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ok = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state)?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: ok.clone(), src: MidCode::RValue::BINARYOP { op: openmodelica_codegen_util::MidCode::BinaryOp::EQUAL, lsrc: scrutineeCompareVar, rsrc: patCompareVar } }, state)?;
            stateAddBailOnFalse(ok, labelNoMatch, state)?;
            patternToMidCode2(state, restMatches, labelNoMatch, assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: __esc_morePatterns }), tail: restMatches } => {
            morePatterns = (*__esc_morePatterns).clone();
            let mut moreMatches: metamodelica::List<(MidCode::Var, metamodelica::Ref<DAE::Pattern>)>;
            let mut listTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut midvar: MidCode::Var;
            listTypes = (match &*scrutinee.ty.clone() {
        DAE::Type::T_METATUPLE { types: __esc_listTypes } => {
            listTypes = (*__esc_listTypes).clone();
            listTypes.clone()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Wrong type of midvar in tuple pattern: ")); __mm_s.push_str(&*DAEDump::daeTypeStr(&scrutinee.ty)?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
    });
            moreMatches = metamodelica::nil();
            iterator = morePatterns.clone();
            fieldNr = 0;
            while !((iterator).is_empty()) {
                midvar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: fieldNr, ty: (listTypes).head().cloned()? }, state)?;
                moreMatches = metamodelica::cons((midvar, (iterator).head().cloned()?), moreMatches);
                fieldNr = fieldNr + 1;
                iterator = (iterator).rest()?;
                listTypes = (listTypes).rest()?;
            }
            moreMatches = moreMatches.reverse();
            patternToMidCode2(state, &(listAppend(moreMatches, restMatches.clone())), labelNoMatch, assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_SOME { pat: pattern }), tail: restMatches } => {
            let mut ok: MidCode::Var;
            let mut midvar: MidCode::Var;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut scrutineeCompareVar: MidCode::Var;
            let mut patCompareVar: MidCode::Var;
            ok = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state)?;
            scrutineeCompareVar = RValueToVar(MidCode::RValue::ISSOME { src: scrutinee.clone() }, state)?;
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: true }, state)?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: ok.clone(), src: MidCode::RValue::BINARYOP { op: openmodelica_codegen_util::MidCode::BinaryOp::EQUAL, lsrc: scrutineeCompareVar, rsrc: patCompareVar } }, state)?;
            stateAddBailOnFalse(ok, labelNoMatch, state)?;
            ty = (match &*scrutinee.ty.clone() {
        DAE::Type::T_METAOPTION { ty: __esc_ty } => {
            ty = (*__esc_ty).clone();
            ty.clone()
        },
        _ => {
            Error::addInternalError(literal!("Wrong type of midvar in option pattern.\n"), metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
    });
            midvar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: 0, ty: ty }, state)?;
            patternToMidCode2(state, &(metamodelica::cons((midvar, pattern.clone()), restMatches.clone())), labelNoMatch, assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_CONS { head: headPattern, tail: restPattern }), tail: restMatches } => {
            let mut ok: MidCode::Var;
            let mut headVar: MidCode::Var;
            let mut restVar: MidCode::Var;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut scrutineeCompareVar: MidCode::Var;
            let mut patCompareVar: MidCode::Var;
            scrutineeCompareVar = RValueToVar(MidCode::RValue::ISCONS { src: scrutinee.clone() }, state)?;
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: true }, state)?;
            ok = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state)?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: ok.clone(), src: MidCode::RValue::BINARYOP { op: openmodelica_codegen_util::MidCode::BinaryOp::EQUAL, lsrc: scrutineeCompareVar, rsrc: patCompareVar } }, state)?;
            stateAddBailOnFalse(ok, labelNoMatch, state)?;
            ty = (::match_deref::match_deref! { match &(scrutinee.ty.clone()) {
        Deref @ DAE::Type::T_METALIST { ty: Deref @ DAE::Type::T_UNKNOWN { .. } } => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found list of unknown in cons pattern: ")); __mm_s.push_str(&*DAEDump::daeTypeStr(&scrutinee.ty)?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        Deref @ DAE::Type::T_METALIST { ty: __esc_ty } => {
            ty = (*__esc_ty).clone();
            ty.clone()
        },
        _ => {
            Error::addInternalError(literal!("Wrong type of midvar in option pattern.\n"), metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            headVar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: 0, ty: ty }, state)?;
            restVar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: 1, ty: scrutinee.ty.clone() }, state)?;
            patternToMidCode2(state, &(metamodelica::cons((headVar, headPattern.clone()), metamodelica::cons((restVar, restPattern.clone()), restMatches.clone()))), labelNoMatch, assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_CALL { name: _, index: __esc_index, patterns: __esc_morePatterns, fields: __esc_fields, typeVars: _, knownSingleton: __esc_knownSingleton }), tail: restMatches } => {
            index = (*__esc_index).clone();
            morePatterns = (*__esc_morePatterns).clone();
            fields = (*__esc_fields).clone();
            knownSingleton = (*__esc_knownSingleton).clone();
            let mut moreMatches: metamodelica::List<(MidCode::Var, metamodelica::Ref<DAE::Pattern>)>;
            let mut listTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut ok: MidCode::Var;
            let mut midvar: MidCode::Var;
            let mut scrutineeCompareVar: MidCode::Var;
            let mut patCompareVar: MidCode::Var;
            if !(knownSingleton.clone()) {
                ok = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state)?;
                scrutineeCompareVar = RValueToVar(MidCode::RValue::UNIONTYPEVARIANT { src: scrutinee.clone() }, state)?;
                patCompareVar = RValueToVar(MidCode::RValue::LITERALINTEGER { value: index.clone() }, state)?;
                stateAddStmt(MidCode::Stmt::ASSIGN { dest: ok.clone(), src: MidCode::RValue::BINARYOP { op: openmodelica_codegen_util::MidCode::BinaryOp::EQUAL, lsrc: scrutineeCompareVar, rsrc: patCompareVar } }, state)?;
                stateAddBailOnFalse(ok, labelNoMatch, state)?;
            }
            listTypes = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut v in (fields.clone()).into_iter().cloned() {
            let __x = v.ty.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            moreMatches = metamodelica::nil();
            iterator = morePatterns.clone();
            fieldNr = 1;
            while !((iterator).is_empty()) {
                midvar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: fieldNr, ty: (listTypes).head().cloned()? }, state)?;
                moreMatches = metamodelica::cons((midvar, (iterator).head().cloned()?), moreMatches);
                fieldNr = fieldNr + 1;
                iterator = (iterator).rest()?;
                listTypes = (listTypes).rest()?;
            }
            moreMatches = moreMatches.reverse();
            patternToMidCode2(state, &(listAppend(moreMatches, restMatches.clone())), labelNoMatch, assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (_, Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. }), tail: _ } => {
            Error::addInternalError(literal!("DAE.Pattern to Mid conversion failed. Unimplemented pattern: PAT_AS_FUNC_PTR.\n"), metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        Deref @ metamodelica::ListNode::Cons { head: (_, Deref @ DAE::Pattern::PAT_CALL_TUPLE { .. }), tail: _ } => {
            Error::addInternalError(literal!("DAE.Pattern to Mid conversion failed. Unimplemented pattern: PAT_CALL_TUPLE.\n"), metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        Deref @ metamodelica::ListNode::Cons { head: (_, Deref @ DAE::Pattern::PAT_CALL_NAMED { .. }), tail: _ } => {
            Error::addInternalError(literal!("DAE.Pattern to Mid conversion failed. Unimplemented pattern: PAT_CALL_NAMED.\n"), metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        _ => {
            Error::addInternalError(literal!("DAE.Pattern to Mid conversion failed\n"), metamodelica::sourceInfo!("MidCode/DAEToMid.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}
