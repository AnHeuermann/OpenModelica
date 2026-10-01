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
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

pub type UseTable = metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>;

pub fn getTotalModel(
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut classPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>> = program;
    let mut used: UseTable;
    let mut prev_size: i32 = 0;
    used = UnorderedSet::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        13,
    );
    analysePath(classPath, used.clone())?;
    UnorderedSet::add(literal!("constructor"), used.clone())?;
    UnorderedSet::add(literal!("destructor"), used.clone())?;
    while UnorderedSet::size(used.clone()) != prev_size {
        prev_size = UnorderedSet::size(used.clone());
        analyseProgram(&program, used.clone())?;
    }
    program = saveElements(&program, used)?;
    Ok(program)
}

pub(crate) fn analyseProgram(
    mut program: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut used: UseTable,
) -> Result<()> {
    for mut e in &**program {
        analyseElement(metamodelica::AsArg::as_arg(&e), used.clone())?;
    }
    Ok(())
}

pub(crate) fn analyseElements(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut used: UseTable,
) -> Result<()> {
    for mut e in &**elements {
        analyseElement(metamodelica::AsArg::as_arg(&e), used.clone())?;
    }
    Ok(())
}

pub(crate) fn analyseElement(mut element: &metamodelica::Ref<SCode::Element>, mut used: UseTable) -> Result<()> {
    let () = (match &**element {
        SCode::Element::IMPORT { imp: __element_imp, .. } => {
            analyseImport(metamodelica::AsArg::as_arg(&__element_imp), used.clone())?;
            ()
        }
        SCode::Element::EXTENDS {
            baseClassPath: __element_baseClassPath,
            modifications: __element_modifications,
            ..
        } => {
            analysePath(metamodelica::AsArg::as_arg(&__element_baseClassPath), used.clone())?;
            analyseMod(metamodelica::AsArg::as_arg(&__element_modifications), used.clone())?;
            ()
        }
        SCode::Element::CLASS {
            classDef: __element_classDef,
            cmt: __element_cmt,
            name: __element_name,
            prefixes: __element_prefixes,
            ..
        } if (UnorderedSet::contains(__element_name.clone(), used.clone())?) => {
            if SCodeUtil::isOperatorRecord(element) {
                analyseOperatorRecord(element, used.clone())?;
            }
            analyseClassDef(metamodelica::AsArg::as_arg(&__element_classDef), used.clone())?;
            analysePrefixes(metamodelica::AsArg::as_arg(&__element_prefixes), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__element_cmt), used.clone())?;
            ()
        }
        SCode::Element::COMPONENT {
            attributes: __element_attributes,
            comment: __element_comment,
            condition: __element_condition,
            modifications: __element_modifications,
            prefixes: __element_prefixes,
            typeSpec: __element_typeSpec,
            ..
        } => {
            analysePrefixes(metamodelica::AsArg::as_arg(&__element_prefixes), used.clone())?;
            analyseAttributes(metamodelica::AsArg::as_arg(&__element_attributes), used.clone())?;
            analyseTypeSpec(metamodelica::AsArg::as_arg(&__element_typeSpec), used.clone())?;
            analyseMod(metamodelica::AsArg::as_arg(&__element_modifications), used.clone())?;
            analyseExpOpt(__element_condition.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__element_comment), used.clone())?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn analyseImport(mut imp: &Absyn::Import, mut used: UseTable) -> Result<()> {
    analysePath(&(AbsynUtil::importPath(imp)), used)?;
    Ok(())
}

pub(crate) fn analyseClassDef(mut def: &metamodelica::Ref<SCode::ClassDef>, mut used: UseTable) -> Result<()> {
    let () = (match &**def {
        SCode::ClassDef::PARTS {
            elementLst: __def_elementLst,
            externalDecl: __def_externalDecl,
            initialAlgorithmLst: __def_initialAlgorithmLst,
            initialEquationLst: __def_initialEquationLst,
            normalAlgorithmLst: __def_normalAlgorithmLst,
            normalEquationLst: __def_normalEquationLst,
            ..
        } => {
            analyseElements(metamodelica::AsArg::as_arg(&__def_elementLst), used.clone())?;
            analyseEquations(metamodelica::AsArg::as_arg(&__def_normalEquationLst), used.clone())?;
            analyseEquations(metamodelica::AsArg::as_arg(&__def_initialEquationLst), used.clone())?;
            analyseAlgorithms(metamodelica::AsArg::as_arg(&__def_normalAlgorithmLst), used.clone())?;
            analyseAlgorithms(metamodelica::AsArg::as_arg(&__def_initialAlgorithmLst), used.clone())?;
            if (__def_externalDecl).is_some() {
                analyseExternalDecl(&(__def_externalDecl.clone().ok_or("pattern mismatch")?), used)?;
            }
            ()
        }
        SCode::ClassDef::CLASS_EXTENDS {
            composition: __def_composition,
            modifications: __def_modifications,
        } => {
            analyseMod(metamodelica::AsArg::as_arg(&__def_modifications), used.clone())?;
            analyseClassDef(metamodelica::AsArg::as_arg(&__def_composition), used)?;
            ()
        }
        SCode::ClassDef::DERIVED {
            attributes: __def_attributes,
            modifications: __def_modifications,
            typeSpec: __def_typeSpec,
        } => {
            analyseTypeSpec(metamodelica::AsArg::as_arg(&__def_typeSpec), used.clone())?;
            analyseMod(metamodelica::AsArg::as_arg(&__def_modifications), used.clone())?;
            analyseAttributes(metamodelica::AsArg::as_arg(&__def_attributes), used)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn analyseExternalDecl(
    mut extDecl: &metamodelica::Ref<SCode::ExternalDecl>,
    mut used: UseTable,
) -> Result<()> {
    if (extDecl.annotation_).is_some() {
        analyseAnnotation(&(extDecl.annotation_.clone().ok_or("pattern mismatch")?), used)?;
    }
    Ok(())
}

pub(crate) fn analyseOperatorRecord(mut element: &metamodelica::Ref<SCode::Element>, mut used: UseTable) -> Result<()> {
    let () = (match &**element {
        SCode::Element::CLASS {
            name: __element_name, ..
        } => {
            UnorderedSet::add(__element_name.clone(), used.clone())?;
            for mut e in &*SCodeUtil::getClassElements(element) {
                analyseOperatorRecord(metamodelica::AsArg::as_arg(&e), used.clone())?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn analyseAttributes(mut attributes: &SCode::Attributes, mut used: UseTable) -> Result<()> {
    analyseDims(attributes.arrayDims.clone(), used)?;
    Ok(())
}

pub(crate) fn analysePrefixes(mut prefixes: &metamodelica::Ref<SCode::Prefixes>, mut used: UseTable) -> Result<()> {
    analyseReplaceable(&prefixes.replaceablePrefix, used)?;
    Ok(())
}

pub(crate) fn analyseReplaceable(mut repl: &metamodelica::Ref<SCode::Replaceable>, mut used: UseTable) -> Result<()> {
    let mut cc: metamodelica::Ref<SCode::ConstrainClass>;
    let () = (::match_deref::match_deref! { match repl {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(__esc_cc) } => {
            cc = (*__esc_cc).clone();
            analyseConstrainClass(metamodelica::AsArg::as_arg(&cc), used)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn analyseConstrainClass(
    mut cc: &metamodelica::Ref<SCode::ConstrainClass>,
    mut used: UseTable,
) -> Result<()> {
    analysePath(&cc.constrainingClass, used.clone())?;
    analyseMod(&cc.modifier, used.clone())?;
    analyseComment(&cc.comment, used)?;
    Ok(())
}

pub(crate) fn analyseMod(mut r#mod: &metamodelica::Ref<SCode::Mod>, mut used: UseTable) -> Result<()> {
    let () = (match &**r#mod {
        SCode::Mod::MOD {
            binding: __mod_binding,
            subModLst: __mod_subModLst,
            ..
        } => {
            for mut s in &*__mod_subModLst.clone() {
                analyseMod(&s.r#mod, used.clone())?;
            }
            analyseExpOpt(__mod_binding.clone(), used)?;
            ()
        }
        SCode::Mod::REDECL {
            element: __mod_element, ..
        } => {
            analyseElement(metamodelica::AsArg::as_arg(&__mod_element), used)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn analyseTypeSpec(mut ty: &metamodelica::Ref<Absyn::TypeSpec>, mut used: UseTable) -> Result<()> {
    let () = (match &**ty {
        Absyn::TypeSpec::TPATH {
            arrayDim: __ty_arrayDim,
            path: __ty_path,
        } => {
            analysePath(metamodelica::AsArg::as_arg(&__ty_path), used.clone())?;
            if (__ty_arrayDim).is_some() {
                analyseDims(__ty_arrayDim.clone().ok_or("pattern mismatch")?, used)?;
            }
            ()
        }
        Absyn::TypeSpec::TCOMPLEX {
            arrayDim: __ty_arrayDim,
            path: __ty_path,
            typeSpecs: __ty_typeSpecs,
        } => {
            analysePath(metamodelica::AsArg::as_arg(&__ty_path), used.clone())?;
            for mut t in &*__ty_typeSpecs.clone() {
                analyseTypeSpec(metamodelica::AsArg::as_arg(&t), used.clone())?;
            }
            if (__ty_arrayDim).is_some() {
                analyseDims(__ty_arrayDim.clone().ok_or("pattern mismatch")?, used)?;
            }
            ()
        }
    });
    Ok(())
}

pub(crate) fn analysePath(mut path: &metamodelica::Ref<Absyn::Path>, mut used: UseTable) -> Result<()> {
    for mut i in &*AbsynUtil::pathToStringList(path) {
        UnorderedSet::add(i.clone(), used.clone())?;
    }
    Ok(())
}

pub(crate) fn analyseEquations(
    mut eqs: &metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut used: UseTable,
) -> Result<()> {
    for mut e in &**eqs {
        analyseEquation(metamodelica::AsArg::as_arg(&e), used.clone())?;
    }
    Ok(())
}

pub(crate) fn analyseEquation(mut eq: &metamodelica::Ref<SCode::Equation>, mut used: UseTable) -> Result<()> {
    let () = (match &**eq {
        SCode::Equation::EQ_IF {
            comment: __eq_comment,
            condition: __eq_condition,
            elseBranch: __eq_elseBranch,
            thenBranch: __eq_thenBranch,
            ..
        } => {
            analyseExpList(metamodelica::AsArg::as_arg(&__eq_condition), used.clone())?;
            for mut b in &*__eq_thenBranch.clone() {
                analyseEquations(metamodelica::AsArg::as_arg(&b), used.clone())?;
            }
            analyseEquations(metamodelica::AsArg::as_arg(&__eq_elseBranch), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_EQUALS {
            comment: __eq_comment,
            expLeft: __eq_expLeft,
            expRight: __eq_expRight,
            ..
        } => {
            analyseExp(__eq_expLeft.clone(), used.clone())?;
            analyseExp(__eq_expRight.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_PDE {
            comment: __eq_comment,
            expLeft: __eq_expLeft,
            expRight: __eq_expRight,
            ..
        } => {
            analyseExp(__eq_expLeft.clone(), used.clone())?;
            analyseExp(__eq_expRight.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_CONNECT {
            comment: __eq_comment,
            crefLeft: __eq_crefLeft,
            crefRight: __eq_crefRight,
            ..
        } => {
            analyseCref(metamodelica::AsArg::as_arg(&__eq_crefLeft), used.clone(), true)?;
            analyseCref(metamodelica::AsArg::as_arg(&__eq_crefRight), used.clone(), true)?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_FOR {
            comment: __eq_comment,
            eEquationLst: __eq_eEquationLst,
            range: __eq_range,
            ..
        } => {
            analyseExpOpt(__eq_range.clone(), used.clone())?;
            analyseEquations(metamodelica::AsArg::as_arg(&__eq_eEquationLst), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_WHEN {
            comment: __eq_comment,
            condition: __eq_condition,
            eEquationLst: __eq_eEquationLst,
            elseBranches: __eq_elseBranches,
            ..
        } => {
            analyseExp(__eq_condition.clone(), used.clone())?;
            analyseEquations(metamodelica::AsArg::as_arg(&__eq_eEquationLst), used.clone())?;
            for mut b in &*__eq_elseBranches.clone() {
                analyseExp(Util::tuple21(b.clone()), used.clone())?;
                analyseEquations(&(Util::tuple22(b.clone())), used.clone())?;
            }
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_ASSERT {
            comment: __eq_comment,
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => {
            analyseExp(__eq_condition.clone(), used.clone())?;
            analyseExp(__eq_message.clone(), used.clone())?;
            analyseExp(__eq_level.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_TERMINATE {
            comment: __eq_comment,
            message: __eq_message,
            ..
        } => {
            analyseExp(__eq_message.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_REINIT {
            comment: __eq_comment,
            cref: __eq_cref,
            expReinit: __eq_expReinit,
            ..
        } => {
            analyseExp(__eq_cref.clone(), used.clone())?;
            analyseExp(__eq_expReinit.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
        SCode::Equation::EQ_NORETCALL {
            comment: __eq_comment,
            exp: __eq_exp,
            ..
        } => {
            analyseExp(__eq_exp.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__eq_comment), used)?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn analyseAlgorithms(
    mut algs: &metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut used: UseTable,
) -> Result<()> {
    for mut a in &**algs {
        analyseAlgorithm(metamodelica::AsArg::as_arg(&a), used.clone())?;
    }
    Ok(())
}

pub(crate) fn analyseAlgorithm(mut alg: &metamodelica::Ref<SCode::AlgorithmSection>, mut used: UseTable) -> Result<()> {
    analyseStatements(&alg.statements, used)?;
    Ok(())
}

pub(crate) fn analyseStatements(
    mut stmts: &metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut used: UseTable,
) -> Result<()> {
    for mut s in &**stmts {
        analyseStatement(metamodelica::AsArg::as_arg(&s), used.clone())?;
    }
    Ok(())
}

pub(crate) fn analyseStatement(mut stmt: &metamodelica::Ref<SCode::Statement>, mut used: UseTable) -> Result<()> {
    let () = (match &**stmt {
        SCode::Statement::ALG_ASSIGN {
            assignComponent: __stmt_assignComponent,
            comment: __stmt_comment,
            value: __stmt_value,
            ..
        } => {
            analyseExp(__stmt_assignComponent.clone(), used.clone())?;
            analyseExp(__stmt_value.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_IF {
            boolExpr: __stmt_boolExpr,
            comment: __stmt_comment,
            elseBranch: __stmt_elseBranch,
            elseIfBranch: __stmt_elseIfBranch,
            trueBranch: __stmt_trueBranch,
            ..
        } => {
            analyseExp(__stmt_boolExpr.clone(), used.clone())?;
            analyseStatements(metamodelica::AsArg::as_arg(&__stmt_trueBranch), used.clone())?;
            for mut b in &*__stmt_elseIfBranch.clone() {
                analyseExp(Util::tuple21(b.clone()), used.clone())?;
                analyseStatements(&(Util::tuple22(b.clone())), used.clone())?;
            }
            analyseStatements(metamodelica::AsArg::as_arg(&__stmt_elseBranch), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_FOR {
            comment: __stmt_comment,
            forBody: __stmt_forBody,
            range: __stmt_range,
            ..
        } => {
            analyseExpOpt(__stmt_range.clone(), used.clone())?;
            analyseStatements(metamodelica::AsArg::as_arg(&__stmt_forBody), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_PARFOR {
            comment: __stmt_comment,
            parforBody: __stmt_parforBody,
            range: __stmt_range,
            ..
        } => {
            analyseExpOpt(__stmt_range.clone(), used.clone())?;
            analyseStatements(metamodelica::AsArg::as_arg(&__stmt_parforBody), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_WHILE {
            boolExpr: __stmt_boolExpr,
            comment: __stmt_comment,
            whileBody: __stmt_whileBody,
            ..
        } => {
            analyseExp(__stmt_boolExpr.clone(), used.clone())?;
            analyseStatements(metamodelica::AsArg::as_arg(&__stmt_whileBody), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_WHEN_A {
            branches: __stmt_branches,
            comment: __stmt_comment,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                analyseExp(Util::tuple21(b.clone()), used.clone())?;
                analyseStatements(&(Util::tuple22(b.clone())), used.clone())?;
            }
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_ASSERT {
            comment: __stmt_comment,
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            ..
        } => {
            analyseExp(__stmt_condition.clone(), used.clone())?;
            analyseExp(__stmt_message.clone(), used.clone())?;
            analyseExp(__stmt_level.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_TERMINATE {
            comment: __stmt_comment,
            message: __stmt_message,
            ..
        } => {
            analyseExp(__stmt_message.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_REINIT {
            comment: __stmt_comment,
            cref: __stmt_cref,
            newValue: __stmt_newValue,
            ..
        } => {
            analyseExp(__stmt_cref.clone(), used.clone())?;
            analyseExp(__stmt_newValue.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_NORETCALL {
            comment: __stmt_comment,
            exp: __stmt_exp,
            ..
        } => {
            analyseExp(__stmt_exp.clone(), used.clone())?;
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_RETURN {
            comment: __stmt_comment,
            ..
        } => {
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_BREAK {
            comment: __stmt_comment,
            ..
        } => {
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        SCode::Statement::ALG_CONTINUE {
            comment: __stmt_comment,
            ..
        } => {
            analyseComment(metamodelica::AsArg::as_arg(&__stmt_comment), used)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub use analyseSubscripts as analyseDims;

pub fn analyseSubscripts(
    mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut used: UseTable,
) -> Result<()> {
    for mut s in &*subs {
        analyseSubscript(metamodelica::AsArg::as_arg(&s), used.clone())?;
    }
    Ok(())
}

pub(crate) fn analyseSubscript(mut sub: &metamodelica::Ref<Absyn::Subscript>, mut used: UseTable) -> Result<()> {
    let () = (match &**sub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub_subscript,
        } => {
            analyseExp(__sub_subscript.clone(), used)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn analyseExpOpt(mut exp: Option<metamodelica::Ref<Absyn::Exp>>, mut used: UseTable) -> Result<()> {
    if (exp).is_some() {
        analyseExp(exp.ok_or("pattern mismatch")?, used)?;
    }
    Ok(())
}

pub(crate) fn analyseExpList(
    mut expl: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut used: UseTable,
) -> Result<()> {
    for mut e in &**expl {
        analyseExp(e.clone(), used.clone())?;
    }
    Ok(())
}

pub(crate) fn analyseExp(mut exp: metamodelica::Ref<Absyn::Exp>, mut used: UseTable) -> Result<()> {
    AbsynUtil::traverseExp(
        exp,
        (std::sync::Arc::new(analyseExpTraverse)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
                    )> + 'static,
            >),
        used,
    )?;
    Ok(())
}

pub(crate) fn analyseExpTraverse(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut used: UseTable,
) -> Result<(metamodelica::Ref<Absyn::Exp>, UseTable)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut used: UseTable = used;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } => {
            analyseCref(metamodelica::AsArg::as_arg(&__exp_componentRef), used.clone(), true)?;
            ()
        }
        Absyn::Exp::CALL {
            function_: __exp_function_,
            ..
        } => {
            analyseCref(metamodelica::AsArg::as_arg(&__exp_function_), used.clone(), true)?;
            ()
        }
        Absyn::Exp::PARTEVALFUNCTION {
            function_: __exp_function_,
            ..
        } => {
            analyseCref(metamodelica::AsArg::as_arg(&__exp_function_), used.clone(), true)?;
            ()
        }
        _ => (),
    });
    Ok((exp, used))
}

pub(crate) fn analyseCref(
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut used: UseTable,
    mut includeLast: bool,
) -> Result<()> {
    let () = (match &**cref {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => {
            analyseCref(metamodelica::AsArg::as_arg(&__cref_componentRef), used, includeLast)?;
            ()
        }
        Absyn::ComponentRef::CREF_QUAL {
            componentRef: __cref_componentRef,
            name: __cref_name,
            subscripts: __cref_subscripts,
        } => {
            UnorderedSet::add(__cref_name.clone(), used.clone())?;
            analyseSubscripts(__cref_subscripts.clone(), used.clone())?;
            analyseCref(metamodelica::AsArg::as_arg(&__cref_componentRef), used, includeLast)?;
            ()
        }
        Absyn::ComponentRef::CREF_IDENT {
            name: __cref_name,
            subscripts: __cref_subscripts,
        } => {
            if includeLast {
                UnorderedSet::add(__cref_name.clone(), used.clone())?;
            }
            analyseSubscripts(__cref_subscripts.clone(), used)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn analyseComment(mut comment: &metamodelica::Ref<SCode::Comment>, mut used: UseTable) -> Result<()> {
    if (comment.annotation_).is_some() {
        analyseAnnotation(&(comment.annotation_.clone().ok_or("pattern mismatch")?), used)?;
    }
    Ok(())
}

pub(crate) fn analyseAnnotation(mut ann: &metamodelica::Ref<SCode::Annotation>, mut used: UseTable) -> Result<()> {
    analyseMod(&ann.modification, used)?;
    Ok(())
}

pub(crate) fn saveElements(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut used: UseTable,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    for mut e in &**elements {
        outElements = saveElement(e.clone(), used.clone(), outElements)?;
    }
    outElements = metamodelica::Dangerous::listReverseInPlace(outElements);
    Ok(outElements)
}

pub(crate) fn saveElement(
    mut element: metamodelica::Ref<SCode::Element>,
    mut used: UseTable,
    mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>> = elements;
    let mut elem: metamodelica::Ref<SCode::Element> = element.clone();
    elements = (match &*elem {
        SCode::Element::CLASS {
            classDef: __elem_classDef,
            name: __elem_name,
            ..
        } if (UnorderedSet::contains(__elem_name.clone(), used.clone())?) => {
            assign_variant_field!(elem => SCode::Element::CLASS; classDef = saveClassDef(__elem_classDef.clone(), used.clone())?);
            metamodelica::cons(elem, elements)
        }
        SCode::Element::CLASS { .. } => elements,
        SCode::Element::EXTENDS {
            baseClassPath: __elem_baseClassPath,
            ..
        } if (AbsynUtil::pathContains(metamodelica::AsArg::as_arg(&__elem_baseClassPath), &(literal!("Icons")))) => {
            elements
        }
        _ => metamodelica::cons(element, elements),
    });
    Ok(elements)
}

pub(crate) fn saveClassDef(
    mut def: metamodelica::Ref<SCode::ClassDef>,
    mut used: UseTable,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut def: metamodelica::Ref<SCode::ClassDef> = def;
    let () = (match &*def {
        SCode::ClassDef::PARTS {
            elementLst: __def_elementLst,
            ..
        } => {
            assign_variant_field!(def => SCode::ClassDef::PARTS; elementLst = saveElements(metamodelica::AsArg::as_arg(&__def_elementLst), used)?);
            ()
        }
        SCode::ClassDef::CLASS_EXTENDS {
            composition: __def_composition,
            ..
        } => {
            assign_variant_field!(def => SCode::ClassDef::CLASS_EXTENDS; composition = saveClassDef(__def_composition.clone(), used)?);
            ()
        }
        _ => (),
    });
    Ok(def)
}
