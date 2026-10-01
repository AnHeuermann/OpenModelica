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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::Graphviz;

pub fn dump(mut p: &Absyn::Program) -> Result<()> {
    let mut r: metamodelica::Ref<Graphviz::Node>;
    r = buildGraphviz(p)?;
    Graphviz::dump(&r)?;
    Ok(())
}

fn buildGraphviz(mut inProgram: &Absyn::Program) -> Result<metamodelica::Ref<Graphviz::Node>> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = (match inProgram.clone() {
        Absyn::Program { classes: ref cs, .. } => {
            let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            nl = printClasses(metamodelica::AsArg::as_arg(&cs))?;
            metamodelica::Ref::new(Graphviz::Node::NODE {
                type_: literal!("ROOT"),
                attributes: metamodelica::nil(),
                children: nl,
            })
        }
    });
    Ok(outNode)
}

fn printClasses(
    mut inAbsynClassLst: &metamodelica::List<metamodelica::Ref<Absyn::Class>>,
) -> Result<metamodelica::List<metamodelica::Ref<Graphviz::Node>>> {
    let mut outNodeLst: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
    outNodeLst = (::match_deref::match_deref! { match inAbsynClassLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: cs } => {
            let mut node: metamodelica::Ref<Graphviz::Node>;
            let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            node = printClass(metamodelica::AsArg::as_arg(&c))?;
            nl = printClasses(cs)?;
            metamodelica::cons(node, nl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNodeLst)
}

fn printClass(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Graphviz::Node>> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { restriction: r, body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut rs: ArcStr;
            let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            rs = AbsynUtil::restrString(r);
            nl = printParts(metamodelica::AsArg::as_arg(&parts));
            metamodelica::Ref::new(Graphviz::Node::NODE { type_: rs, attributes: metamodelica::nil(), children: nl })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outNode)
}

fn printParts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Graphviz::Node>> {
    let mut outNodeLst: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
    outNodeLst = (::match_deref::match_deref! { match inAbsynClassPartLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: cs } => {
            let mut node: metamodelica::Ref<Graphviz::Node>;
            let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            node = printClassPart(metamodelica::AsArg::as_arg(&c));
            nl = printParts(cs);
            metamodelica::cons(node, nl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outNodeLst
}

fn printClassPart(mut inClassPart: &metamodelica::Ref<Absyn::ClassPart>) -> metamodelica::Ref<Graphviz::Node> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = &**inClassPart;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::PUBLIC { contents: el } => {
                    let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    nl = printElementitems(metamodelica::AsArg::as_arg(&el))?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("PUBLIC"), attributes: metamodelica::nil(), children: nl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::PROTECTED { contents: el } => {
                    let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    nl = printElementitems(metamodelica::AsArg::as_arg(&el))?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("PROTECTED"), attributes: metamodelica::nil(), children: nl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs } => {
                    let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    nl = printEquations(metamodelica::AsArg::as_arg(&eqs))?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("EQUATIONS"), attributes: metamodelica::nil(), children: nl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::ALGORITHMS { contents: als } => {
                    let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    nl = printAlgorithms(metamodelica::AsArg::as_arg(&als));
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("ALGORITHMS"), attributes: metamodelica::nil(), children: nl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!(" DumpGraphViz.printClassPart PART_ERROR"), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outNode
}

fn printElementitems(
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Graphviz::Node>>> {
    let mut outNodeLst: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
    outNodeLst = (::match_deref::match_deref! { match inAbsynElementItemLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: e }, tail: el } => {
            let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            let mut node: metamodelica::Ref<Graphviz::Node>;
            node = printElement(metamodelica::AsArg::as_arg(&e))?;
            nl = printElementitems(el)?;
            metamodelica::cons(node, nl)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outNodeLst)
}

fn makeBoolAttr(mut r#str: ArcStr, mut flag: bool) -> Graphviz::Attribute {
    let mut outAttribute: Graphviz::Attribute;
    outAttribute = Graphviz::Attribute {
        name: r#str,
        value: boolString(flag),
    };
    outAttribute
}

fn makeLeaf(mut r#str: ArcStr, mut al: metamodelica::List<Graphviz::Attribute>) -> metamodelica::Ref<Graphviz::Node> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = metamodelica::Ref::new(Graphviz::Node::NODE {
        type_: r#str,
        attributes: al,
        children: metamodelica::nil(),
    });
    outNode
}

fn printElement(mut inElement: &metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Graphviz::Node>> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = (match &**inElement {
        Absyn::Element::ELEMENT {
            finalPrefix,
            specification: spec,
            ..
        } => {
            let mut fa: Graphviz::Attribute;
            let mut elsp: metamodelica::Ref<Graphviz::Node>;
            fa = makeBoolAttr(literal!("final"), finalPrefix.clone());
            elsp = printElementspec(spec);
            metamodelica::Ref::new(Graphviz::Node::NODE {
                type_: literal!("ELEMENT"),
                attributes: list![fa],
                children: list![elsp],
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outNode)
}

fn printPath(mut p: metamodelica::Ref<Absyn::Path>) -> Result<metamodelica::Ref<Graphviz::Node>> {
    let mut pn: metamodelica::Ref<Graphviz::Node>;
    let mut s: ArcStr;
    s = AbsynUtil::pathString(p, literal!("."), true, false)?;
    pn = makeLeaf(s, metamodelica::nil());
    Ok(pn)
}

fn printElementspec(mut inElementSpec: &metamodelica::Ref<Absyn::ElementSpec>) -> metamodelica::Ref<Graphviz::Node> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = &**inElementSpec;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: repl, class_: cl } => {
                    let mut ra: Graphviz::Attribute;
                    printClass(metamodelica::AsArg::as_arg(&cl))?;
                    ra = makeBoolAttr(literal!("replaceable"), repl.clone());
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("CLASSDEF"), attributes: list![ra.clone()], children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::EXTENDS { path: p, .. } => {
                    let mut en: metamodelica::Ref<Graphviz::Node>;
                    en = printPath(p.clone())?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("EXTENDS"), attributes: metamodelica::nil(), children: list![en.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: tspec, components: cs, .. } => {
                    let mut pn: metamodelica::Ref<Graphviz::Node>;
                    let mut cns: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    let mut s: ArcStr;
                    s = Dump::unparseTypeSpec(tspec.clone())?;
                    pn = makeLeaf(s.clone(), metamodelica::nil());
                    cns = printComponents(metamodelica::AsArg::as_arg(&cs))?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("COMPONENTS"), attributes: metamodelica::nil(), children: metamodelica::cons(pn.clone(), cns.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!(" DumpGraphviz.printElementspec ELSPEC_ERROR"), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outNode
}

fn printComponents(
    mut inAbsynComponentItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Graphviz::Node>>> {
    let mut outNodeLst: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
    outNodeLst = (::match_deref::match_deref! { match inAbsynComponentItemLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: cs } => {
            let mut n: metamodelica::Ref<Graphviz::Node>;
            let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            n = printComponentitem(metamodelica::AsArg::as_arg(&c))?;
            nl = printComponents(cs)?;
            metamodelica::cons(n, nl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNodeLst)
}

fn printComponentitem(
    mut inComponentItem: &metamodelica::Ref<Absyn::ComponentItem>,
) -> Result<metamodelica::Ref<Graphviz::Node>> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = (match &**inComponentItem {
        Absyn::ComponentItem {
            component: Absyn::Component { name: n, .. },
            ..
        } => {
            let mut nn: metamodelica::Ref<Graphviz::Node>;
            nn = metamodelica::Ref::new(Graphviz::Node::NODE {
                type_: n.clone(),
                attributes: metamodelica::nil(),
                children: metamodelica::nil(),
            });
            metamodelica::Ref::new(Graphviz::Node::LNODE {
                type_: literal!("COMPONENT"),
                labelLst: list![n.clone()],
                attributes: metamodelica::nil(),
                children: list![nn],
            })
        }
    });
    Ok(outNode)
}

fn printEquations(
    mut inAbsynEquationItemLst: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Graphviz::Node>>> {
    let mut outNodeLst: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
    outNodeLst = (::match_deref::match_deref! { match inAbsynEquationItemLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq, .. }, tail: el } => {
            let mut node: metamodelica::Ref<Graphviz::Node>;
            let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            node = printEquation(metamodelica::AsArg::as_arg(&eq));
            nl = printEquations(el)?;
            metamodelica::cons(node, nl)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outNodeLst)
}

fn printEquation(mut inEquation: &metamodelica::Ref<Absyn::Equation>) -> metamodelica::Ref<Graphviz::Node> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = &**inEquation;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: e1, rightSide: e2 } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    s1 = Dump::printExpStr(e1.clone())?;
                    s2 = Dump::printExpStr(e2.clone())?;
                    s = stringAppend(s1.clone(), literal!(" = "));
                    s_1 = stringAppend(s.clone(), s2.clone());
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("EQ_EQUALS"), labelLst: list![s_1.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_PDE { leftSide: e1, rightSide: e2, domain: c1 } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    s1 = Dump::printExpStr(e1.clone())?;
                    s2 = Dump::printExpStr(e2.clone())?;
                    s3 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?;
                    s = stringAppend(s1.clone(), literal!(" = "));
                    s_1 = stringAppend(s.clone(), s2.clone());
                    s_1 = stringAppend(s_1.clone(), literal!(" indomain "));
                    s_1 = stringAppend(s_1.clone(), s3.clone());
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("EQ_PDE"), labelLst: list![s_1.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_CONNECT { connector1: c1, connector2: c2 } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s1 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?;
                    s2 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?;
                    s = stringAppend(literal!("connect("), s1.clone());
                    s_1 = stringAppend(s.clone(), s2.clone());
                    s_2 = stringAppend(s_1.clone(), literal!(")"));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("EQ_CONNECT"), labelLst: list![s_2.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_FOR { iterators, forEquations: eqs } => {
                    let mut es: ArcStr;
                    let mut eqn: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    eqn = printEquations(metamodelica::AsArg::as_arg(&eqs))?;
                    es = Dump::printIteratorsStr(metamodelica::AsArg::as_arg(&iterators));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("EQ_FOR"), labelLst: list![es.clone()], attributes: metamodelica::nil(), children: eqn.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("EQ_ERROR"), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outNode
}

fn printAlgorithms(
    mut inAbsynAlgorithmItemLst: &metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
) -> metamodelica::List<metamodelica::Ref<Graphviz::Node>> {
    let mut outNodeLst: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
    outNodeLst = (::match_deref::match_deref! { match inAbsynAlgorithmItemLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: el } => {
            let mut node: metamodelica::Ref<Graphviz::Node>;
            let mut nl: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            node = printAlgorithmitem(metamodelica::AsArg::as_arg(&e));
            nl = printAlgorithms(el);
            metamodelica::cons(node, nl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outNodeLst
}

fn printAlgorithmitem(
    mut inAlgorithmItem: &metamodelica::Ref<Absyn::AlgorithmItem>,
) -> metamodelica::Ref<Graphviz::Node> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = (match &**inAlgorithmItem {
        Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg, .. } => {
            let mut node: metamodelica::Ref<Graphviz::Node>;
            node = printAlgorithm(alg);
            node
        }
        _ => metamodelica::Ref::new(Graphviz::Node::NODE {
            type_: literal!("ALG_ERROR"),
            attributes: metamodelica::nil(),
            children: metamodelica::nil(),
        }),
    });
    outNode
}

fn printAlgorithm(mut inAlgorithm: &metamodelica::Ref<Absyn::Algorithm>) -> metamodelica::Ref<Graphviz::Node> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = (match &**inAlgorithm {
        Absyn::Algorithm::ALG_ASSIGN { .. } => metamodelica::Ref::new(Graphviz::Node::NODE {
            type_: literal!("ALG_ASSIGN"),
            attributes: metamodelica::nil(),
            children: metamodelica::nil(),
        }),
        _ => metamodelica::Ref::new(Graphviz::Node::NODE {
            type_: literal!(" DumpGraphviz.printAlgorithm ALG_ERROR"),
            attributes: metamodelica::nil(),
            children: metamodelica::nil(),
        }),
    });
    outNode
}

fn variabilitySymbol(mut inVariability: Absyn::Variability) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVariability {
        Absyn::Variability::VAR { .. } => literal!(""),
        Absyn::Variability::DISCRETE { .. } => literal!("DISCRETE"),
        Absyn::Variability::PARAM { .. } => literal!("PARAM"),
        Absyn::Variability::CONST { .. } => literal!("CONST"),
    });
    outString
}

fn directionSymbol(mut inDirection: Absyn::Direction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inDirection {
        Absyn::Direction::BIDIR { .. } => literal!(""),
        Absyn::Direction::INPUT { .. } => literal!("INPUT"),
        Absyn::Direction::OUTPUT { .. } => literal!("OUTPUT"),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}
