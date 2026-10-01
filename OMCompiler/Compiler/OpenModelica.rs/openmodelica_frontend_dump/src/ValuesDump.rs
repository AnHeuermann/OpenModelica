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
use crate::Dump;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub fn valString(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut handle: i32;
    if Flags::getConfigEnum(Flags::INTERACTIVE_DUMP_FORMAT.clone())? == Flags::IDUMP_JSON.clone() {
        outString = valStringJSON(inValue)?;
        return Ok(outString);
    }
    handle = Print::saveAndClearBuf()?;
    valString2(inValue)?;
    outString = Print::getString()?;
    Print::restoreBuf(handle)?;
    Ok(outString)
}

pub(crate) fn valStringJSON(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut handle: i32;
    handle = Print::saveAndClearBuf()?;
    valJSON(inValue)?;
    outString = Print::getString()?;
    Print::restoreBuf(handle)?;
    Ok(outString)
}

fn valJSON(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inValue {
        Deref @ Values::Value::INTEGER { integer: n } => {
            Print::printBuf(intString(n.clone()))?;
            ()
        },
        Deref @ Values::Value::REAL { real: x } => {
            Print::printBuf(realString(x.clone()))?;
            ()
        },
        Deref @ Values::Value::BOOL { boolean: true } => {
            Print::printBuf(literal!("true"))?;
            ()
        },
        Deref @ Values::Value::BOOL { boolean: false } => {
            Print::printBuf(literal!("false"))?;
            ()
        },
        Deref @ Values::Value::STRING { string: s } => {
            printJSONString(s.clone())?;
            ()
        },
        Deref @ Values::Value::ENUM_LITERAL { name: p, .. } => {
            printJSONString(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        },
        Deref @ Values::Value::ARRAY { valueLst: xs, .. } => {
            valJSONArray(xs)?;
            ()
        },
        Deref @ Values::Value::LIST { valueLst: xs } => {
            valJSONArray(xs)?;
            ()
        },
        Deref @ Values::Value::META_ARRAY { valueLst: xs } => {
            valJSONArray(xs)?;
            ()
        },
        Deref @ Values::Value::TUPLE { valueLst: xs } => {
            valJSONArray(xs)?;
            ()
        },
        Deref @ Values::Value::META_TUPLE { valueLst: xs } => {
            valJSONArray(xs)?;
            ()
        },
        Deref @ Values::Value::OPTION { some: Some(v) } => {
            valJSON(metamodelica::AsArg::as_arg(&v))?;
            ()
        },
        Deref @ Values::Value::OPTION { some: None } => {
            Print::printBuf(literal!("null"))?;
            ()
        },
        Deref @ Values::Value::RECORD { orderd: xs, comp: ids, .. } => {
            valJSONRecord(xs, ids.clone())?;
            ()
        },
        _ => {
            Print::printBuf(literal!("null"))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn valJSONArray(mut values: &metamodelica::List<metamodelica::Ref<Values::Value>>) -> Result<()> {
    let mut first: bool = true;
    Print::printBuf(literal!("["))?;
    for mut v in &**values {
        if !(first) {
            Print::printBuf(literal!(","))?;
        }
        first = false;
        valJSON(metamodelica::AsArg::as_arg(&v))?;
    }
    Print::printBuf(literal!("]"))?;
    Ok(())
}

fn valJSONRecord(
    mut values: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut names: metamodelica::List<ArcStr>,
) -> Result<()> {
    let mut first: bool = true;
    let mut rest: metamodelica::List<ArcStr> = names;
    let mut nm: ArcStr;
    Print::printBuf(literal!("{"))?;
    for mut v in &**values {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        nm = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if !(first) {
            Print::printBuf(literal!(","))?;
        }
        first = false;
        printJSONString(nm)?;
        Print::printBuf(literal!(":"))?;
        valJSON(metamodelica::AsArg::as_arg(&v))?;
    }
    Print::printBuf(literal!("}"))?;
    Ok(())
}

fn printJSONString(mut s: ArcStr) -> Result<()> {
    Print::printBuf(literal!("\""))?;
    Print::printBuf(System::stringReplace(
        System::escapedString(s, true),
        literal!("\t"),
        literal!("\\t"),
    )?)?;
    Print::printBuf(literal!("\""))?;
    Ok(())
}

pub fn valString2(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inValue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::INTEGER { integer: n } => {
                    let mut s: ArcStr;
                    s = intString(n.clone());
                    Print::printBuf(s.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::REAL { real: x } => {
                    let mut s: ArcStr;
                    s = realString(x.clone());
                    Print::printBuf(s.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::STRING { string: s } => {
                    Print::printBuf(literal!("\""))?;
                    Print::printBuf(System::escapedString(s.clone(), false))?;
                    Print::printBuf(literal!("\""))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::BOOL { boolean: false } => {
                    Print::printBuf(literal!("false"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::BOOL { boolean: true } => {
                    Print::printBuf(literal!("true"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ENUM_LITERAL { name: p, .. } => {
                    let mut s: ArcStr;
                    s = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                    Print::printBuf(s.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: vs, .. } => {
                    Print::printBuf(literal!("{"))?;
                    valListString(metamodelica::AsArg::as_arg(&vs))?;
                    Print::printBuf(literal!("}"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::TUPLE { valueLst: vs } => {
                    Print::printBuf(literal!("("))?;
                    valListString(metamodelica::AsArg::as_arg(&vs))?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_TUPLE { valueLst: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_TUPLE { valueLst: vs } => {
                    Print::printBuf(literal!("("))?;
                    valListString(metamodelica::AsArg::as_arg(&vs))?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { record_: Deref @ Absyn::Path::IDENT { name: Deref @ "SimulationResult" }, orderd: xs, comp: ids, .. } => {
                    let mut xs = (*xs).clone();
                    let mut ids = (*ids).clone();
                    Print::printBuf(literal!("record SimulationResult\n"))?;
                    (xs, ids) = filterSimulationResults(Flags::isSet(Flags::SHORT_OUTPUT.clone())?, xs.clone(), ids.clone(), metamodelica::nil(), metamodelica::nil())?;
                    valRecordString(xs.clone(), ids.clone())?;
                    Print::printBuf(literal!("end SimulationResult;"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { record_: recordPath, orderd: xs, comp: ids, .. } => {
                    let mut recordName: ArcStr;
                    recordName = AbsynUtil::pathStringNoQual(recordPath.clone(), literal!("."), false, false)?;
                    Print::printBuf({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("record ")); __mm_s.push_str(&*recordName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    valRecordString(xs.clone(), ids.clone())?;
                    Print::printBuf({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("end ")); __mm_s.push_str(&*recordName); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) })?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::OPTION { some: Some(r) } => {
                    Print::printBuf(literal!("SOME("))?;
                    valString2(metamodelica::AsArg::as_arg(&r))?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::OPTION { some: None } => {
                    Print::printBuf(literal!("NONE()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_BOX { value: r } => {
                    Print::printBuf(literal!("#("))?;
                    valString2(metamodelica::AsArg::as_arg(&r))?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } } => {
                    Print::printBuf(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } } => {
                    Print::printBuf(Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: c } => {
                    Print::printBuf(literal!("$Code("))?;
                    Print::printBuf(Dump::printCodeStr(c.clone())?)?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::LIST { valueLst: vs } => {
                    Print::printBuf(literal!("{"))?;
                    valListString(metamodelica::AsArg::as_arg(&vs))?;
                    Print::printBuf(literal!("}"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_ARRAY { valueLst: vs } => {
                    Print::printBuf(literal!("meta_array("))?;
                    valListString(metamodelica::AsArg::as_arg(&vs))?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ENUM_LITERAL { index: n, name: p } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!(" /* ENUM: ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" */")); ArcStr::from(__mm_s) };
                    Print::printBuf(s.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::NORETCALL { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::META_FAIL { .. } => {
                    Print::printBuf(literal!("fail()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::EMPTY { scope, name, tyStr, .. } => {
                    Print::printBuf({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("/* <EMPTY(scope: ")); __mm_s.push_str(&*scope); __mm_s.push_str(&*literal!(", name: ")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(", ty: ")); __mm_s.push_str(&*tyStr); __mm_s.push_str(&*literal!(")> */")); ArcStr::from(__mm_s) })?;
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
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("ValuesDump.valString2 failed")])?;
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

fn filterSimulationResults(
    mut filter: bool,
    mut inValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inIds: metamodelica::List<ArcStr>,
    mut valacc: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut idacc: metamodelica::List<ArcStr>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Values::Value>>,
    metamodelica::List<ArcStr>,
)> {
    let mut outValues: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut outIds: metamodelica::List<ArcStr>;
    (outValues, outIds) = (::match_deref::match_deref! { match &((filter, inValues.clone(), inIds.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            (valacc.reverse(), idacc.reverse())
        },
        (true, Deref @ metamodelica::ListNode::Cons { head: v, tail: vrest }, Deref @ metamodelica::ListNode::Cons { head: id @ Deref @ "messages", tail: idrest }) => {
            (outValues, outIds) = filterSimulationResults(filter, vrest.clone(), idrest.clone(), metamodelica::cons(v.clone(), valacc), metamodelica::cons(id.clone(), idacc))?;
            (outValues, outIds)
        },
        (true, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: vrest }, Deref @ metamodelica::ListNode::Cons { head: id @ Deref @ "resultFile", tail: idrest }) => {
            let mut r#str = (*r#str).clone();
            r#str = System::basename(r#str.clone());
            (outValues, outIds) = filterSimulationResults(filter, vrest.clone(), idrest.clone(), metamodelica::cons(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }), valacc), metamodelica::cons(id.clone(), idacc))?;
            (outValues, outIds)
        },
        (true, Deref @ metamodelica::ListNode::Cons { head: _, tail: vrest }, Deref @ metamodelica::ListNode::Cons { head: _, tail: idrest }) => {
            (outValues, outIds) = filterSimulationResults(filter, vrest.clone(), idrest.clone(), valacc, idacc)?;
            (outValues, outIds)
        },
        (false, _, _) => {
            (inValues, inIds)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outValues, outIds))
}

fn valRecordString(
    mut inValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inIds: metamodelica::List<ArcStr>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inValues, inIds);
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
                (Deref @ metamodelica::ListNode::Cons { head: x, tail: xs @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, Deref @ metamodelica::ListNode::Cons { head: id, tail: ids @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }) => {
                    Print::printBuf(literal!("    "))?;
                    Print::printBuf(id.clone())?;
                    Print::printBuf(literal!(" = "))?;
                    valString2(metamodelica::AsArg::as_arg(&x))?;
                    Print::printBuf(literal!(",\n"))?;
                    valRecordString(xs.clone(), ids.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: x, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: id, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Print::printBuf(literal!("    "))?;
                    Print::printBuf(id.clone())?;
                    Print::printBuf(literal!(" = "))?;
                    valString2(metamodelica::AsArg::as_arg(&x))?;
                    Print::printBuf(literal!("\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (xs, ids) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ValuesUtil.valRecordString failed:\nids: ")); __mm_s.push_str(&*stringDelimitList(ids.clone(), literal!(", "))); __mm_s.push_str(&*literal!("\nvals: ")); __mm_s.push_str(&*stringDelimitList(List::map(xs.clone(), &move |__a0: metamodelica::Ref<Values::Value>| valString(&__a0))?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
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

fn valListString(mut inValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inValueLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: v, tail: Deref @ metamodelica::ListNode::Nil } => {
            valString2(metamodelica::AsArg::as_arg(&v))?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: v, tail: vs } => {
            valString2(metamodelica::AsArg::as_arg(&v))?;
            Print::printBuf(literal!(", "))?;
            valListString(vs)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn printVal(mut v: &metamodelica::Ref<Values::Value>) -> Result<()> {
    let mut s: ArcStr;
    s = valString(v)?;
    Print::printBuf(s)?;
    Ok(())
}

pub fn printValStr(mut v: &metamodelica::Ref<Values::Value>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = valString(v)?;
    Ok(s)
}

pub fn unparseValues(mut inValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inValueLst {
        Deref @ metamodelica::ListNode::Cons { head: v, tail: vallst } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut r#str: ArcStr;
            s1 = unparseDescription(&(list![v.clone()]))?;
            s2 = unparseValueNumbers(&(list![v.clone()]))?;
            s3 = unparseValues(vallst)?;
            r#str = stringAppendList(list![s1, s2, literal!("\n"), s3]);
            r#str
        },
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn unparseValueNumbers(mut inValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inValueLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::TUPLE { valueLst: lst }, tail: xs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = unparseValueNumbers(metamodelica::AsArg::as_arg(&lst))?;
            s2 = unparseValueNumbers(xs)?;
            res = stringAppend(s1, s2);
            res
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::META_TUPLE { valueLst: lst }, tail: xs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = unparseValueNumbers(metamodelica::AsArg::as_arg(&lst))?;
            s2 = unparseValueNumbers(xs)?;
            res = stringAppend(s1, s2);
            res
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: lst, .. }, tail: xs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = unparseValueNumbers(metamodelica::AsArg::as_arg(&lst))?;
            s2 = unparseValueNumbers(xs)?;
            res = stringAppend(s1, s2);
            res
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: xs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            let mut istr: ArcStr;
            s1 = unparseValueNumbers(xs)?;
            istr = intString(i.clone());
            s2 = stringAppend(istr, literal!(" "));
            res = stringAppend(s2, s1);
            res
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r }, tail: xs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            let mut istr: ArcStr;
            s1 = unparseValueNumbers(xs)?;
            istr = realString(r.clone());
            s2 = stringAppend(istr, literal!(" "));
            res = stringAppend(s2, s1);
            res
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: sval }, tail: xs } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = unparseValueNumbers(xs)?;
            s2 = stringAppend(sval.clone(), literal!(" "));
            res = stringAppend(s2, s1);
            res
        },
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn unparseDescription(mut inValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inValueLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: xs } => {
            let mut s1: ArcStr;
            let mut r#str: ArcStr;
            s1 = unparseDescription(xs)?;
            r#str = stringAppend(literal!("# i!\n"), s1);
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { .. }, tail: xs } => {
            let mut s1: ArcStr;
            let mut r#str: ArcStr;
            s1 = unparseDescription(xs)?;
            r#str = stringAppend(literal!("# r!\n"), s1);
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: sval }, tail: xs } => {
            let mut s1: ArcStr;
            let mut r#str: ArcStr;
            let mut slenstr: ArcStr;
            let mut slen: i32;
            s1 = unparseDescription(xs)?;
            slen = ((sval).len() as i32);
            slenstr = intString(slen);
            r#str = stringAppendList(list![literal!("# s! 1 "), slenstr, literal!("\n"), s1]);
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vallst, .. }, tail: xs } => {
            let mut s1: ArcStr;
            let mut r#str: ArcStr;
            let mut s2: ArcStr;
            let mut s4: ArcStr;
            s1 = unparseDescription(xs)?;
            s2 = unparseArrayDescription(vallst.clone());
            s4 = stringAppend(s2, s1);
            r#str = stringAppend(s4, literal!(" \n"));
            r#str
        },
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn unparseArrayDescription(mut lst: metamodelica::List<metamodelica::Ref<Values::Value>>) -> ArcStr {
    let mut r#str: ArcStr;
    let mut pt: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut s3: ArcStr;
    let mut s4: ArcStr;
    let mut s5: ArcStr;
    let mut s6: ArcStr;
    let mut i1: i32;
    pt = unparsePrimType(lst.clone());
    s1 = stringAppend(literal!("# "), pt);
    s2 = stringAppend(s1, literal!("["));
    i1 = unparseNumDims(lst.clone(), 0);
    s3 = intString(i1);
    s4 = stringAppend(s2, s3);
    s5 = stringAppend(s4, literal!(" "));
    s6 = unparseDimSizes(lst);
    r#str = stringAppend(s5, s6);
    r#str
}

fn unparsePrimType(mut inValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>) -> ArcStr {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inValueLst) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: elts, .. }, tail: _ } => {
                let mut res: ArcStr;
                { inValueLst = elts.clone(); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ } => {
                return literal!("i")
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ } => {
                return literal!("r")
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { .. }, tail: _ } => {
                return literal!("s")
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { .. }, tail: _ } => {
                return literal!("b")
            },
            Deref @ metamodelica::ListNode::Nil => {
                return literal!("{}")
            },
            _ => {
                return literal!("error")
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn unparseNumDims(mut inValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>, mut inInteger: i32) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inValueLst) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: _ } => {
                { (inValueLst, inInteger) = (vals.clone(), inInteger + 1); continue '__tco; }
            },
            _ => {
                return inInteger + 1
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn unparseDimSizes(mut inValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inValueLst) {
        lst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: _ } => {
            let mut i1: i32;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut res: ArcStr;
            i1 = ((lst).len() as i32);
            s1 = intString(i1);
            s2 = stringAppend(s1, literal!(" "));
            s3 = unparseDimSizes(vals.clone());
            res = stringAppend(s2, s3);
            res
        },
        lst => {
            let mut len: i32;
            let mut res: ArcStr;
            len = ((lst).len() as i32);
            res = intString(len);
            res
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}
