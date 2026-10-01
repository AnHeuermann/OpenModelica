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

use crate::FNode;
use openmodelica_frontend_dump::FCore;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// public imports
// protected imports
pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = metamodelica::Ref<FCore::Node>;

pub type Ref = Mutable::Mutable<metamodelica::Ref<FCore::Node>>;

pub type Data = metamodelica::Ref<FCore::Data>;

pub type Visit = FCore::Visit;

pub type VAvlTree = metamodelica::Ref<FCore::VAvlTree>;

pub type Visited = FCore::Visited;

pub type AvlTree = metamodelica::Ref<FCore::VAvlTree>;

pub type AvlKey = i32;

pub type AvlValue = FCore::Visit;

pub type AvlTreeValue = FCore::VAvlTreeValue;

thread_local! { static __emptyVisited_TLS: FCore::Visited = FCore::Visited { tree: FCore::emptyVAvlTree().clone(), next: FCore::firstId.clone() }; }
pub(crate) fn emptyVisited() -> FCore::Visited {
    __emptyVisited_TLS.with(|__t| __t.clone())
}

pub(crate) fn new() -> Visited {
    let mut visited: Visited;
    visited = emptyVisited().clone();
    visited
}

pub(crate) fn reset(mut inVisited: &Visited) -> Visited {
    let mut visited: Visited;
    visited = new();
    visited
}

pub(crate) fn next(mut inVisited: Visited) -> (Visited, Next) {
    let mut outVisited: Visited;
    let mut next: Next;
    let mut v: VAvlTree;
    let mut n: Next;
    let FCore::V {
        tree: __pa0,
        next: __pa1,
    } = inVisited;
    v = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    next = n;
    n = FCore::next(n);
    outVisited = FCore::Visited { tree: v, next: n };
    (outVisited, next)
}

pub(crate) fn visited(mut inVisited: &Visited, mut inRef: Ref) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = inVisited.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let FCore::Visited { tree: ref a, .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            FNode::id(&(FNode::fromRef(inRef.clone())));
            avlTreeGet(&(a.clone()), FNode::id(&(FNode::fromRef(inRef.clone()))))?;
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

pub(crate) fn seq(mut v: Visit) -> Seq {
    let mut s: Seq;
    let FCore::VN { seq: __pa0, .. } = v;
    s = metamodelica::Own::own(__pa0);
    s
}

pub(crate) fn r#ref(mut v: Visit) -> Ref {
    let mut r: Ref;
    let FCore::VN { r#ref: __pa0, .. } = v;
    r = metamodelica::Own::own(__pa0);
    r
}

pub(crate) fn tree(mut v: Visited) -> AvlTree {
    let mut a: AvlTree;
    let FCore::V { tree: __pa0, .. } = v;
    a = metamodelica::Own::own(__pa0);
    a
}

pub(crate) fn visit(mut inVisited: Visited, mut inRef: Ref) -> Result<Visited> {
    let mut outVisited: Visited = <FCore::Visited as ::std::default::Default>::default();
    outVisited = 'mc: {
        let __mc_input = inVisited.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut v: Visit;
            FNode::id(&(FNode::fromRef(inRef.clone())));
            v = avlTreeGet(&(tree(inVisited.clone())), FNode::id(&(FNode::fromRef(inRef.clone()))))?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Already visited: "));
                __mm_s.push_str(&*FNode::toStr(&(FNode::fromRef(inRef.clone()))));
                __mm_s.push_str(&*literal!(" seq: "));
                __mm_s.push_str(&*intString(seq(v.clone())));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let FCore::Visited { tree: ref a, next: _ } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut s: Seq;
            let mut n: Next;
            let mut id: Id;
            let mut a = a.clone();
            let mut outVisited: FCore::Visited = outVisited.clone();
            id = FNode::id(&(FNode::fromRef(inRef.clone())));
            if '__try0: {
                unwrap_break_err!(avlTreeGet(&(tree(inVisited.clone())), id), '__try0);
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            let (FCore::V { next: __pa1, .. }, __pa2) = next(inVisited.clone());
            n = metamodelica::Own::own(__pa1);
            s = metamodelica::Own::own(__pa2);
            a = avlTreeAdd(
                &(a.clone()),
                id,
                FCore::Visit {
                    r#ref: inRef.clone(),
                    seq: s,
                },
            )?;
            outVisited = FCore::Visited {
                tree: a.clone(),
                next: n,
            };
            Ok((outVisited.clone(), outVisited.clone()))
        })() {
            outVisited = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVisited)
}

// ************************ AVL Tree implementation ***************************
// ************************ AVL Tree implementation ***************************
// ************************ AVL Tree implementation ***************************
// ************************ AVL Tree implementation ***************************
pub(crate) fn keyCompare(mut k1: AvlKey, mut k2: AvlKey) -> i32 {
    let mut i: i32;
    i = if (intGt(k1, k2)) {
        1
    } else {
        if (intLt(k1, k2)) { -1 } else { 0 }
    };
    i
}

pub(crate) fn keyStr(mut k: AvlKey) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = intString(k);
    r#str
}

pub(crate) fn valueStr(mut v: &AvlValue) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match v.clone() {
        FCore::Visit { seq: mut seq, .. } => intString(seq.clone()),
    });
    r#str
}

/* Generic Code below */
pub(crate) fn avlTreeNew() -> AvlTree {
    let mut tree: AvlTree;
    tree = FCore::emptyVAvlTree().clone();
    tree
}

pub(crate) fn avlTreeAdd(mut inAvlTree: &AvlTree, mut inKey: AvlKey, mut inValue: AvlValue) -> Result<AvlTree> {
    let mut outAvlTree: AvlTree;
    outAvlTree = (::match_deref::match_deref! { match inAvlTree {
        Deref @ FCore::VAvlTree { value: None, left: None, right: None, .. } => {
            let mut key = inKey;
            let mut value = inValue;
            metamodelica::Ref::new(FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: key, value: value }), height: 1, left: None, right: None })
        },
        Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: rkey, .. }), .. } => {
            let mut key = inKey;
            let mut value = inValue;
            balance(avlTreeAdd2(inAvlTree, keyCompare(key, rkey.clone()), key, value)?)?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Env.avlTreeAdd failed")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAvlTree)
}

pub(crate) fn avlTreeAdd2(
    mut inAvlTree: &AvlTree,
    mut keyComp: i32,
    mut inKey: AvlKey,
    mut inValue: AvlValue,
) -> Result<AvlTree> {
    let mut outAvlTree: AvlTree;
    outAvlTree = (::match_deref::match_deref! { match &((&**inAvlTree, keyComp)) {
        (Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: rkey, .. }), height: h, left, right }, 0) => {
            let mut value = inValue;
            metamodelica::Ref::new(FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: rkey.clone(), value: value }), height: h.clone(), left: left.clone(), right: right.clone() })
        },
        (Deref @ FCore::VAvlTree { value: oval, height: h, left, right }, 1) => {
            let mut key = inKey;
            let mut value = inValue;
            let mut t_1: AvlTree;
            let mut t: AvlTree;
            t = createEmptyAvlIfNone(right.clone());
            t_1 = avlTreeAdd(&t, key, value)?;
            metamodelica::Ref::new(FCore::VAvlTree { value: oval.clone(), height: h.clone(), left: left.clone(), right: Some(t_1) })
        },
        (Deref @ FCore::VAvlTree { value: oval, height: h, left, right }, (-1)) => {
            let mut key = inKey;
            let mut value = inValue;
            let mut t_1: AvlTree;
            let mut t: AvlTree;
            t = createEmptyAvlIfNone(left.clone());
            t_1 = avlTreeAdd(&t, key, value)?;
            metamodelica::Ref::new(FCore::VAvlTree { value: oval.clone(), height: h.clone(), left: Some(t_1), right: right.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAvlTree)
}

fn createEmptyAvlIfNone(mut t: Option<metamodelica::Ref<FCore::VAvlTree>>) -> AvlTree {
    let mut outT: AvlTree;
    outT = (::match_deref::match_deref! { match &(t) {
        None => metamodelica::Ref::new(FCore::VAvlTree { value: None, height: 0, left: None, right: None }),
        Some(__esc_outT) => {
            outT = (*__esc_outT).clone();
            outT.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outT
}

fn nodeValue(mut bt: &AvlTree) -> Result<AvlValue> {
    let mut v: AvlValue;
    v = (match &**bt {
        FCore::VAvlTree {
            value: Some(FCore::VAvlTreeValue { key: _, value: __esc_v }),
            ..
        } => {
            v = (*__esc_v).clone();
            v.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(v)
}

fn balance(mut inBt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = (::match_deref::match_deref! { match &(inBt) {
        bt => {
            let mut d: i32;
            let mut bt = (*bt).clone();
            d = differenceInHeight(metamodelica::AsArg::as_arg(&bt))?;
            bt = doBalance(d, bt.clone())?;
            bt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBt)
}

fn doBalance(mut difference: i32, mut inBt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = (match difference {
        (-1) => {
            let mut bt = inBt;
            computeHeight(&bt)?
        }
        0 => {
            let mut bt = inBt;
            computeHeight(&bt)?
        }
        1 => {
            let mut bt = inBt;
            computeHeight(&bt)?
        }
        _ => {
            let mut bt = inBt;
            bt = doBalance2(difference < 0, bt)?;
            bt
        }
    });
    Ok(outBt)
}

fn doBalance2(mut differenceIsNegative: bool, mut inBt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = (match differenceIsNegative {
        true => {
            let mut bt = inBt;
            bt = doBalance3(bt);
            bt = rotateLeft(bt)?;
            bt
        }
        false => {
            let mut bt = inBt;
            bt = doBalance4(bt);
            bt = rotateRight(bt)?;
            bt
        }
    });
    Ok(outBt)
}

fn doBalance3(mut inBt: AvlTree) -> AvlTree {
    let mut outBt: AvlTree;
    outBt = 'mc: {
        let __mc_input = inBt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                bt => {
                    let mut rr: AvlTree;
                    let mut bt = (*bt).clone();
                    let true = (differenceInHeight(&(getOption(rightNode(metamodelica::AsArg::as_arg(&bt)))?))? > 0) else { return Err("pattern mismatch") };
                    rr = rotateRight(getOption(rightNode(metamodelica::AsArg::as_arg(&bt)))?)?;
                    bt = setRight(metamodelica::AsArg::as_arg(&bt), Some(rr.clone()));
                    Ok(bt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inBt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBt
}

fn doBalance4(mut inBt: AvlTree) -> AvlTree {
    let mut outBt: AvlTree;
    outBt = 'mc: {
        let __mc_input = inBt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                bt => {
                    let mut rl: AvlTree;
                    let mut bt = (*bt).clone();
                    let true = (differenceInHeight(&(getOption(leftNode(metamodelica::AsArg::as_arg(&bt)))?))? < 0) else { return Err("pattern mismatch") };
                    rl = rotateLeft(getOption(leftNode(metamodelica::AsArg::as_arg(&bt)))?)?;
                    bt = setLeft(metamodelica::AsArg::as_arg(&bt), Some(rl.clone()));
                    Ok(bt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inBt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBt
}

fn setRight(mut node: &AvlTree, mut right: Option<metamodelica::Ref<FCore::VAvlTree>>) -> AvlTree {
    let mut outNode: AvlTree;
    outNode = (match &**node {
        FCore::VAvlTree {
            value,
            height,
            left: l,
            right: _,
        } => metamodelica::Ref::new(FCore::VAvlTree {
            value: value.clone(),
            height: height.clone(),
            left: l.clone(),
            right: right,
        }),
    });
    outNode
}

fn setLeft(mut node: &AvlTree, mut left: Option<metamodelica::Ref<FCore::VAvlTree>>) -> AvlTree {
    let mut outNode: AvlTree;
    outNode = (match &**node {
        FCore::VAvlTree {
            value,
            height,
            left: _,
            right: r,
        } => metamodelica::Ref::new(FCore::VAvlTree {
            value: value.clone(),
            height: height.clone(),
            left: left,
            right: r.clone(),
        }),
    });
    outNode
}

fn leftNode(mut node: &AvlTree) -> Option<metamodelica::Ref<FCore::VAvlTree>> {
    let mut subNode: Option<metamodelica::Ref<FCore::VAvlTree>>;
    subNode = (match &**node {
        FCore::VAvlTree {
            left: __esc_subNode, ..
        } => {
            subNode = (*__esc_subNode).clone();
            subNode.clone()
        }
    });
    subNode
}

fn rightNode(mut node: &AvlTree) -> Option<metamodelica::Ref<FCore::VAvlTree>> {
    let mut subNode: Option<metamodelica::Ref<FCore::VAvlTree>>;
    subNode = (match &**node {
        FCore::VAvlTree {
            right: __esc_subNode, ..
        } => {
            subNode = (*__esc_subNode).clone();
            subNode.clone()
        }
    });
    subNode
}

fn exchangeLeft(mut inNode: AvlTree, mut inParent: AvlTree) -> Result<AvlTree> {
    let mut outParent: AvlTree;
    outParent = (::match_deref::match_deref! { match &((inNode, inParent)) {
        (node, parent) => {
            let mut bt: AvlTree;
            let mut node = (*node).clone();
            let mut parent = (*parent).clone();
            parent = setRight(metamodelica::AsArg::as_arg(&parent), leftNode(metamodelica::AsArg::as_arg(&node)));
            parent = balance(parent.clone())?;
            node = setLeft(metamodelica::AsArg::as_arg(&node), Some(parent.clone()));
            bt = balance(node.clone())?;
            bt
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outParent)
}

fn exchangeRight(mut inNode: AvlTree, mut inParent: AvlTree) -> Result<AvlTree> {
    let mut outParent: AvlTree;
    outParent = (::match_deref::match_deref! { match &((inNode, inParent)) {
        (node, parent) => {
            let mut bt: AvlTree;
            let mut node = (*node).clone();
            let mut parent = (*parent).clone();
            parent = setLeft(metamodelica::AsArg::as_arg(&parent), rightNode(metamodelica::AsArg::as_arg(&node)));
            parent = balance(parent.clone())?;
            node = setRight(metamodelica::AsArg::as_arg(&node), Some(parent.clone()));
            bt = balance(node.clone())?;
            bt
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outParent)
}

fn rotateLeft(mut node: AvlTree) -> Result<AvlTree> {
    let mut outNode: AvlTree;
    outNode = exchangeLeft(getOption(rightNode(&node))?, node)?;
    Ok(outNode)
}

fn getOption<T: Clone + 'static + metamodelica::gc::MMTrace>(mut opt: Option<T>) -> Result<T> {
    let mut val: T;
    val = (match opt {
        Some(mut __esc_val) => {
            val = __esc_val.clone();
            val
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(val)
}

fn rotateRight(mut node: AvlTree) -> Result<AvlTree> {
    let mut outNode: AvlTree;
    outNode = exchangeRight(getOption(leftNode(&node))?, node)?;
    Ok(outNode)
}

fn differenceInHeight(mut node: &AvlTree) -> Result<i32> {
    let mut diff: i32;
    diff = (match &**node {
        FCore::VAvlTree { left: l, right: r, .. } => {
            let mut lh: i32;
            let mut rh: i32;
            lh = getHeight(l.clone())?;
            rh = getHeight(r.clone())?;
            lh - rh
        }
    });
    Ok(diff)
}

pub(crate) fn avlTreeGet(mut inAvlTree: &AvlTree, mut inKey: AvlKey) -> Result<AvlValue> {
    let mut outValue: AvlValue;
    outValue = (match &**inAvlTree {
        FCore::VAvlTree {
            value: Some(FCore::VAvlTreeValue { key: rkey, .. }),
            ..
        } => {
            let mut key = inKey;
            avlTreeGet2(inAvlTree, keyCompare(key, rkey.clone()), key)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outValue)
}

fn avlTreeGet2(mut inAvlTree: &AvlTree, mut keyComp: i32, mut inKey: AvlKey) -> Result<AvlValue> {
    let mut outValue: AvlValue;
    outValue = (::match_deref::match_deref! { match &((&**inAvlTree, keyComp)) {
        (Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { value: rval, .. }), .. }, 0) => {
            rval.clone()
        },
        (Deref @ FCore::VAvlTree { right: Some(right), .. }, 1) => {
            let mut key = inKey;
            avlTreeGet(metamodelica::AsArg::as_arg(&right), key)?
        },
        (Deref @ FCore::VAvlTree { left: Some(left), .. }, (-1)) => {
            let mut key = inKey;
            avlTreeGet(metamodelica::AsArg::as_arg(&left), key)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValue)
}

fn getOptionStr<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeAOption: Option<Type_a>,
    mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>,
) -> Result<ArcStr> {
    pub type FuncTypeType_aToString<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    outString = (match (inTypeAOption, inFuncTypeTypeAToString.clone()) {
        (Some(mut a), mut r) => {
            let mut r#str: ArcStr;
            r#str = r(a)?;
            r#str
        }
        (None, _) => {
            literal!("")
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn printAvlTreeStr(mut inAvlTree: &AvlTree) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inAvlTree {
        FCore::VAvlTree {
            value: Some(FCore::VAvlTreeValue { key: _, value: rval }),
            left: l,
            right: r,
            ..
        } => {
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut res: ArcStr;
            s2 = getOptionStr(
                l.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<FCore::VAvlTree>| printAvlTreeStr(&__a0))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<FCore::VAvlTree>) -> Result<ArcStr> + 'static,
                    >),
            )?;
            s3 = getOptionStr(
                r.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<FCore::VAvlTree>| printAvlTreeStr(&__a0))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<FCore::VAvlTree>) -> Result<ArcStr> + 'static,
                    >),
            )?;
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*valueStr(metamodelica::AsArg::as_arg(&rval)));
                __mm_s.push_str(&*literal!(",  "));
                __mm_s.push_str(&*if (stringEq(&s2, &(literal!("")))) {
                    literal!("")
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*s2);
                        __mm_s.push_str(&*literal!(", "));
                        ArcStr::from(__mm_s)
                    }
                });
                __mm_s.push_str(&*s3);
                ArcStr::from(__mm_s)
            };
            res
        }
        FCore::VAvlTree {
            value: None,
            left: l,
            right: r,
            ..
        } => {
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut res: ArcStr;
            s2 = getOptionStr(
                l.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<FCore::VAvlTree>| printAvlTreeStr(&__a0))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<FCore::VAvlTree>) -> Result<ArcStr> + 'static,
                    >),
            )?;
            s3 = getOptionStr(
                r.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<FCore::VAvlTree>| printAvlTreeStr(&__a0))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<FCore::VAvlTree>) -> Result<ArcStr> + 'static,
                    >),
            )?;
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*if (stringEq(&s2, &(literal!("")))) {
                    literal!("")
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*s2);
                        __mm_s.push_str(&*literal!(", "));
                        ArcStr::from(__mm_s)
                    }
                });
                __mm_s.push_str(&*s3);
                ArcStr::from(__mm_s)
            };
            res
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn computeHeight(mut bt: &AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = (match &**bt {
        FCore::VAvlTree {
            value: v @ Some(_),
            left: l,
            right: r,
            ..
        } => {
            let mut hl: i32;
            let mut hr: i32;
            let mut height: i32;
            hl = getHeight(l.clone())?;
            hr = getHeight(r.clone())?;
            height = intMax(hl, hr) + 1;
            metamodelica::Ref::new(FCore::VAvlTree {
                value: v.clone(),
                height: height,
                left: l.clone(),
                right: r.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outBt)
}

fn getHeight(mut bt: Option<metamodelica::Ref<FCore::VAvlTree>>) -> Result<i32> {
    let mut height: i32;
    height = (::match_deref::match_deref! { match &(bt) {
        None => 0,
        Some(Deref @ FCore::VAvlTree { height: __esc_height, .. }) => {
            height = (*__esc_height).clone();
            height.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(height)
}

pub(crate) fn printAvlTreeStrPP(mut inTree: AvlTree) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = printAvlTreeStrPP2(Some(inTree), &(literal!("")))?;
    Ok(outString)
}

fn printAvlTreeStrPP2(mut inTree: Option<metamodelica::Ref<FCore::VAvlTree>>, mut inIndent: &ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inTree) {
        None => {
            literal!("")
        },
        Some(Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: rkey, .. }), left: l, right: r, .. }) => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            let mut indent: ArcStr;
            indent = { let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) };
            s1 = printAvlTreeStrPP2(l.clone(), &indent)?;
            s2 = printAvlTreeStrPP2(r.clone(), &indent)?;
            res = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*inIndent); __mm_s.push_str(&*keyStr(rkey.clone())); __mm_s.push_str(&*s1); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) };
            res
        },
        Some(Deref @ FCore::VAvlTree { value: None, left: l, right: r, .. }) => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            let mut indent: ArcStr;
            indent = { let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) };
            s1 = printAvlTreeStrPP2(l.clone(), &indent)?;
            s2 = printAvlTreeStrPP2(r.clone(), &indent)?;
            res = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*s1); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) };
            res
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub(crate) fn avlTreeReplace(mut inAvlTree: &AvlTree, mut inKey: AvlKey, mut inValue: AvlValue) -> Result<AvlTree> {
    let mut outAvlTree: AvlTree;
    outAvlTree = (match &**inAvlTree {
        FCore::VAvlTree {
            value: Some(FCore::VAvlTreeValue { key: rkey, .. }),
            ..
        } => {
            let mut key = inKey;
            let mut value = inValue;
            avlTreeReplace2(inAvlTree, keyCompare(key, rkey.clone()), key, value)?
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("FVisit.avlTreeReplace"));
                    __mm_s.push_str(&*literal!(" failed"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(outAvlTree)
}

fn avlTreeReplace2(
    mut inAvlTree: &AvlTree,
    mut inKeyComp: i32,
    mut inKey: AvlKey,
    mut inValue: AvlValue,
) -> Result<AvlTree> {
    let mut outAvlTree: AvlTree;
    outAvlTree = (::match_deref::match_deref! { match &((&**inAvlTree, inKeyComp)) {
        (Deref @ FCore::VAvlTree { value: Some(_), height: h, left, right }, 0) => {
            let mut key = inKey;
            let mut value = inValue;
            metamodelica::Ref::new(FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: key, value: value }), height: h.clone(), left: left.clone(), right: right.clone() })
        },
        (Deref @ FCore::VAvlTree { value: oval, height: h, left, right }, 1) => {
            let mut key = inKey;
            let mut value = inValue;
            let mut t: AvlTree;
            t = createEmptyAvlIfNone(right.clone());
            t = avlTreeReplace(&t, key, value)?;
            metamodelica::Ref::new(FCore::VAvlTree { value: oval.clone(), height: h.clone(), left: left.clone(), right: Some(t) })
        },
        (Deref @ FCore::VAvlTree { value: oval, height: h, left, right }, (-1)) => {
            let mut key = inKey;
            let mut value = inValue;
            let mut t: AvlTree;
            t = createEmptyAvlIfNone(left.clone());
            t = avlTreeReplace(&t, key, value)?;
            metamodelica::Ref::new(FCore::VAvlTree { value: oval.clone(), height: h.clone(), left: Some(t), right: right.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAvlTree)
}

pub(crate) fn getAvlTreeValues(
    mut tree: metamodelica::List<Option<metamodelica::Ref<FCore::VAvlTree>>>,
    mut acc: metamodelica::List<FCore::VAvlTreeValue>,
) -> Result<metamodelica::List<FCore::VAvlTreeValue>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(tree) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(acc)
            },
            Deref @ metamodelica::ListNode::Cons { head: Some(Deref @ FCore::VAvlTree { value, left, right, .. }), tail: rest } => {
                { (tree, acc) = (metamodelica::cons(left.clone(), metamodelica::cons(right.clone(), rest.clone())), List::consOption(value.clone(), acc)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: None, tail: rest } => {
                { (tree, acc) = (rest.clone(), acc); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn getAvlValue(mut inValue: &AvlTreeValue) -> AvlValue {
    let mut res: AvlValue;
    res = (match inValue.clone() {
        FCore::VAvlTreeValue {
            value: mut __esc_res, ..
        } => {
            res = __esc_res.clone();
            res
        }
    });
    res
}

// ************************ END AVL Tree implementation ***************************
// ************************ END AVL Tree implementation ***************************
// ************************ END AVL Tree implementation ***************************
// ************************ END AVL Tree implementation ***************************
