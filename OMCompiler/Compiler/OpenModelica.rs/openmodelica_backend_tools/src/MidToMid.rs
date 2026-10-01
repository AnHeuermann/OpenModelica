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

use openmodelica_codegen_util::MidCode;
use openmodelica_util_datatypes_basic::List;

/*
Longjmps are not allowed to land in the same function.
This is handled in midtomid.
Handling it here allows other tranformations to
deal with goto instead of longjmp, which might enable
further transformation.

pushpopjmp possible.
can remove push-pop -jmp pairs if there is no possible longjmp in between.

Typechecking possible.
Useful for correctness of midcode transformations.

Normalisation possble. (AKA canonicalisation)
Probably essential to simplify other transformations.
Remove greater than comparisons and similar.

Inlining possible.
Important catalyst for other optimisations.

Common subexpression elimination possible.
But requires some data flow and side effect analysis.
Some SSA variables and purity marked functions perhaps.

*/
pub(crate) fn longJmpGoto(mut oldFunction: &MidCode::Function) -> Result<MidCode::Function> {
    let mut newFunction: MidCode::Function;
    let mut newBody: metamodelica::List<MidCode::Block>;
    let mut oldBody: metamodelica::List<MidCode::Block>;
    let mut newBlock: MidCode::Block;
    let mut oldBlock: MidCode::Block;
    let mut node: i32;
    let mut jump: i32;
    let mut jumps: metamodelica::List<i32>;
    let mut nodes_tmp: metamodelica::List<i32>;
    let mut checkedNodes: metamodelica::List<i32>;
    let mut tasks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut tasks_tmp: metamodelica::List<(metamodelica::List<i32>, i32)>;
    oldBody = oldFunction.body.clone();
    newBody = metamodelica::nil();
    checkedNodes = list![oldFunction.entryId.clone()];
    tasks = list![(metamodelica::nil(), oldFunction.entryId.clone())];
    while !((tasks).is_empty()) {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(tasks) {
            Deref @ metamodelica::ListNode::Cons { head: (__pa0, __pa1), tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        jumps = metamodelica::Own::own(__pa0);
        node = metamodelica::Own::own(__pa1);
        tasks = metamodelica::Own::own(__pa2);
        oldBlock = lookupId(oldBody.clone(), node)?;
        newBlock = oldBlock.clone();
        if isPushJmp(&oldBlock.terminator) {
            jumps = metamodelica::cons((getSuccessors(&oldBlock)?).head().cloned()?, jumps);
        } else if isLongJmp(&oldBlock.terminator) && !((jumps).is_empty()) {
            let __pa3 = ::match_deref::match_deref! { match &(jumps.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: _ } => __pa3.clone(),
                _ => return Err("pattern mismatch"),
            } };
            jump = metamodelica::Own::own(__pa3);
            newBlock = MidCode::Block {
                id: oldBlock.id.clone(),
                stmts: oldBlock.stmts.clone(),
                terminator: MidCode::Terminator::GOTO { next: jump },
            };
        } else if isPopJmp(&oldBlock.terminator) {
            let __pa4 = ::match_deref::match_deref! { match &(jumps) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa4 } => __pa4.clone(),
                _ => return Err("pattern mismatch"),
            } };
            jumps = metamodelica::Own::own(__pa4);
        }
        newBody = metamodelica::cons(newBlock, newBody);
        nodes_tmp = List::setDifference(getSuccessors(&oldBlock)?, &checkedNodes)?;
        checkedNodes = listAppend(nodes_tmp.clone(), checkedNodes);
        tasks_tmp = ({
            let mut __acc: metamodelica::List<(metamodelica::List<i32>, i32)> = metamodelica::nil();
            for mut node_tmp in (nodes_tmp).into_iter().cloned() {
                let __x = (jumps.clone(), node_tmp.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        tasks = listAppend(tasks_tmp, tasks);
    }
    newBody = newBody.reverse();
    newFunction = MidCode::Function {
        name: oldFunction.name.clone(),
        locals: oldFunction.locals.clone(),
        localBufs: oldFunction.localBufs.clone(),
        localBufPtrs: oldFunction.localBufPtrs.clone(),
        inputs: oldFunction.inputs.clone(),
        outputs: oldFunction.outputs.clone(),
        body: newBody,
        entryId: oldFunction.entryId.clone(),
        exitId: oldFunction.exitId.clone(),
    };
    Ok(newFunction)
}

pub(crate) fn lookupId(mut blocks: metamodelica::List<MidCode::Block>, mut id: i32) -> Result<MidCode::Block> {
    '__tco: loop {
        let mut blocks_local: metamodelica::List<MidCode::Block>;
        let mut block_local: MidCode::Block;
        ::match_deref::match_deref! { match &(blocks) {
            Deref @ metamodelica::ListNode::Cons { head: block_local, tail: _ } if (block_local.id.clone() == id) => return Ok(block_local.clone()),
            Deref @ metamodelica::ListNode::Cons { head: _, tail: __esc_blocks_local } => {
                blocks_local = (*__esc_blocks_local).clone();
                { (blocks, id) = (blocks_local.clone(), id); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getSuccessors(mut block_: &MidCode::Block) -> Result<metamodelica::List<i32>> {
    let mut neighbours: metamodelica::List<i32>;
    let mut l0: i32;
    let mut l1: i32;
    let mut switchList: metamodelica::List<(i32, i32)>;
    neighbours = (match block_.terminator.clone() {
        MidCode::Terminator::GOTO { next: mut __esc_l0 } => {
            l0 = __esc_l0.clone();
            list![l0]
        }
        MidCode::Terminator::BRANCH {
            condition: _,
            onTrue: mut __esc_l0,
            onFalse: mut __esc_l1,
        } => {
            l0 = __esc_l0.clone();
            l1 = __esc_l1.clone();
            list![l0, l1]
        }
        MidCode::Terminator::CALL {
            func: _,
            builtin: _,
            inputs: _,
            outputs: _,
            next: mut __esc_l0,
        } => {
            l0 = __esc_l0.clone();
            list![l0]
        }
        MidCode::Terminator::RETURN { .. } => metamodelica::nil(),
        MidCode::Terminator::SWITCH {
            condition: _,
            cases: ref __esc_switchList,
        } => {
            switchList = __esc_switchList.clone();
            ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut x in (switchList.clone()).into_iter().cloned() {
                    let __x = tupleSnd(x.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        MidCode::Terminator::LONGJMP { .. } => metamodelica::nil(),
        MidCode::Terminator::PUSHJMP {
            old_buf: _,
            new_buf: _,
            next: mut __esc_l0,
        } => {
            l0 = __esc_l0.clone();
            list![l0]
        }
        MidCode::Terminator::POPJMP {
            old_buf: _,
            next: mut __esc_l0,
        } => {
            l0 = __esc_l0.clone();
            list![l0]
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(neighbours)
}

fn tupleSnd(mut t: (i32, i32)) -> i32 {
    let mut i: i32;
    (_, i) = t;
    i
}

fn isLongJmp(mut t: &MidCode::Terminator) -> bool {
    let mut b: bool;
    b = (match t.clone() {
        MidCode::Terminator::LONGJMP { .. } => true,
        _ => false,
    });
    b
}

fn isPushJmp(mut t: &MidCode::Terminator) -> bool {
    let mut b: bool;
    b = (match t.clone() {
        MidCode::Terminator::PUSHJMP {
            old_buf: _,
            new_buf: _,
            next: _,
        } => true,
        _ => false,
    });
    b
}

fn isPopJmp(mut t: &MidCode::Terminator) -> bool {
    let mut b: bool;
    b = (match t.clone() {
        MidCode::Terminator::POPJMP { old_buf: _, next: _ } => true,
        _ => false,
    });
    b
}
