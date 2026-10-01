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

use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFRecord as Record;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_util::Error;

pub(crate) fn instConstructor(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut recordNode: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut recordNode: metamodelica::Ref<InstNode::InstNode> = recordNode;
    let mut ctor_ref: metamodelica::Ref<ComponentRef::NFComponentRef> = crate::NFComponentRef::interned_EMPTY();
    let mut ctor_path: metamodelica::Ref<Absyn::Path>;
    let mut ctor_overloaded: bool;
    let mut ctor_node: metamodelica::Ref<InstNode::InstNode>;
    match '__try0: {
        ctor_ref = unwrap_break_err!(Function::lookupFunctionSimple(literal!("'constructor'"), recordNode.clone(), context), '__try0);
        ctor_overloaded = true;
        Ok::<_, &'static str>((ctor_overloaded.clone(),))
    } {
        Ok((__try0_o0,)) => {
            ctor_overloaded = __try0_o0;
        }
        Err(_) => {
            ctor_overloaded = false;
        }
    }
    if ctor_overloaded {
        (_, ctor_node, _) = Function::instFunctionRef(ctor_ref, context, info.clone())?;
        ctor_path = InstNode::fullPath(ctor_node.clone(), false)?;
        for mut f in &*Function::getCachedFuncs(ctor_node)? {
            checkOperatorConstructorOutput(
                metamodelica::AsArg::as_arg(&f),
                Class::lastBaseClass(recordNode.clone())?,
                ctor_path.clone(),
                &info,
            )?;
            recordNode = InstNode::cacheAddFunc(recordNode, f.clone(), false)?;
        }
    }
    recordNode = Record::instDefaultConstructor(path, recordNode, context, &info)?;
    Ok(recordNode)
}

pub(crate) fn instOperatorFunctions(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut mclss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut allfuncs: metamodelica::List<metamodelica::Ref<Function::Function>> = metamodelica::nil();
    let mut funcs: metamodelica::List<metamodelica::Ref<Function::Function>>;
    checkOperatorRestrictions(node.clone())?;
    tree = Class::classTree(InstNode::getClass(node.clone())?)?;
    let () = (match &*tree {
        ClassTree::FLAT_TREE {
            classes: __esc_mclss, ..
        } => {
            mclss = (*__esc_mclss).clone();
            let __range0 = mclss.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut op in __range0 {
                Function::instFunctionNode(op.clone(), context, info.clone())?;
                funcs = Function::getCachedFuncs(op)?;
                allfuncs = listAppend(funcs, allfuncs);
            }
            for mut f in &*allfuncs {
                node = InstNode::cacheAddFunc(node, f.clone(), false)?;
            }
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFOperatorOverloading.instOperatorFunctions"));
                    __mm_s.push_str(&*literal!(" got non-instantiated function"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFOperatorOverloading.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(node)
}

pub(crate) fn checkOperatorRestrictions(mut operatorNode: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
    if !(SCodeUtil::isElementEncapsulated(&(InstNode::definition(operatorNode.clone())?))) {
        Error::addSourceMessage(
            &(Error::OPERATOR_NOT_ENCAPSULATED.clone()),
            list![AbsynUtil::pathString(
                InstNode::fullPath(operatorNode.clone(), false)?,
                literal!("."),
                true,
                false
            )?],
            &(InstNode::info(&operatorNode)),
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn lookupOperatorFunctionsInType(
    mut operatorName: ArcStr,
    mut ty: &metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::List<metamodelica::Ref<Function::Function>>> {
    let mut functions: metamodelica::List<metamodelica::Ref<Function::Function>>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef> = crate::NFComponentRef::interned_EMPTY();
    let mut is_defined: bool;
    functions = (::match_deref::match_deref! { match &(Type::arrayElementType(ty)) {
        __esc_elem_ty @ Deref @ Type::COMPLEX { .. } => {
            elem_ty = (*__esc_elem_ty).clone();
            node = Type::complexNode(metamodelica::AsArg::as_arg(&elem_ty))?;
            match '__try0: {
                fn_ref = unwrap_break_err!(Function::lookupFunctionSimple(operatorName.clone(), node.clone(), InstContext::NO_CONTEXT.clone()), '__try0);
                is_defined = true;
                Ok::<_, &'static str>((is_defined.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    is_defined = __try0_o0;
                }
                Err(_) => {
                    is_defined = false;
                }
            }
            if is_defined {
                (fn_ref, _, _) = Function::instFunctionRef(fn_ref, InstContext::NO_CONTEXT.clone(), InstNode::info(&node))?;
                functions = Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?;
            } else {
                functions = metamodelica::nil();
            }
            functions
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(functions)
}

pub(crate) fn patchOperatorRecordConstructorBinding(
    mut r#fn: metamodelica::Ref<Function::Function>,
) -> Result<metamodelica::Ref<Function::Function>> {
    let mut r#fn: metamodelica::Ref<Function::Function> = r#fn;
    let mut output_node: metamodelica::Ref<InstNode::InstNode>;
    let mut output_comp: metamodelica::Ref<Component::NFComponent>;
    let mut output_binding: metamodelica::Ref<Binding::NFBinding>;
    if ((r#fn.outputs).len() as i32) != 1 {
        return Ok(r#fn);
    }
    output_node = InstNode::fromHandle(&((r#fn.outputs).head().cloned()?))?;
    output_comp = InstNode::component(&output_node)?;
    output_binding = Component::getBinding(&output_comp);
    if !(Binding::isBound(&output_binding)) {
        return Ok(r#fn);
    }
    output_binding = Binding::mapExp(
        output_binding,
        (std::sync::Arc::new({
            let __pe_b1 = r#fn.clone();
            move |__pe_a0| patchOperatorRecordConstructorBinding_traverser(__pe_a0, &__pe_b1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    output_comp = Component::setBinding(output_binding, output_comp)?;
    output_node = InstNode::updateComponent(output_comp, output_node)?;
    Ok(r#fn)
}

fn checkOperatorConstructorOutput(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut recordNode: metamodelica::Ref<InstNode::InstNode>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut info: &SourceInfo,
) -> Result<()> {
    let mut output_node: metamodelica::Ref<InstNode::InstNode>;
    let mut output_ty: metamodelica::Ref<InstNode::InstNode>;
    if ((r#fn.outputs).len() as i32) != 1 {
        Error::addSourceMessage(
            &(Error::OPERATOR_OVERLOADING_ONE_OUTPUT_ERROR.clone()),
            list![AbsynUtil::pathString(path.clone(), literal!("."), true, false)?],
            info,
        )?;
        return Err("fail");
    }
    output_node = InstNode::fromHandle(&((r#fn.outputs).head().cloned()?))?;
    output_ty = InstNode::classScope(output_node.clone())?;
    if !(InstNode::isSame(output_ty.clone(), recordNode.clone())) {
        Error::addSourceMessage(
            &(Error::OPERATOR_OVERLOADING_INVALID_OUTPUT_TYPE.clone()),
            list![
                InstNode::name(&output_node)?,
                AbsynUtil::pathString(path, literal!("."), true, false)?,
                InstNode::name(&recordNode)?,
                InstNode::name(&output_ty)?
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

fn patchOperatorRecordConstructorBinding_traverser(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut constructorFn: &metamodelica::Ref<Function::Function>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn, ty: __esc_ty, arguments: __esc_args, .. } } if (referenceEq(&*(InstNode::fromHandle(&constructorFn.node)?),&*(InstNode::fromHandle(&r#fn.node)?))) => {
            ty = (*__esc_ty).clone();
            args = (*__esc_args).clone();
            Expression::makeRecord(Function::name(constructorFn), ty.clone(), args.clone())
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}
