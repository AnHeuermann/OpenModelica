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

use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCheckModel as CheckModel;
use crate::NFClass as Class;
use crate::NFClassTree;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectBreakTree as ConnectBreakTree;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFConvertDAE as ConvertDAE;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEvalConstants as EvalConstants;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFInstUtil as InstUtil;
use crate::NFLookup as Lookup;
use crate::NFModifier::Modifier;
use crate::NFModifier::ModifierScope;
use crate::NFOperator as Operator;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPackage as Package;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFScalarize as Scalarize;
use crate::NFSections as Sections;
use crate::NFSimplifyModel as SimplifyModel;
use crate::NFStateMachineFlatten as StateMachineFlatten;
use crate::NFStatement as Statement;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFUnitCheck as UnitCheck;
use crate::NFVariable as Variable;
use crate::NFVerifyModel as VerifyModel;
use openmodelica_ast::Absyn;
use openmodelica_ast::Absyn::Path;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;

pub mod InstSettings {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct InstSettings {
        /// Merge sections from extends clauses if true
        pub mergeExtendsSections: bool,
        /// Consider all arrays resizable if true
        pub resizableArrays: bool,
    }

    impl metamodelica::gc::MMTrace for InstSettings {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.mergeExtendsSections, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.resizableArrays, __mmv)?;
            Ok(())
        }
    }
    impl Default for InstSettings {
        fn default() -> Self {
            Self {
                mergeExtendsSections: Default::default(),
                resizableArrays: Default::default(),
            }
        }
    }

    pub type SETTINGS = InstSettings;

    pub(crate) fn create() -> Result<metamodelica::Ref<InstSettings>> {
        let mut settings: metamodelica::Ref<InstSettings> = metamodelica::Ref::new(InstSettings {
            mergeExtendsSections: true,
            resizableArrays: Flags::getConfigBool(Flags::RESIZABLE_ARRAYS.clone())?,
        });
        Ok(settings)
    }
}

pub static DEFAULT_SETTINGS: std::sync::LazyLock<metamodelica::Ref<InstSettings::InstSettings>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(InstSettings::InstSettings {
            mergeExtendsSections: true,
            resizableArrays: false,
        })
    });

pub(crate) fn Inst_makeTopNode(
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut annotationProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::Ref<InstNode::InstNode> {
    let mut topNode: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("Inst_makeTopNode"), lang: Some("C"), output_: Some(CREF_IDENT { name: "topNode", subscripts: [] }), args: [CREF { componentRef: CREF_IDENT { name: "program", subscripts: [] } }, CREF { componentRef: CREF_IDENT { name: "annotationProgram", subscripts: [] } }], annotation_: None }, annotation: None }
    topNode
}

pub fn instClassInProgram(
    mut classPath: metamodelica::Ref<Path>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut annotationProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut relaxedFrontend: bool,
    mut dumpFlat: bool,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    ArcStr,
)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>;
    let mut functions: metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>;
    let mut flatString: ArcStr;
    System::reportProgress(-1, 3);
    if let Ok((__pa0, __pa1, __pa2)) = instClassInProgram2(
        classPath.clone(),
        program.clone(),
        annotationProgram.clone(),
        relaxedFrontend,
        dumpFlat,
    ) {
        flatModel = metamodelica::Own::own(__pa0);
        functions = metamodelica::Own::own(__pa1);
        flatString = metamodelica::Own::own(__pa2);
    } else {
        System::reportProgress(-1, 0);
        return Err("fail");
    }
    System::reportProgress(-1, 0);
    Ok((flatModel, functions, flatString))
}

pub(crate) fn instClassInProgram2(
    mut classPath: metamodelica::Ref<Path>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut annotationProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut relaxedFrontend: bool,
    mut dumpFlat: bool,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    ArcStr,
)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>;
    let mut functions: metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>;
    let mut flatString: ArcStr = arcstr::literal!("");
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    let mut inst_cls: metamodelica::Ref<InstNode::InstNode>;
    let mut context: i32;
    let mut prog: metamodelica::List<metamodelica::Ref<SCode::Element>> = program;
    let mut settings: metamodelica::Ref<InstSettings::InstSettings>;
    resetGlobalFlags()?;
    context = if (relaxedFrontend
        || Flags::getConfigBool(Flags::CHECK_MODEL.clone())?
        || Flags::isSet(Flags::NF_API.clone())?)
    {
        InstContext::RELAXED.clone()
    } else {
        InstContext::NO_CONTEXT.clone()
    };
    top = makeTopNode(prog, annotationProgram)?;
    cls = lookupRootClass(classPath.clone(), top, context)?;
    if SCodeUtil::isFunction(&(NFInstNode::InstNode::definition(cls.clone())?)) {
        (flatModel, functions, flatString) = instantiateRootFunction(cls, context)?;
        return Ok((flatModel, functions, flatString));
    }
    inst_cls = instantiateRootClass(cls, context, crate::NFModifier::Modifier::interned_NOMOD())?;
    execStat(
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("NFInst.instantiate("));
            __mm_s.push_str(&*AbsynUtil::pathString(classPath.clone(), literal!("."), true, false)?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    settings = InstSettings::create()?;
    Error::checkCancel()?;
    instExpressions(
        inst_cls.clone(),
        &(inst_cls.clone()),
        crate::NFSections::interned_EMPTY(),
        &(ConnectBreakTree::new()),
        context,
        &settings,
    )?;
    execStat(&(literal!("NFInst.instExpressions")))?;
    updateImplicitVariability(inst_cls.clone(), Flags::isSet(Flags::EVAL_PARAM.clone())?, context)?;
    execStat(&(literal!("NFInst.updateImplicitVariability")))?;
    Error::checkCancel()?;
    Typing::typeClass(inst_cls.clone(), context)?;
    Error::checkCancel()?;
    flatModel = Flatten::flatten(inst_cls.clone(), classPath, true)?;
    flatModel = EvalConstants::evaluate(flatModel, context)?;
    InstUtil::dumpFlatModelDebug(literal!("eval"), flatModel.clone(), &(Flatten::FunctionTreeImpl::new()))?;
    flatModel = UnitCheck::checkUnits(flatModel)?;
    if !(Flags::getConfigBool(Flags::NO_SIMPLIFY.clone())?) {
        flatModel = SimplifyModel::simplify(flatModel)?;
        InstUtil::dumpFlatModelDebug(
            literal!("simplify"),
            flatModel.clone(),
            &(Flatten::FunctionTreeImpl::new()),
        )?;
    }
    flatModel = StateMachineFlatten::flatten(flatModel)?;
    InstUtil::dumpFlatModelDebug(
        literal!("stateMachineFlatten"),
        flatModel.clone(),
        &(Flatten::FunctionTreeImpl::new()),
    )?;
    flatModel = Package::collectConstants(flatModel)?;
    functions = Flatten::collectFunctions(&flatModel)?;
    if !(Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), literal!("scalarize"))?) {
        flatString = if (dumpFlat) {
            InstUtil::dumpFlatModel(flatModel.clone(), &functions)?
        } else {
            literal!("")
        };
    }
    InstUtil::printStructuralParameters(&flatModel)?;
    if Flags::isSet(Flags::NF_SCALARIZE.clone())? {
        flatModel = Scalarize::scalarize(flatModel)?;
    } else {
        assign_field!(
            flatModel.variables = List::filterOnFalse(flatModel.variables.clone(), &move |__a0: metamodelica::Ref<
                Variable::NFVariable,
            >| {
                Variable::isEmptyArray(&__a0)
            })?
        );
        assign_field!(
            flatModel.variables = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
                for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                    let __x = Flatten::fillVectorizedVariableBinding(v.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        );
    }
    flatModel = InstUtil::replaceEmptyArrays(flatModel)?;
    InstUtil::dumpFlatModelDebug(literal!("scalarize"), flatModel.clone(), &functions)?;
    if Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), literal!("scalarize"))? {
        flatString = if (dumpFlat) {
            InstUtil::dumpFlatModel(flatModel.clone(), &functions)?
        } else {
            literal!("")
        };
    }
    if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
        flatModel = SimplifyModel::combineBinaries(flatModel)?;
        execStat(&(literal!("combineBinaries")))?;
        assign_field!(
            flatModel.equations = Equation::mapExpList(
                flatModel.equations.clone(),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Call::NFCall>) -> Result<metamodelica::Ref<Call::NFCall>>
                            + 'static,
                    > = (std::sync::Arc::new({
                        let __pe_b1 = Pointer::create(1);
                        move |__pe_a0| Call::toArrayConstructor(__pe_a0, __pe_b1.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Call::NFCall>,
                                )
                                    -> Result<metamodelica::Ref<Call::NFCall>>
                                + 'static,
                        >);
                    move |__pe_a0| Expression::wrapCall(__pe_a0, &*__pe_b1)
                })
            )?,
            flatModel.variables = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
                for mut var in (flatModel.variables.clone()).into_iter().cloned() {
                    let __x = Variable::mapExp(
                        var.clone(),
                        (std::sync::Arc::new({
                            let __pe_b1: Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Call::NFCall>,
                                    )
                                        -> Result<metamodelica::Ref<Call::NFCall>>
                                    + 'static,
                            > = (std::sync::Arc::new({
                                let __pe_b1 = Pointer::create(1);
                                move |__pe_a0| Call::toArrayConstructor(__pe_a0, __pe_b1.clone())
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<Call::NFCall>,
                                        )
                                            -> Result<metamodelica::Ref<Call::NFCall>>
                                        + 'static,
                                >);
                            move |__pe_a0| Expression::wrapCall(__pe_a0, &*__pe_b1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        );
        execStat(&(literal!("replaceArrayConstructors")))?;
    }
    VerifyModel::verify(&flatModel, NFInstNode::InstNode::isPartial(&inst_cls)?)?;
    (flatModel, functions) = InstUtil::expandSlicedCrefs(flatModel, functions)?;
    flatModel = InstUtil::combineSubscripts(flatModel)?;
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut var in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = Variable::propagateAnnotation(literal!("HideResult"), false, true, var.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    flatModel = FlatModel::removeNonTopLevelDirections(flatModel)?;
    if metamodelica::stringEq(
        &(Flags::getConfigString(Flags::OBFUSCATE.clone())?),
        &(literal!("protected")),
    ) || metamodelica::stringEq(
        &(Flags::getConfigString(Flags::OBFUSCATE.clone())?),
        &(literal!("encrypted")),
    ) {
        flatModel = FlatModel::obfuscate(flatModel)?;
    }
    clearCaches()?;
    Ok((flatModel, functions, flatString))
}

pub fn instClassForConnection(
    mut classPath: metamodelica::Ref<Path>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut annotationProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut connList: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut conns: metamodelica::Ref<Connections::NFConnections>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    let mut inst_cls: metamodelica::Ref<InstNode::InstNode>;
    let mut context: i32;
    resetGlobalFlags()?;
    context = if (Flags::getConfigBool(Flags::CHECK_MODEL.clone())? || Flags::isSet(Flags::NF_API.clone())?) {
        InstContext::RELAXED.clone()
    } else {
        InstContext::NO_CONTEXT.clone()
    };
    top = makeTopNode(program, annotationProgram)?;
    cls = lookupRootClass(classPath.clone(), top, context)?;
    inst_cls = instantiateRootClass(cls, context, crate::NFModifier::Modifier::interned_NOMOD())?;
    instExpressions(
        inst_cls.clone(),
        &(inst_cls.clone()),
        crate::NFSections::interned_EMPTY(),
        &(ConnectBreakTree::new()),
        context,
        &(DEFAULT_SETTINGS.clone()),
    )?;
    Typing::typeClass(inst_cls.clone(), context)?;
    conns = Flatten::flattenConnection(inst_cls, classPath)?;
    connList = Connections::toStringList(&conns)?;
    clearCaches()?;
    Ok(connList)
}

pub fn resetGlobalFlags() -> Result<()> {
    if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
        if !(Flags::isSet(Flags::FORCE_SCALARIZE.clone())?) {
            FlagsUtil::set(Flags::NF_SCALARIZE.clone(), false)?;
        }
        FlagsUtil::set(Flags::VECTORIZE_BINDINGS.clone(), true)?;
    }
    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        FlagsUtil::set(Flags::NF_EXPAND_OPERATIONS.clone(), false)?;
        FlagsUtil::set(Flags::NF_EXPAND_FUNC_ARGS.clone(), false)?;
    }
    System::setUsesCardinality(false);
    System::setHasOverconstrainedConnectors(false);
    System::setHasStreamConnectors(false);
    Ok(())
}

pub fn clearCaches() -> Result<()> {
    EvalFunction::clearLibraryCache()?;
    Ok(())
}

pub fn lookupRootClass(
    mut path: metamodelica::Ref<Path>,
    mut topScope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut clsNode: metamodelica::Ref<InstNode::InstNode>;
    let mut next_context: i32;
    let mut last: ArcStr;
    let mut cty: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut structor_ref: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    next_context = InstContext::set(context, InstContext::RELAXED.clone());
    ErrorExt::setCheckpoint(literal!("NFInst.lookupRootClass"));
    match '__try0: {
        (clsNode, _) = unwrap_break_err!(Lookup::lookupClassName(path.clone(), topScope.clone(), next_context, Absyn::dummyInfo.clone(), false), '__try0);
        ErrorExt::delCheckpoint(literal!("NFInst.lookupRootClass"));
        Ok::<_, &'static str>((clsNode.clone(),))
    } {
        Ok((__try0_o0,)) => {
            clsNode = __try0_o0;
        }
        Err(_) => {
            match '__try1: {
                last = AbsynUtil::pathLastIdent(&path);
                let true = (metamodelica::stringEq(&last, &(literal!("constructor")))
                    || metamodelica::stringEq(&last, &(literal!("destructor"))))
                else {
                    break '__try1 Err::<_, _>("pattern mismatch");
                };
                (clsNode, _, _) = unwrap_break_err!(Lookup::lookupName(&(unwrap_break_err!(AbsynUtil::stripLast(&path), '__try1)), topScope.clone(), next_context, false), '__try1);
                let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(NFInstNode::InstNode::getType(clsNode.clone()), '__try1)) {
                    Deref @ Type::COMPLEX { complexTy: __pa2, .. } => __pa2.clone(),
                    _ => break '__try1 Err::<_, _>("pattern mismatch"),
                } };
                cty = metamodelica::Own::own(__pa2);
                if metamodelica::stringEq(&last, &(literal!("constructor"))) {
                    let __pa3 = ::match_deref::match_deref! { match &(cty.clone()) {
                        Deref @ ComplexType::EXTERNAL_OBJECT { constructor: __pa3, .. } => __pa3.clone(),
                        _ => break '__try1 Err::<_, _>("pattern mismatch"),
                    } };
                    structor_ref = metamodelica::Own::own(__pa3);
                } else {
                    let __pa4 = ::match_deref::match_deref! { match &(cty.clone()) {
                        Deref @ ComplexType::EXTERNAL_OBJECT { destructor: __pa4, .. } => __pa4.clone(),
                        _ => break '__try1 Err::<_, _>("pattern mismatch"),
                    } };
                    structor_ref = metamodelica::Own::own(__pa4);
                }
                clsNode = unwrap_break_err!(NFInstNode::InstNode::borrow(structor_ref.clone()), '__try1);
                ErrorExt::rollBack(literal!("NFInst.lookupRootClass"));
                Ok::<_, &'static str>((clsNode.clone(), cty.clone(), last.clone(), structor_ref.clone()))
            } {
                Ok((__try1_o0, __try1_o1, __try1_o2, __try1_o3)) => {
                    clsNode = __try1_o0;
                    cty = __try1_o1;
                    last = __try1_o2;
                    structor_ref = __try1_o3;
                }
                Err(__try1_err) => {
                    ErrorExt::delCheckpoint(literal!("NFInst.lookupRootClass"));
                    return Err(__try1_err);
                }
            }
        }
    }
    clsNode = InstUtil::mergeScalars(clsNode, path.clone(), true, InstUtil::makeMergeNameMap())?;
    checkInstanceRestriction(clsNode.clone(), path, context)?;
    clsNode = NFInstNode::InstNode::makeRootClass(clsNode, crate::NFInstNode::InstNode::interned_EMPTY_NODE(), None)?;
    Ok(clsNode)
}

pub fn instantiateRootClass(
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut r#mod: metamodelica::Ref<Modifier::Modifier>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut clsNode: metamodelica::Ref<InstNode::InstNode> = clsNode;
    clsNode = instantiate(
        clsNode,
        r#mod,
        crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        context,
        false,
    )?;
    checkPartialClass(&clsNode, context)?;
    insertGeneratedInners(
        clsNode.clone(),
        &(NFInstNode::InstNode::topScope(clsNode.clone())?),
        context,
    )?;
    Ok(clsNode)
}

pub(crate) fn instantiateRootFunction(
    mut funcNode: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    ArcStr,
)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>;
    let mut functions: metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>;
    let mut flatString: ArcStr = literal!("");
    Function::instFunctionNode(funcNode.clone(), context, NFInstNode::InstNode::info(&funcNode))?;
    functions = Flatten::FunctionTreeImpl::new();
    for mut r#fn in &*Function::typeNodeCache(funcNode.clone(), context)? {
        functions = Flatten::flattenFunction(r#fn.clone(), functions)?;
    }
    flatModel = metamodelica::Ref::new(FlatModel::NFFlatModel {
        name: metamodelica::Ref::new(Path::IDENT {
            name: NFInstNode::InstNode::name(&funcNode)?,
        }),
        variables: metamodelica::nil(),
        equations: metamodelica::nil(),
        initialEquations: metamodelica::nil(),
        algorithms: metamodelica::nil(),
        initialAlgorithms: metamodelica::nil(),
        source: ElementSource::createElementSource(
            NFInstNode::InstNode::info(&funcNode),
            None,
            &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
            (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
        ),
    });
    Ok((flatModel, functions, flatString))
}

pub fn instantiate(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut r#mod: metamodelica::Ref<Modifier::Modifier>,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut instPartial: bool,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    node = expand(node, context)?;
    if instPartial
        || !(NFInstNode::InstNode::isPartial(&node)?)
        || InstContext::inRelaxed(context)
        || InstContext::inRedeclared(context)
    {
        (node, _) = instClass(
            node,
            r#mod,
            Attributes::DEFAULT_ATTR().clone(),
            true,
            0,
            0,
            parent,
            context,
        )?;
    }
    Ok(node)
}

pub fn expand(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    node = partialInstClass(node)?;
    node = expandClass(node, context)?;
    Ok(node)
}

pub fn makeTopNode(
    mut topClasses: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut annotationClasses: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut topNode: metamodelica::Ref<InstNode::InstNode>;
    let mut top_classes: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cls_elem: metamodelica::Ref<SCode::Element>;
    let mut ann_package: metamodelica::Ref<SCode::Element>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut elems: metamodelica::Ref<ClassTree::ClassTree>;
    let mut node_ty: metamodelica::Ref<InstNodeType>;
    let mut ann_node: metamodelica::Ref<InstNode::InstNode>;
    let mut generated_inners: metamodelica::Ref<
        UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<InstNode::InstNode>>,
    >;
    let mut roots: MutableWeak::Roots;
    top_classes = topClasses;
    if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
        top_classes = metamodelica::cons(NFBuiltinFuncs::BASE_MODELICA_POSITIVE_MAX_SIMPLE.clone(), top_classes);
    }
    cls_elem = metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("<top>"),
        prefixes: SCode::defaultPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PACKAGE,
        classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
            elementLst: top_classes.clone(),
            normalEquationLst: metamodelica::nil(),
            initialEquationLst: metamodelica::nil(),
            normalAlgorithmLst: metamodelica::nil(),
            initialAlgorithmLst: metamodelica::nil(),
            constraintLst: metamodelica::nil(),
            clsattrs: metamodelica::nil(),
            externalDecl: None,
        }),
        cmt: metamodelica::Ref::new(SCode::Comment {
            annotation_: None,
            comment: None,
        }),
        info: Absyn::dummyInfo.clone(),
    });
    generated_inners = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    roots = MutableWeak::newRoots();
    node_ty = metamodelica::Ref::new(InstNodeType::TOP_SCOPE {
        annotationScope: crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        generatedInners: generated_inners.clone(),
        roots: roots.clone(),
    });
    topNode = NFInstNode::InstNode::newClass(cls_elem, crate::NFInstNode::InstNode::interned_EMPTY_NODE(), node_ty)?;
    ann_package = metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("<annotations>"),
        prefixes: SCode::defaultPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PACKAGE,
        classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
            elementLst: annotationClasses,
            normalEquationLst: metamodelica::nil(),
            initialEquationLst: metamodelica::nil(),
            normalAlgorithmLst: metamodelica::nil(),
            initialAlgorithmLst: metamodelica::nil(),
            constraintLst: metamodelica::nil(),
            clsattrs: metamodelica::nil(),
            externalDecl: None,
        }),
        cmt: metamodelica::Ref::new(SCode::Comment {
            annotation_: None,
            comment: None,
        }),
        info: Absyn::dummyInfo.clone(),
    });
    ann_node = NFInstNode::InstNode::newClass(
        ann_package,
        topNode.clone(),
        crate::NFInstNode::InstNodeType::interned_IMPLICIT_SCOPE(),
    )?;
    expand(ann_node.clone(), InstContext::NO_CONTEXT.clone())?;
    cls = NFInstNode::InstNode::getClass(ann_node.clone())?;
    elems = Class::classTree(cls.clone())?;
    ClassTree::mapClasses(&elems, &markBuiltinTypeNodes)?;
    cls = Class::setClassTree(elems, cls)?;
    ann_node = NFInstNode::InstNode::updateClass(cls, ann_node)?;
    node_ty = metamodelica::Ref::new(InstNodeType::TOP_SCOPE {
        annotationScope: ann_node,
        generatedInners: generated_inners,
        roots: roots,
    });
    topNode = NFInstNode::InstNode::setNodeType(node_ty, topNode)?;
    cls = Class::fromSCode(&top_classes, false, topNode.clone(), Class::DEFAULT_PREFIXES.clone())?;
    elems = Class::classTree(cls.clone())?;
    ClassTree::mapClasses(&elems, &markBuiltinTypeNodesByAnnotation)?;
    ClassTree::replaceClass(Builtin::CLOCK_NODE().clone(), elems.clone())?;
    cls = Class::setClassTree(elems, cls)?;
    topNode = NFInstNode::InstNode::updateClass(cls, topNode)?;
    {
        let __v = list![topNode.clone()];
        crate::Globals::nfTopScope.with(|__root| *__root.borrow_mut() = __v)
    };
    {
        let __v = metamodelica::nil();
        crate::Globals::nbCreatedVars.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(topNode)
}

pub(crate) fn markBuiltinTypeNodes(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    node = NFInstNode::InstNode::setNodeType(crate::NFInstNode::InstNodeType::interned_BUILTIN_CLASS(), node)?;
    Ok(node)
}

pub(crate) fn markBuiltinTypeNodesByAnnotation(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    if SCodeUtil::hasBooleanNamedAnnotationInClass(
        &(NFInstNode::InstNode::definition(node.clone())?),
        &(literal!("__OpenModelica_builtin")),
    ) {
        node = NFInstNode::InstNode::setNodeType(crate::NFInstNode::InstNodeType::interned_BUILTIN_CLASS(), node)?;
    }
    Ok(node)
}

pub(crate) fn partialInstClass(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut c: metamodelica::Ref<Class::NFClass>;
    let () = (match &*(NFInstNode::InstNode::getClass(node.clone())?) {
        Class::NOT_INSTANTIATED => {
            c = partialInstClass2(&(NFInstNode::InstNode::definition(node.clone())?), node.clone())?;
            node = NFInstNode::InstNode::updateClass(c.clone(), node)?;
            c = Class::initImports(c, &node)?;
            node = NFInstNode::InstNode::updateClass(c, node)?;
            ()
        }
        _ => (),
    });
    Ok(node)
}

pub(crate) fn partialInstClass2(
    mut definition: &metamodelica::Ref<SCode::Element>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Class::NFClass>> {
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    let mut ce_cdef: metamodelica::Ref<SCode::ClassDef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut prefs: metamodelica::Ref<Class::Prefixes::Prefixes>;
    Error::assertion(
        SCodeUtil::elementIsClass(definition),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("NFInst.partialInstClass2"));
            __mm_s.push_str(&*literal!(" got non-class element"));
            ArcStr::from(__mm_s)
        },
        &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")),
    )?;
    let __pa0 = ::match_deref::match_deref! { match &((*definition)) {
        Deref @ SCode::Element::CLASS { classDef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cdef = metamodelica::Own::own(__pa0);
    prefs = instClassPrefixes(definition)?;
    cls = (::match_deref::match_deref! { match &(cdef) {
        Deref @ SCode::ClassDef::PARTS { elementLst: __cdef_elementLst, .. } => Class::fromSCode(metamodelica::AsArg::as_arg(&__cdef_elementLst), false, scope, prefs)?,
        Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: __esc_ce_cdef @ Deref @ SCode::ClassDef::PARTS { .. }, .. } => {
            ce_cdef = (*__esc_ce_cdef).clone();
            if !(SCodeUtil::isElementRedeclare(definition)?) {
                Error::addSourceMessage(&(Error::CLASS_EXTENDS_MISSING_REDECLARE.clone()), list![SCodeUtil::elementName(definition)?], &(SCodeUtil::elementInfo(definition)))?;
            }
            Class::fromSCode(var_field!((*ce_cdef).elementLst, SCode::ClassDef::PARTS), true, scope, prefs)?
        },
        Deref @ SCode::ClassDef::ENUMERATION { enumLst: __cdef_enumLst } => {
            ty = makeEnumerationType(__cdef_enumLst.clone(), scope.clone())?;
            Class::fromEnumeration(metamodelica::AsArg::as_arg(&__cdef_enumLst), ty, prefs, scope)?
        },
        _ => metamodelica::Ref::new(Class::NFClass::PARTIAL_CLASS { elements: NFClassTree::EMPTY().clone(), modifier: crate::NFModifier::Modifier::interned_NOMOD(), ccMod: crate::NFModifier::Modifier::interned_NOMOD(), prefixes: prefs }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub(crate) fn instClassPrefixes(
    mut cls: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<Class::Prefixes::Prefixes>> {
    let mut prefixes: metamodelica::Ref<Class::Prefixes::Prefixes>;
    let mut prefs: metamodelica::Ref<SCode::Prefixes>;
    prefixes = (::match_deref::match_deref! { match cls {
        Deref @ SCode::Element::CLASS { encapsulatedPrefix: SCode::Encapsulated::NOT_ENCAPSULATED { .. }, partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, prefixes: Deref @ SCode::Prefixes { finalPrefix: SCode::Final::NOT_FINAL { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, .. }, .. } => Class::DEFAULT_PREFIXES.clone(),
        Deref @ SCode::Element::CLASS { prefixes: __esc_prefs, encapsulatedPrefix: __cls_encapsulatedPrefix, partialPrefix: __cls_partialPrefix, .. } => {
            prefs = (*__esc_prefs).clone();
            metamodelica::Ref::new(Class::Prefixes::Prefixes { encapsulatedPrefix: __cls_encapsulatedPrefix.clone(), partialPrefix: __cls_partialPrefix.clone(), finalPrefix: prefs.finalPrefix.clone(), innerOuter: prefs.innerOuter.clone(), replaceablePrefix: prefs.replaceablePrefix.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(prefixes)
}

pub(crate) fn makeEnumerationType(
    mut literals: metamodelica::List<metamodelica::Ref<SCode::Enum>>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut lits: metamodelica::List<ArcStr>;
    let mut path: metamodelica::Ref<Path>;
    path = NFInstNode::InstNode::scopePath(scope, NFInstNode::InstNode::ScopeType::RELATIVE.clone(), false)?;
    lits = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut e in (literals).into_iter().cloned() {
            let __x = e.literal.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    ty = metamodelica::Ref::new(Type::NFType::ENUMERATION {
        typePath: path,
        literals: lits,
    });
    Ok(ty)
}

pub(crate) fn expandClass(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    node = (match &*(NFInstNode::InstNode::getClass(node.clone())?) {
        Class::PARTIAL_CLASS { .. } => expandClass2(node, context)?,
        _ => node,
    });
    Ok(node)
}

pub(crate) fn expandClass2(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut def: metamodelica::Ref<SCode::Element> = NFInstNode::InstNode::definition(node.clone())?;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    let mut info: SourceInfo;
    let mut name_map: Option<
        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Absyn::ComponentRef>>>,
    > = None;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::Element::CLASS { classDef: __pa0, info: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cdef = metamodelica::Own::own(__pa0);
    info = metamodelica::Own::own(__pa1);
    node = (match &*cdef.clone() {
        SCode::ClassDef::PARTS { .. } => {
            (node, name_map) = expandClassParts(&def, node, context, &info)?;
            if (name_map).is_some() {
                InstUtil::mergeScalarsComponentBindings(node.clone(), Util::getOption(name_map)?)?;
            }
            node
        }
        SCode::ClassDef::CLASS_EXTENDS { .. } => {
            (node, name_map) = expandClassParts(&def, node, context, &info)?;
            if (name_map).is_some() {
                InstUtil::mergeScalarsComponentBindings(node.clone(), Util::getOption(name_map)?)?;
            }
            node
        }
        SCode::ClassDef::DERIVED {
            typeSpec: __cdef_typeSpec,
            ..
        } => {
            (match &*__cdef_typeSpec.clone() {
                Absyn::TypeSpec::TCOMPLEX { .. } => expandClassDerivedComplex(&def, &cdef, node, &info)?,
                _ => expandClassDerived(&def, &cdef, node, context, info)?,
            })
        }
        SCode::ClassDef::OVERLOAD { .. } => node,
        SCode::ClassDef::PDER {
            functionPath: __cdef_functionPath,
            ..
        } => expandClassDerived(
            &def,
            &(metamodelica::Ref::new(SCode::ClassDef::DERIVED {
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: __cdef_functionPath.clone(),
                    arrayDim: None,
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                attributes: SCode::defaultVarAttr.clone(),
            })),
            node,
            context,
            info,
        )?,
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFInst.expandClass2"));
                    __mm_s.push_str(&*literal!(" got unknown class:\n"));
                    __mm_s.push_str(&*SCodeDump::unparseElementStr(def, SCodeDump::defaultOptions.clone())?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(node)
}

pub(crate) fn expandClassParts(
    mut def: &metamodelica::Ref<SCode::Element>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    Option<metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Absyn::ComponentRef>>>>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut nameMap: Option<
        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Absyn::ComponentRef>>>,
    >;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
    let mut cc_mod: metamodelica::Ref<Modifier::Modifier>;
    let mut builtin_ext: metamodelica::Ref<InstNode::InstNode>;
    let mut prefs: metamodelica::Ref<Class::Prefixes::Prefixes>;
    let mut res: metamodelica::Ref<Restriction::NFRestriction>;
    let mut name_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Absyn::ComponentRef>>>;
    cls = NFInstNode::InstNode::getClass(node.clone())?;
    cls = Class::initExpandedClass(cls)?;
    node = NFInstNode::InstNode::updateClass(cls.clone(), node)?;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(cls) {
        Deref @ Class::EXPANDED_CLASS { elements: __pa0, modifier: __pa1, ccMod: __pa2, prefixes: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cls_tree = metamodelica::Own::own(__pa0);
    r#mod = metamodelica::Own::own(__pa1);
    cc_mod = metamodelica::Own::own(__pa2);
    prefs = metamodelica::Own::own(__pa3);
    if ClassTree::extendsCount(&cls_tree) > 0 {
        name_map = InstUtil::makeMergeNameMap();
        builtin_ext = ClassTree::mapFoldExtends(
            &cls_tree,
            &({
                let __pe_b2 = context;
                let __pe_b3 = name_map.clone();
                move |__pe_a0, __pe_a1| expandExtends(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
            }),
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        )?;
        nameMap = if (UnorderedMap::isEmpty(name_map.clone())) {
            None
        } else {
            Some(name_map)
        };
    } else {
        builtin_ext = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
        nameMap = None;
    }
    if metamodelica::stringEq(
        &(NFInstNode::InstNode::name(&builtin_ext)?),
        &(literal!("ExternalObject")),
    ) {
        node = expandExternalObject(&cls_tree, node)?;
    } else {
        if !(NFInstNode::InstNode::isEmpty(&builtin_ext)) {
            checkBuiltinTypeExtends(&builtin_ext, &cls_tree, &node)?;
        }
        cls_tree = ClassTree::expand(cls_tree)?;
        res = Restriction::fromSCode(&(SCodeUtil::getClassRestriction(def)?));
        cls = metamodelica::Ref::new(Class::NFClass::EXPANDED_CLASS {
            elements: cls_tree,
            modifier: r#mod,
            ccMod: cc_mod,
            prefixes: prefs,
            restriction: res,
        });
        node = NFInstNode::InstNode::updateClass(cls, node)?;
    }
    Ok((node, nameMap))
}

pub(crate) fn expandExtends(
    mut ext: metamodelica::Ref<InstNode::InstNode>,
    mut builtinExt: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Absyn::ComponentRef>>>,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut ext: metamodelica::Ref<InstNode::InstNode> = ext;
    let mut builtinExt: metamodelica::Ref<InstNode::InstNode> = builtinExt;
    let mut def: metamodelica::Ref<SCode::Element>;
    let mut base_path: metamodelica::Ref<Path>;
    let mut base_nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    let mut base_node: metamodelica::Ref<InstNode::InstNode>;
    let mut smod: metamodelica::Ref<SCode::Mod>;
    let mut info: SourceInfo;
    if NFInstNode::InstNode::isEmpty(&ext) {
        return Ok((ext, builtinExt));
    }
    def = NFInstNode::InstNode::definition(ext.clone())?;
    let () = (match &*def {
        SCode::Element::EXTENDS {
            baseClassPath: __esc_base_path,
            visibility: _,
            modifications: __esc_smod,
            ann: _,
            info: __esc_info,
        } => {
            base_path = (*__esc_base_path).clone();
            smod = (*__esc_smod).clone();
            info = (*__esc_info).clone();
            scope = NFInstNode::InstNode::parent(&ext)?;
            let (__pa1, __pa0) = ::match_deref::match_deref! { match &(Lookup::lookupBaseClassName(base_path.clone(), scope.clone(), context, info.clone())?) {
                __pa1 @ Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => (__pa1.clone(), __pa0.clone()),
                _ => return Err("pattern mismatch"),
            } };
            base_node = metamodelica::Own::own(__pa0);
            base_nodes = metamodelica::Own::own(__pa1);
            checkExtendsLoop(
                base_node.clone(),
                scope.clone(),
                base_path.clone(),
                metamodelica::AsArg::as_arg(&info),
            )?;
            checkReplaceableBaseClass(base_nodes, base_path.clone(), info.clone())?;
            if NFInstNode::InstNode::isRootClass(&scope) && SCodeUtil::isEmptyMod(metamodelica::AsArg::as_arg(&smod)) {
                base_node = InstUtil::mergeScalars(base_node, base_path.clone(), false, nameMap)?;
            }
            base_node = expand(base_node, context)?;
            ext = NFInstNode::InstNode::setNodeType(
                metamodelica::Ref::new(InstNodeType::BASE_CLASS {
                    parent: NFInstNode::InstNode::identityCell(scope),
                    definition: def,
                    ty: NFInstNode::InstNode::nodeType(&base_node)?,
                }),
                base_node.clone(),
            )?;
            if NFInstNode::InstNode::isBuiltin(&base_node)
                || Class::isBuiltin(NFInstNode::InstNode::getClass(base_node)?)?
            {
                builtinExt = ext.clone();
            }
            ()
        }
        _ => (),
    });
    Ok((ext, builtinExt))
}

pub(crate) fn checkExtendsLoop(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut path: metamodelica::Ref<Path>,
    mut info: &SourceInfo,
) -> Result<()> {
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    let () = (::match_deref::match_deref! { match &(NFInstNode::InstNode::getClass(node.clone())?) {
        Deref @ Class::EXPANDED_CLASS { elements: Deref @ ClassTree::PARTIAL_TREE { .. }, .. } => {
            Error::addSourceMessage(&(Error::EXTENDS_LOOP.clone()), list![AbsynUtil::pathString(path, literal!("."), true, false)?], info)?;
            return Err("fail")
        },
        _ => {
            parent = scope;
            while !(NFInstNode::InstNode::isTopScope(&parent)) {
                if NFInstNode::InstNode::refEqual(&parent, &node)? {
                    Error::addSourceMessage(&(Error::EXTENDS_LOOP.clone()), list![AbsynUtil::pathString(path.clone(), literal!("."), true, false)?], info)?;
                    return Err("fail");
                }
                parent = NFInstNode::InstNode::parentScope(parent, false)?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkReplaceableBaseClass(
    mut baseClasses: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut basePath: metamodelica::Ref<Path>,
    mut info: SourceInfo,
) -> Result<()> {
    let mut i: i32 = 0;
    let mut name: ArcStr;
    let mut rest: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    for mut base in &*baseClasses {
        i = i + 1;
        if SCodeUtil::isElementReplaceable(&(NFInstNode::InstNode::definition(base.clone())?))? {
            if ((baseClasses).len() as i32) > 1 {
                rest = baseClasses.clone();
                name = literal!("");
                for mut j in 1..=i - 1 {
                    name = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("."));
                        __mm_s.push_str(&*NFInstNode::InstNode::name(&((rest).head().cloned()?))?);
                        __mm_s.push_str(&*name);
                        ArcStr::from(__mm_s)
                    };
                    rest = (rest).rest()?;
                }
                name = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("<"));
                    __mm_s.push_str(&*NFInstNode::InstNode::name(&((rest).head().cloned()?))?);
                    __mm_s.push_str(&*literal!(">"));
                    __mm_s.push_str(&*name);
                    ArcStr::from(__mm_s)
                };
                rest = (rest).rest()?;
                for mut n in &*rest {
                    name = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&n))?);
                        __mm_s.push_str(&*literal!("."));
                        __mm_s.push_str(&*name);
                        ArcStr::from(__mm_s)
                    };
                }
            } else {
                name = AbsynUtil::pathString(basePath.clone(), literal!("."), true, false)?;
            }
            Error::addMultiSourceMessage(
                &(Error::REPLACEABLE_BASE_CLASS.clone()),
                &(list![NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&base))?, name]),
                &(list![
                    NFInstNode::InstNode::info(metamodelica::AsArg::as_arg(&base)),
                    info.clone()
                ]),
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

pub(crate) fn expandExternalObject(
    mut clsTree: &metamodelica::Ref<ClassTree::ClassTree>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut eo_ty: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut c: metamodelica::Ref<Class::NFClass>;
    eo_ty = makeExternalObjectType(clsTree, &node)?;
    c = metamodelica::Ref::new(Class::NFClass::PARTIAL_BUILTIN {
        ty: metamodelica::Ref::new(Type::NFType::COMPLEX {
            cls: NFInstNode::InstNode::identityCell(node.clone()),
            complexTy: eo_ty,
        }),
        elements: NFClassTree::EMPTY_FLAT().clone(),
        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
        prefixes: Class::DEFAULT_PREFIXES.clone(),
        restriction: crate::NFRestriction::interned_EXTERNAL_OBJECT(),
    });
    node = NFInstNode::InstNode::updateClass(c, node)?;
    Ok(node)
}

pub(crate) fn checkBuiltinTypeExtends(
    mut builtinExtends: &metamodelica::Ref<InstNode::InstNode>,
    mut tree: &metamodelica::Ref<ClassTree::ClassTree>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    if ClassTree::componentCount(tree) > 0 || ClassTree::extendsCount(tree) > 1 {
        Error::addSourceMessage(
            &(Error::BUILTIN_EXTENDS_INVALID_ELEMENTS.clone()),
            list![NFInstNode::InstNode::name(builtinExtends)?],
            &(NFInstNode::InstNode::info(node)),
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn makeExternalObjectType(
    mut tree: &metamodelica::Ref<ClassTree::ClassTree>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<ComplexType::NFComplexType>> {
    let mut ty: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut base_path: metamodelica::Ref<Path>;
    let mut constructor: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut destructor: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    ty = (match &**tree {
        ClassTree::PARTIAL_TREE { .. } => {
            let __range0 = var_field!((**tree).components, ClassTree::ClassTree::PARTIAL_TREE)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut comp in __range0 {
                if NFInstNode::InstNode::isComponent(&comp)? {
                    Error::addSourceMessage(
                        &(Error::EXTERNAL_OBJECT_INVALID_ELEMENT.clone()),
                        list![NFInstNode::InstNode::name(node)?, NFInstNode::InstNode::name(&comp)?],
                        &(NFInstNode::InstNode::info(&comp)),
                    )?;
                    return Err("fail");
                }
            }
            if metamodelica::arrayLength(var_field!((**tree).exts, ClassTree::ClassTree::PARTIAL_TREE).clone()) > 1 {
                let __range1 = var_field!((**tree).exts, ClassTree::ClassTree::PARTIAL_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut ext in __range1 {
                    if !metamodelica::stringEq(&(NFInstNode::InstNode::name(&ext)?), &(literal!("ExternalObject")))
                        && ClassTree::recursiveElementCount(
                            &(Class::classTree(NFInstNode::InstNode::getClass(ext.clone())?)?),
                        )? != 0
                    {
                        let __pa2 = ::match_deref::match_deref! { match &(ext.clone()) {
                            Deref @ NFInstNode::InstNode::CLASS_NODE { nodeType: Deref @ NFInstNode::InstNodeType::BASE_CLASS { definition: Deref @ SCode::Element::EXTENDS { baseClassPath: __pa2, .. }, .. }, .. } => __pa2.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        base_path = metamodelica::Own::own(__pa2);
                        Error::addSourceMessage(
                            &(Error::EXTERNAL_OBJECT_INVALID_ELEMENT.clone()),
                            list![NFInstNode::InstNode::name(node)?, {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("extends "));
                                __mm_s.push_str(&*AbsynUtil::pathString(base_path, literal!("."), true, false)?);
                                ArcStr::from(__mm_s)
                            }],
                            &(NFInstNode::InstNode::info(&ext)),
                        )?;
                        return Err("fail");
                    }
                }
            }
            let __range4 = var_field!((**tree).classes, ClassTree::ClassTree::PARTIAL_TREE)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut cls in __range4 {
                let () = (::match_deref::match_deref! { match &(NFInstNode::InstNode::name(&cls)?) {
                    Deref @ "constructor" if (SCodeUtil::isFunction(&(NFInstNode::InstNode::definition(cls.clone())?))) => {
                        checkElementNotReplaceable(cls.clone())?;
                        constructor = cls.clone();
                        ()
                    },
                    Deref @ "destructor" if (SCodeUtil::isFunction(&(NFInstNode::InstNode::definition(cls.clone())?))) => {
                        checkElementNotReplaceable(cls.clone())?;
                        destructor = cls.clone();
                        ()
                    },
                    _ => {
                        Error::addSourceMessage(&(Error::EXTERNAL_OBJECT_INVALID_ELEMENT.clone()), list![NFInstNode::InstNode::name(node)?, NFInstNode::InstNode::name(&cls)?], &(NFInstNode::InstNode::info(&cls)))?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            if NFInstNode::InstNode::isEmpty(&constructor) {
                Error::addSourceMessage(
                    &(Error::EXTERNAL_OBJECT_MISSING_STRUCTOR.clone()),
                    list![NFInstNode::InstNode::name(node)?, literal!("constructor")],
                    &(NFInstNode::InstNode::info(node)),
                )?;
                return Err("fail");
            }
            if NFInstNode::InstNode::isEmpty(&destructor) {
                Error::addSourceMessage(
                    &(Error::EXTERNAL_OBJECT_MISSING_STRUCTOR.clone()),
                    list![NFInstNode::InstNode::name(node)?, literal!("destructor")],
                    &(NFInstNode::InstNode::info(node)),
                )?;
                return Err("fail");
            }
            metamodelica::Ref::new(ComplexType::NFComplexType::EXTERNAL_OBJECT {
                constructor: NFInstNode::InstNode::scopeRef(constructor),
                destructor: NFInstNode::InstNode::scopeRef(destructor),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(ty)
}

pub(crate) fn checkElementNotReplaceable(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
    if SCodeUtil::isElementReplaceable(&(NFInstNode::InstNode::definition(node.clone())?))? {
        Error::addSourceMessage(
            &(Error::ELEMENT_REPLACEABLE_NOT_ALLOWED.clone()),
            list![NFInstNode::InstNode::name(&node)?],
            &(NFInstNode::InstNode::info(&node)),
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn expandClassDerived(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut definition: &metamodelica::Ref<SCode::ClassDef>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    let mut ext_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut prefs: metamodelica::Ref<Class::Prefixes::Prefixes>;
    let mut sattrs: SCode::Attributes;
    let mut attrs: metamodelica::Ref<Attributes::NFAttributes>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
    let mut cc_mod: metamodelica::Ref<Modifier::Modifier>;
    let mut res: metamodelica::Ref<Restriction::NFRestriction>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*definition)) {
        Deref @ SCode::ClassDef::DERIVED { typeSpec: __pa0, attributes: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    sattrs = metamodelica::Own::own(__pa1);
    let __pa2 = ::match_deref::match_deref! { match &(Lookup::lookupBaseClassName(AbsynUtil::typeSpecPath(&ty), NFInstNode::InstNode::parent(&node)?, context, info.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: _ } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ext_node = metamodelica::Own::own(__pa2);
    if referenceEq(&*(&*ext_node), &*(&*node)) {
        Error::addSourceMessage(
            &(Error::RECURSIVE_SHORT_CLASS_DEFINITION.clone()),
            list![NFInstNode::InstNode::name(&node)?, Dump::unparseTypeSpec(ty.clone())?],
            &info,
        )?;
        return Err("fail");
    }
    ext_node = expand(ext_node, context)?;
    ext_node = NFInstNode::InstNode::clone(ext_node)?;
    cls = NFInstNode::InstNode::getClass(node.clone())?;
    prefs = Class::getPrefixes(cls.clone())?;
    if !(Class::Prefixes::isPartial(&prefs)) && NFInstNode::InstNode::isPartial(&ext_node)? {
        assign_field!(prefs.partialPrefix = openmodelica_frontend_types::SCode::Partial::PARTIAL);
    }
    attrs = Attributes::fromDerivedSCode(&sattrs);
    dims = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
        for mut d in (AbsynUtil::typeSpecDimensions(&ty)).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Dimension::NFDimension::RAW_DIM {
                dim: d.clone(),
                scope: NFInstNode::InstNode::scopeRef(NFInstNode::InstNode::parent(&node)?),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    r#mod = Class::getModifier(&cls);
    cc_mod = Class::getCCModifier(&cls);
    res = Restriction::fromSCode(&(SCodeUtil::getClassRestriction(element)?));
    cls = metamodelica::Ref::new(Class::NFClass::EXPANDED_DERIVED {
        baseClass: ext_node,
        modifier: r#mod,
        ccMod: cc_mod,
        dims: metamodelica::arrayFromVec(dims.into_iter().cloned().collect()),
        prefixes: prefs,
        attributes: attrs,
        restriction: res,
    });
    node = NFInstNode::InstNode::updateClass(cls, node)?;
    Ok(node)
}

pub(crate) fn expandClassDerivedComplex(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut definition: &metamodelica::Ref<SCode::ClassDef>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut ty_path: metamodelica::Ref<Path>;
    let mut prefs: metamodelica::Ref<Class::Prefixes::Prefixes>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut res: metamodelica::Ref<Restriction::NFRestriction>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let __pa0 = ::match_deref::match_deref! { match &((*definition)) {
        Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ty_path = metamodelica::Own::own(__pa0);
    ty = (::match_deref::match_deref! { match &(ty_path.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" } => metamodelica::Ref::new(Type::NFType::POLYMORPHIC { name: NFInstNode::InstNode::name(&node)? }),
        _ => {
            Error::addSourceMessage(&(Error::LOOKUP_BASECLASS_ERROR.clone()), list![AbsynUtil::pathString(ty_path, literal!("."), true, false)?, NFInstNode::InstNode::scopeName(&node)?], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cls = NFInstNode::InstNode::getClass(node.clone())?;
    prefs = Class::getPrefixes(cls)?;
    res = Restriction::fromSCode(&(SCodeUtil::getClassRestriction(element)?));
    cls = metamodelica::Ref::new(Class::NFClass::PARTIAL_BUILTIN {
        ty: ty,
        elements: NFClassTree::EMPTY().clone(),
        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
        prefixes: prefs,
        restriction: res,
    });
    node = NFInstNode::InstNode::updateClass(cls, node)?;
    Ok(node)
}

pub fn instClass(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut modifier: metamodelica::Ref<Modifier::Modifier>,
    mut attributes: metamodelica::Ref<Attributes::NFAttributes>,
    mut useBinding: bool,
    mut instLevel: i32,
    mut typeConfidence: i32,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<Attributes::NFAttributes>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut attributes: metamodelica::Ref<Attributes::NFAttributes> = attributes;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut outer_mod: metamodelica::Ref<Modifier::Modifier>;
    Error::checkCancel()?;
    cls = NFInstNode::InstNode::getClass(node.clone())?;
    outer_mod = Class::getModifier(&cls);
    if Modifier::hasBinding(&outer_mod) {
        Error::addSourceMessage(
            &(Error::MISSING_REDECLARE_IN_CLASS_MOD.clone()),
            list![NFInstNode::InstNode::name(&node)?],
            &(Binding::getInfo(&(Modifier::binding(&outer_mod)))),
        )?;
        return Err("fail");
    }
    (attributes, node) = instClassDef(
        cls,
        modifier,
        attributes,
        useBinding,
        node,
        parent,
        instLevel,
        typeConfidence,
        context,
    )?;
    Ok((node, attributes))
}

pub(crate) fn instClassDef(
    mut cls: metamodelica::Ref<Class::NFClass>,
    mut outerMod: metamodelica::Ref<Modifier::Modifier>,
    mut attributes: metamodelica::Ref<Attributes::NFAttributes>,
    mut useBinding: bool,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut instLevel: i32,
    mut typeConfidence: i32,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<Attributes::NFAttributes>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut attributes: metamodelica::Ref<Attributes::NFAttributes> = attributes;
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut par: metamodelica::Ref<InstNode::InstNode>;
    let mut base_node: metamodelica::Ref<InstNode::InstNode>;
    let mut inst_cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
    let mut outer_mod: metamodelica::Ref<Modifier::Modifier>;
    let mut res: metamodelica::Ref<Restriction::NFRestriction>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut attrs: metamodelica::Ref<Attributes::NFAttributes>;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { restriction: __esc_res, ccMod: __cls_ccMod, modifier: __cls_modifier, .. } => {
            res = (*__esc_res).clone();
            if NFInstNode::InstNode::isBaseClass(&node) {
                par = parent.clone();
            } else {
                (node, par, _, _) = ClassTree::instantiate(node, parent.clone(), &(crate::NFInstNode::InstNode::interned_EMPTY_NODE()))?;
            }
            updateComponentType(parent, node.clone())?;
            attributes = Attributes::updateClassConnectorType(metamodelica::AsArg::as_arg(&res), attributes);
            let (__pa1, __pa0) = ::match_deref::match_deref! { match &(NFInstNode::InstNode::getClass(node.clone())?) {
                __pa1 @ Deref @ Class::EXPANDED_CLASS { elements: __pa0, .. } => (__pa1.clone(), __pa0.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cls_tree = metamodelica::Own::own(__pa0);
            inst_cls = metamodelica::Own::own(__pa1);
            r#mod = instElementModifier(&(NFInstNode::InstNode::definition(node.clone())?), &(node.clone()), par.clone(), typeConfidence)?;
            r#mod = Modifier::propagate(r#mod, node.clone(), par.clone())?;
            r#mod = Modifier::merge(r#mod, __cls_ccMod.clone(), &(literal!("")))?;
            outer_mod = Modifier::propagate(__cls_modifier.clone(), node.clone(), par.clone())?;
            r#mod = Modifier::merge(outer_mod, r#mod, &(literal!("")))?;
            r#mod = markTypeModifier(r#mod, metamodelica::AsArg::as_arg(&res), typeConfidence)?;
            r#mod = Modifier::merge(outerMod, r#mod, &(literal!("")))?;
            ClassTree::mapExtends(&cls_tree, &({ let __pe_b1 = par.clone(); let __pe_b2 = context; let __pe_b3 = instLevel; move |__pe_a0| modifyExtends(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            ClassTree::mapExtends(&cls_tree, &({ let __pe_b1 = ExtendsVisibility::PUBLIC.clone(); move |__pe_a0| applyExtendsVisibility(__pe_a0, __pe_b1.clone()) }))?;
            applyModifier(&r#mod, cls_tree.clone(), &node, context)?;
            ClassTree::mapRedeclareChains(&cls_tree, (std::sync::Arc::new({ let __pe_b1 = instLevel; let __pe_b2 = context; move |__pe_a0| redeclareElements(&__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>) -> Result<()> + 'static>))?;
            redeclareClasses(cls_tree.clone(), par, context, instLevel)?;
            ClassTree::mapExtends(&cls_tree, &({ let __pe_b1 = attributes.clone(); let __pe_b2 = useBinding; let __pe_b3 = instLevel; let __pe_b4 = context; move |__pe_a0| instExtends(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
            ClassTree::applyLocalComponents(&cls_tree, &({ let __pe_b1 = attributes.clone(); let __pe_b2 = crate::NFModifier::Modifier::interned_NOMOD(); let __pe_b3 = useBinding; let __pe_b4 = instLevel + 1; let __pe_b5 = context; let __pe_b6 = None; let __pe_b7 = metamodelica::nil(); move |__pe_a0| instComponent(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), &__pe_b7) }))?;
            cls_tree = ClassTree::replaceDuplicates(cls_tree)?;
            ClassTree::checkDuplicates(cls_tree.clone())?;
            NFInstNode::InstNode::updateClass(Class::setClassTree(cls_tree, inst_cls)?, node.clone())?;
            Restriction::checkClass(node.clone(), metamodelica::AsArg::as_arg(&res), context)?;
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { attributes: __cls_attributes, ccMod: __cls_ccMod, modifier: __cls_modifier, restriction: __cls_restriction, .. } => {
            (node, par, _, _) = ClassTree::instantiate(node, parent.clone(), &(crate::NFInstNode::InstNode::interned_EMPTY_NODE()))?;
            node = NFInstNode::InstNode::setNodeType(metamodelica::Ref::new(InstNodeType::DERIVED_CLASS { ty: NFInstNode::InstNode::nodeType(&node)? }), node)?;
            let __pa0 = ::match_deref::match_deref! { match &(NFInstNode::InstNode::getClass(node.clone())?) {
                Deref @ Class::EXPANDED_DERIVED { baseClass: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            base_node = metamodelica::Own::own(__pa0);
            r#mod = instElementModifier(&(NFInstNode::InstNode::definition(node.clone())?), &(node.clone()), NFInstNode::InstNode::rootParent(node.clone())?, typeConfidence)?;
            r#mod = Modifier::propagate(r#mod, node.clone(), par.clone())?;
            r#mod = Modifier::merge(r#mod, __cls_ccMod.clone(), &(literal!("")))?;
            outer_mod = Modifier::propagate(__cls_modifier.clone(), node.clone(), par.clone())?;
            r#mod = Modifier::merge(outer_mod, r#mod, &(literal!("")))?;
            r#mod = markTypeModifier(r#mod, metamodelica::AsArg::as_arg(&__cls_restriction), typeConfidence)?;
            r#mod = Modifier::merge(outerMod, r#mod, &(literal!("")))?;
            attrs = Attributes::updateClassConnectorType(metamodelica::AsArg::as_arg(&__cls_restriction), __cls_attributes.clone());
            attributes = Attributes::mergeDerivedAttributes(attrs, attributes, &parent)?;
            (base_node, attributes) = instClass(base_node, r#mod, attributes, useBinding, instLevel, typeConfidence, par, context)?;
            assign_variant_field!(cls => Class::NFClass::EXPANDED_DERIVED;
                baseClass = base_node,
                attributes = attributes.clone(),
                dims = metamodelica::arrayFromVec(var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone().borrow().clone())
            );
            node = NFInstNode::InstNode::updateClass(cls, node)?;
            updateComponentType(parent, node.clone())?;
            ()
        },
        Deref @ Class::PARTIAL_BUILTIN { restriction: Deref @ Restriction::EXTERNAL_OBJECT, elements: __cls_elements, ty: __cls_ty, .. } => {
            inst_cls = metamodelica::Ref::new(Class::NFClass::INSTANCED_BUILTIN { ty: __cls_ty.clone(), elements: __cls_elements.clone(), restriction: var_field!((*cls).restriction, Class::NFClass::PARTIAL_BUILTIN).clone() });
            applyModifier(&outerMod, __cls_elements.clone(), &node, context)?;
            node = NFInstNode::InstNode::replaceClass(inst_cls, node)?;
            updateComponentType(parent.clone(), node.clone())?;
            instExternalObjectStructors(metamodelica::AsArg::as_arg(&__cls_ty), &parent, context)?;
            ()
        },
        Deref @ Class::PARTIAL_BUILTIN { ty: __esc_ty, restriction: __esc_res, modifier: __cls_modifier, .. } => {
            ty = (*__esc_ty).clone();
            res = (*__esc_res).clone();
            (node, par, _, _) = ClassTree::instantiate(node, parent.clone(), &(crate::NFInstNode::InstNode::interned_EMPTY_NODE()))?;
            updateComponentType(parent, node.clone())?;
            cls_tree = Class::classTree(NFInstNode::InstNode::getClass(node.clone())?)?;
            r#mod = instElementModifier(&(NFInstNode::InstNode::definition(node.clone())?), &(node.clone()), NFInstNode::InstNode::parent(&node)?, typeConfidence)?;
            r#mod = Modifier::merge(__cls_modifier.clone(), r#mod, &(literal!("")))?;
            r#mod = markTypeModifier(r#mod, &(crate::NFRestriction::interned_TYPE()), typeConfidence)?;
            r#mod = Modifier::merge(outerMod, r#mod, &(literal!("")))?;
            applyModifier(&r#mod, cls_tree.clone(), &node, context)?;
            inst_cls = metamodelica::Ref::new(Class::NFClass::INSTANCED_BUILTIN { ty: ty.clone(), elements: cls_tree, restriction: res.clone() });
            node = NFInstNode::InstNode::updateClass(inst_cls, node)?;
            ()
        },
        Deref @ Class::INSTANCED_CLASS { .. } => {
            node = NFInstNode::InstNode::replaceClass(crate::NFClass::interned_NOT_INSTANTIATED(), node)?;
            node = NFInstNode::InstNode::reidentify(node);
            node = NFInstNode::InstNode::setNodeType(crate::NFInstNode::InstNodeType::interned_NORMAL_CLASS(), node)?;
            node = expand(node, context)?;
            (node, _) = instClass(node, outerMod, attributes.clone(), useBinding, instLevel, typeConfidence, parent.clone(), context)?;
            updateComponentType(parent, node.clone())?;
            ()
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instClassDef")); __mm_s.push_str(&*literal!(" got unknown class.")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((attributes, node))
}

pub(crate) fn updateComponentType(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut component: metamodelica::Ref<InstNode::InstNode> = component;
    if NFInstNode::InstNode::isComponent(&component)? {
        component = NFInstNode::InstNode::componentApply(component, &Component::setClassInstance, cls)?;
    }
    Ok(component)
}

pub(crate) fn instExternalObjectStructors(
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut parent: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<()> {
    let mut constructor: metamodelica::Ref<InstNode::InstNode>;
    let mut destructor: metamodelica::Ref<InstNode::InstNode>;
    let mut par: metamodelica::Ref<InstNode::InstNode>;
    let mut con_ref: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut de_ref: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut info: SourceInfo;
    par = NFInstNode::InstNode::parent(&(NFInstNode::InstNode::parent(parent)?))?;
    if !(NFInstNode::InstNode::isClass(&par)? && Class::isExternalObject(&(NFInstNode::InstNode::getClass(par)?))) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*ty)) {
            Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { constructor: __pa0, destructor: __pa1 }, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        con_ref = metamodelica::Own::own(__pa0);
        de_ref = metamodelica::Own::own(__pa1);
        constructor = NFInstNode::InstNode::borrow(con_ref)?;
        destructor = NFInstNode::InstNode::borrow(de_ref)?;
        info = NFInstNode::InstNode::info(parent);
        Function::instFunctionNode(constructor, context, info.clone())?;
        Function::instFunctionNode(destructor, context, info)?;
    }
    Ok(())
}

pub(crate) fn instPackage(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    use crate::NFInstNode::PackageCacheState;
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut cache: metamodelica::Ref<CachedData::CachedData>;
    let mut inst: metamodelica::Ref<InstNode::InstNode>;
    let mut state: PackageCacheState;
    let mut alias: Option<metamodelica::Ref<InstNode::InstNode>>;
    cache = NFInstNode::InstNode::getPackageCache(&node)?;
    (inst, state) = (match &*cache {
        NFInstNode::CachedData::PACKAGE {
            instance: __cache_instance,
            state: __cache_state,
        } => (
            NFInstNode::InstNode::fromHandle(metamodelica::AsArg::as_arg(&__cache_instance))?,
            __cache_state.clone(),
        ),
        _ => (node.clone(), PackageCacheState::NOT_INITIALIZED.clone()),
    });
    if state == PackageCacheState::INSTANTIATED.clone() {
        node = inst;
        return Ok(node);
    }
    if state == PackageCacheState::PROCESSING.clone() {
        node = inst;
        return Ok(node);
    }
    if state == PackageCacheState::NOT_INITIALIZED.clone() {
        alias = packageAlias(node.clone(), context)?;
        if (alias).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(alias) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            inst = metamodelica::Own::own(__pa0);
            NFInstNode::InstNode::setPackageCache(node, inst.clone(), PackageCacheState::INSTANTIATED.clone())?;
            node = inst;
            return Ok(node);
        }
    }
    if state < PackageCacheState::PARTIALLY_INSTANTIATED.clone() {
        NFInstNode::InstNode::setPackageCache(node.clone(), node.clone(), PackageCacheState::PROCESSING.clone())?;
        inst = instantiate(
            node.clone(),
            crate::NFModifier::Modifier::interned_NOMOD(),
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
            context,
            false,
        )?;
        NFInstNode::InstNode::setPackageCache(
            node.clone(),
            inst.clone(),
            PackageCacheState::PARTIALLY_INSTANTIATED.clone(),
        )?;
    }
    if state < PackageCacheState::INSTANTIATED.clone()
        && !(InstContext::inFastLookup(context))
        && (!(NFInstNode::InstNode::isPartial(&inst)?) || InstContext::inRelaxed(context))
    {
        NFInstNode::InstNode::setPackageCache(node, inst.clone(), PackageCacheState::INSTANTIATED.clone())?;
        instExpressions(
            inst.clone(),
            &(inst.clone()),
            crate::NFSections::interned_EMPTY(),
            &(ConnectBreakTree::new()),
            context,
            &(DEFAULT_SETTINGS.clone()),
        )?;
    }
    node = inst;
    Ok(node)
}

pub(crate) fn packageAlias(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<Option<metamodelica::Ref<InstNode::InstNode>>> {
    let mut aliasInst: Option<metamodelica::Ref<InstNode::InstNode>>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut path: metamodelica::Ref<Path>;
    let mut base: metamodelica::Ref<InstNode::InstNode>;
    aliasInst = (::match_deref::match_deref! { match &((NFInstNode::InstNode::definition(node.clone())?, NFInstNode::InstNode::getClass(node.clone())?)) {
        (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE { .. }, partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_path, arrayDim: None }, modifications: Deref @ SCode::Mod::NOMOD { .. }, .. }, .. }, cls @ Deref @ Class::PARTIAL_CLASS { .. }) if (Modifier::isEmpty(var_field!((**cls).modifier, Class::NFClass::PARTIAL_CLASS)) && Modifier::isEmpty(var_field!((**cls).ccMod, Class::NFClass::PARTIAL_CLASS))) => {
            path = (*__esc_path).clone();
            let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupBaseClassName(path.clone(), NFInstNode::InstNode::parent(&node)?, context, NFInstNode::InstNode::info(&node))?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            base = metamodelica::Own::own(__pa0);
            if (isAliasablePackage(&base) && !(referenceEq(&*(&*base),&*(node)))) {Some(instPackage(base, context)?)} else {None}
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(aliasInst)
}

pub(crate) fn isAliasablePackage(mut node: &metamodelica::Ref<InstNode::InstNode>) -> bool {
    let mut aliasable: bool;
    aliasable = (::match_deref::match_deref! { match node {
        Deref @ NFInstNode::InstNode::CLASS_NODE { definition: Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE { .. }, partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    aliasable
}

pub(crate) fn modifyExtends(
    mut extendsNode: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut instLevel: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut extendsNode: metamodelica::Ref<InstNode::InstNode> = extendsNode;
    let mut elem: metamodelica::Ref<SCode::Element>;
    let mut ext_mod: metamodelica::Ref<Modifier::Modifier>;
    let mut ext_node: metamodelica::Ref<InstNode::InstNode>;
    let mut info: SourceInfo;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    cls = NFInstNode::InstNode::getClass(extendsNode.clone())?;
    cls_tree = Class::classTree(cls.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(NFInstNode::InstNode::nodeType(&extendsNode)?) {
        Deref @ NFInstNode::InstNodeType::BASE_CLASS { definition: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    elem = metamodelica::Own::own(__pa0);
    ext_mod = Modifier::fromElement(&elem, scope.clone(), instLevel)?;
    ext_mod = Modifier::merge(
        NFInstNode::InstNode::getModifier(&extendsNode),
        ext_mod,
        &(literal!("")),
    )?;
    if !(Class::isBuiltin(cls)?) {
        ClassTree::mapExtends(
            &cls_tree,
            &({
                let __pe_b1 = extendsNode.clone();
                let __pe_b2 = context;
                let __pe_b3 = instLevel;
                move |__pe_a0| modifyExtends(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
            }),
        )?;
        let () = (match &*elem {
            SCode::Element::EXTENDS {
                baseClassPath: __elem_baseClassPath,
                info: __elem_info,
                ..
            } => {
                let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupBaseClassName(__elem_baseClassPath.clone(), scope, context, __elem_info.clone())?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                ext_node = metamodelica::Own::own(__pa0);
                if !(referenceEq(
                    &*(NFInstNode::InstNode::definition(extendsNode.clone())?),
                    &*(NFInstNode::InstNode::definition(ext_node.clone())?),
                )) && !(Flags::isSet(Flags::MERGE_COMPONENTS.clone())?)
                {
                    Error::addMultiSourceMessage(
                        &(Error::FOUND_OTHER_BASECLASS.clone()),
                        &(list![AbsynUtil::pathString(
                            __elem_baseClassPath.clone(),
                            literal!("."),
                            true,
                            false
                        )?]),
                        &(list![
                            NFInstNode::InstNode::info(&extendsNode),
                            NFInstNode::InstNode::info(&ext_node)
                        ]),
                    )?;
                    return Err("fail");
                }
                ()
            }
            SCode::Element::CLASS { .. } => (),
            _ => return Err("match: no arm matched"),
        });
    }
    applyModifier(&ext_mod, cls_tree, &extendsNode, context)?;
    Ok(extendsNode)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ExtendsVisibility {
    PUBLIC = 1,
    DERIVED_PROTECTED = 2,
    PROTECTED = 3,
}
impl PartialOrd for ExtendsVisibility {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ExtendsVisibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ExtendsVisibility {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn applyExtendsVisibility(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut visibility: ExtendsVisibility,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut vis: ExtendsVisibility = visibility;
    cls = NFInstNode::InstNode::getClass(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { elements: __esc_cls_tree @ Deref @ ClassTree::INSTANTIATED_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            if vis == ExtendsVisibility::PUBLIC.clone() && NFInstNode::InstNode::isProtectedBaseClass(&node) || vis == ExtendsVisibility::DERIVED_PROTECTED.clone() {
                vis = ExtendsVisibility::PROTECTED.clone();
            }
            if vis == ExtendsVisibility::PROTECTED.clone() && visibility != ExtendsVisibility::PROTECTED.clone() {
                let __range0 = var_field!((*cls_tree).classes, ClassTree::ClassTree::INSTANTIATED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range0 {
                    Mutable::update(c.clone(), NFInstNode::InstNode::protectClass(Mutable::access(c)));
                }
                let __range1 = var_field!((*cls_tree).components, ClassTree::ClassTree::INSTANTIATED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range1 {
                    Mutable::update(c.clone(), NFInstNode::InstNode::protectComponent(Mutable::access(c)));
                }
            }
            ClassTree::mapExtends(metamodelica::AsArg::as_arg(&cls_tree), &({ let __pe_b1 = vis; move |__pe_a0| applyExtendsVisibility(__pe_a0, __pe_b1.clone()) }))?;
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { baseClass: __cls_baseClass, .. } => {
            if vis == ExtendsVisibility::PUBLIC.clone() && NFInstNode::InstNode::isProtectedBaseClass(&node) {
                vis = ExtendsVisibility::DERIVED_PROTECTED.clone();
            }
            assign_variant_field!(cls => Class::NFClass::EXPANDED_DERIVED; baseClass = applyExtendsVisibility(__cls_baseClass.clone(), vis)?);
            node = NFInstNode::InstNode::updateClass(cls, node)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub(crate) fn instExtends(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut attributes: &metamodelica::Ref<Attributes::NFAttributes>,
    mut useBinding: bool,
    mut instLevel: i32,
    mut context: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut inst_cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    cls = NFInstNode::InstNode::getClass(node.clone())?;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { elements: __esc_cls_tree @ Deref @ ClassTree::INSTANTIATED_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            ClassTree::mapExtends(metamodelica::AsArg::as_arg(&cls_tree), &({ let __pe_b1 = attributes.clone(); let __pe_b2 = useBinding; let __pe_b3 = instLevel; let __pe_b4 = context; move |__pe_a0| instExtends(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
            ClassTree::applyLocalComponents(metamodelica::AsArg::as_arg(&cls_tree), &({ let __pe_b1 = attributes.clone(); let __pe_b2 = crate::NFModifier::Modifier::interned_NOMOD(); let __pe_b3 = useBinding; let __pe_b4 = instLevel; let __pe_b5 = context; let __pe_b6 = None; let __pe_b7 = metamodelica::nil(); move |__pe_a0| instComponent(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), &__pe_b7) }))?;
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { baseClass: __cls_baseClass, .. } => {
            assign_variant_field!(cls => Class::NFClass::EXPANDED_DERIVED; baseClass = instExtends(__cls_baseClass.clone(), attributes, useBinding, instLevel, context)?);
            node = NFInstNode::InstNode::updateClass(cls, node)?;
            ()
        },
        Deref @ Class::PARTIAL_BUILTIN { elements: __cls_elements, restriction: __cls_restriction, ty: __cls_ty, .. } => {
            inst_cls = metamodelica::Ref::new(Class::NFClass::INSTANCED_BUILTIN { ty: __cls_ty.clone(), elements: __cls_elements.clone(), restriction: __cls_restriction.clone() });
            node = NFInstNode::InstNode::updateClass(inst_cls, node)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub(crate) fn applyModifier(
    mut modifier: &metamodelica::Ref<Modifier::Modifier>,
    mut cls: metamodelica::Ref<ClassTree::ClassTree>,
    mut parent: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<ClassTree::ClassTree>> {
    let mut cls: metamodelica::Ref<ClassTree::ClassTree> = cls;
    let mut mods: metamodelica::List<metamodelica::Ref<Modifier::Modifier>>;
    let mut node_ptrs: metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut found: bool;
    mods = Modifier::toList(modifier);
    if (mods).is_empty() {
        return Ok(cls);
    }
    let () = (match &*cls {
        ClassTree::FLAT_TREE { .. } => {
            for mut r#mod in &*mods {
                if '__try0: {
                    (node, _) = unwrap_break_err!(ClassTree::lookupElement(unwrap_break_err!(Modifier::name(metamodelica::AsArg::as_arg(&r#mod)), '__try0), &cls), '__try0);
                    unwrap_break_err!(NFInstNode::InstNode::componentApply(node.clone(), &Component::mergeModifier, r#mod.clone()), '__try0);
                    Ok::<(), &'static str>(())
                }.is_err() {
                    Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![Modifier::name(metamodelica::AsArg::as_arg(&r#mod))?, NFInstNode::InstNode::name(parent)?], &(Modifier::info(metamodelica::AsArg::as_arg(&r#mod))))?;
                    if !(InstContext::inInstanceAPI(context)) {
                        return Err("fail");
                    }
                }
            }
            ()
        }
        _ => {
            for mut r#mod in &*mods {
                match '__try0: {
                    node_ptrs = unwrap_break_err!(ClassTree::lookupElementsPtr(unwrap_break_err!(Modifier::name(metamodelica::AsArg::as_arg(&r#mod)), '__try0), &cls), '__try0);
                    Ok::<_, &'static str>((node_ptrs.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        node_ptrs = __try0_o0;
                    }
                    Err(_) => {
                        node_ptrs = metamodelica::nil();
                    }
                }
                found = false;
                for mut node_ptr in &*node_ptrs {
                    node = Mutable::access(node_ptr.clone());
                    if NFInstNode::InstNode::isEmpty(&node) {
                        continue;
                    }
                    found = true;
                    node = NFInstNode::InstNode::resolveOuter(node);
                    if NFInstNode::InstNode::isProtected(&node)
                        && !(NFInstNode::InstNode::isExtends(parent) || NFInstNode::InstNode::isBaseClass(parent))
                    {
                        Error::addMultiSourceMessage(
                            &(Error::NF_MODIFY_PROTECTED.clone()),
                            &(list![
                                NFInstNode::InstNode::name(&node)?,
                                Modifier::toString(metamodelica::AsArg::as_arg(&r#mod), true)?
                            ]),
                            &(list![
                                Modifier::info(metamodelica::AsArg::as_arg(&r#mod)),
                                NFInstNode::InstNode::info(&node)
                            ]),
                        )?;
                        if InstContext::inInstanceAPI(context) {
                            continue;
                        } else {
                            return Err("fail");
                        }
                    }
                    if NFInstNode::InstNode::isOnlyOuter(&node)? {
                        Error::addSourceMessage(
                            &(Error::OUTER_ELEMENT_MOD.clone()),
                            list![
                                Modifier::toString(metamodelica::AsArg::as_arg(&r#mod), false)?,
                                Modifier::name(metamodelica::AsArg::as_arg(&r#mod))?
                            ],
                            &(Modifier::info(metamodelica::AsArg::as_arg(&r#mod))),
                        )?;
                        if InstContext::inInstanceAPI(context) {
                            continue;
                        } else {
                            return Err("fail");
                        }
                    }
                    if NFInstNode::InstNode::isComponent(&node)? {
                        NFInstNode::InstNode::componentApply(node, &Component::mergeModifier, r#mod.clone())?;
                    } else {
                        partialInstClass(node.clone())?;
                        node = NFInstNode::InstNode::replaceClass(
                            Class::mergeModifier(r#mod.clone(), NFInstNode::InstNode::getClass(node.clone())?)?,
                            node,
                        )?;
                        node = NFInstNode::InstNode::clearPackageCache(node)?;
                        Mutable::update(node_ptr.clone(), node);
                    }
                }
                if !(found) && !(InstContext::inInstanceAPI(context)) {
                    Error::addSourceMessage(
                        &(Error::MISSING_MODIFIED_ELEMENT.clone()),
                        list![
                            Modifier::name(metamodelica::AsArg::as_arg(&r#mod))?,
                            NFInstNode::InstNode::name(parent)?
                        ],
                        &(Modifier::info(metamodelica::AsArg::as_arg(&r#mod))),
                    )?;
                    return Err("fail");
                }
            }
            ()
        }
    });
    Ok(cls)
}

pub(crate) fn redeclareClasses(
    mut tree: metamodelica::Ref<ClassTree::ClassTree>,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut instLevel: i32,
) -> Result<metamodelica::Ref<ClassTree::ClassTree>> {
    let mut tree: metamodelica::Ref<ClassTree::ClassTree> = tree;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut redecl_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
    let mut cc_mod: metamodelica::Ref<Modifier::Modifier>;
    let () = (match &*tree {
        ClassTree::INSTANTIATED_TREE { .. } => {
            let __range0 = var_field!((*tree).classes, ClassTree::ClassTree::INSTANTIATED_TREE)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut cls_ptr in __range0 {
                cls_node = Mutable::access(cls_ptr.clone());
                cls = NFInstNode::InstNode::getClass(NFInstNode::InstNode::resolveOuter(cls_node.clone()))?;
                r#mod = Class::getModifier(&cls);
                if Modifier::isRedeclare(&r#mod) {
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(r#mod) {
                        Deref @ Modifier::REDECLARE { element: __pa1, outerMod: __pa2, constrainingMod: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    redecl_node = metamodelica::Own::own(__pa1);
                    r#mod = metamodelica::Own::own(__pa2);
                    cc_mod = metamodelica::Own::own(__pa3);
                    cc_mod = getConstrainingMod(
                        &(NFInstNode::InstNode::definition(cls_node.clone())?),
                        parent.clone(),
                        cc_mod,
                        instLevel,
                    )?;
                    cls_node = redeclareClass(redecl_node, cls_node, r#mod, cc_mod, instLevel, context)?;
                    Mutable::update(cls_ptr, cls_node);
                }
            }
            ()
        }
        _ => (),
    });
    Ok(tree)
}

pub(crate) fn redeclareElements(
    mut chain: &metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
    mut instLevel: i32,
    mut context: i32,
) -> Result<()> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut node_ptr: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
    node = Mutable::access((chain).head().cloned()?);
    node_ptr = (chain).head().cloned()?;
    if NFInstNode::InstNode::isClass(&node)? {
        for mut cls_ptr in &*(chain).rest()? {
            node_ptr = redeclareClassElement(cls_ptr.clone(), node_ptr, instLevel, context)?;
        }
        node = Mutable::access(node_ptr);
    } else {
        for mut comp_ptr in &*(chain).rest()? {
            node_ptr = redeclareComponentElement(comp_ptr.clone(), node_ptr, instLevel, context)?;
        }
        node = Mutable::access(node_ptr);
    }
    for mut cls_ptr in &**chain {
        Mutable::update(cls_ptr.clone(), node.clone());
    }
    Ok(())
}

pub(crate) fn redeclareClassElement(
    mut redeclareCls: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>,
    mut replaceableCls: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>,
    mut instLevel: i32,
    mut context: i32,
) -> Result<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>> {
    let mut outCls: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
    let mut rdcl_node: metamodelica::Ref<InstNode::InstNode>;
    let mut repl_node: metamodelica::Ref<InstNode::InstNode>;
    rdcl_node = Mutable::access(redeclareCls);
    repl_node = Mutable::access(replaceableCls);
    rdcl_node = redeclareClass(
        rdcl_node,
        repl_node,
        crate::NFModifier::Modifier::interned_NOMOD(),
        crate::NFModifier::Modifier::interned_NOMOD(),
        instLevel,
        context,
    )?;
    outCls = Mutable::create(rdcl_node);
    Ok(outCls)
}

pub(crate) fn redeclareComponentElement(
    mut redeclareComp: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>,
    mut replaceableComp: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>,
    mut instLevel: i32,
    mut context: i32,
) -> Result<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>> {
    let mut outComp: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
    let mut rdcl_node: metamodelica::Ref<InstNode::InstNode>;
    let mut repl_node: metamodelica::Ref<InstNode::InstNode>;
    rdcl_node = Mutable::access(redeclareComp);
    repl_node = Mutable::access(replaceableComp);
    instComponent(
        repl_node.clone(),
        &(Attributes::DEFAULT_ATTR().clone()),
        crate::NFModifier::Modifier::interned_NOMOD(),
        true,
        instLevel,
        context,
        None,
        &(metamodelica::nil()),
    )?;
    redeclareComponent(
        rdcl_node.clone(),
        repl_node,
        &(crate::NFModifier::Modifier::interned_NOMOD()),
        crate::NFModifier::Modifier::interned_NOMOD(),
        &(metamodelica::nil()),
        &(Attributes::DEFAULT_ATTR().clone()),
        rdcl_node.clone(),
        instLevel,
        context,
    )?;
    outComp = Mutable::create(rdcl_node);
    Ok(outComp)
}

pub(crate) fn redeclareClass(
    mut redeclareNode: metamodelica::Ref<InstNode::InstNode>,
    mut originalNode: metamodelica::Ref<InstNode::InstNode>,
    mut outerMod: metamodelica::Ref<Modifier::Modifier>,
    mut constrainingMod: metamodelica::Ref<Modifier::Modifier>,
    mut instLevel: i32,
    mut context: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut redeclaredNode: metamodelica::Ref<InstNode::InstNode>;
    let mut orig_node: metamodelica::Ref<InstNode::InstNode>;
    let mut orig_cls: metamodelica::Ref<Class::NFClass>;
    let mut rdcl_cls: metamodelica::Ref<Class::NFClass>;
    let mut new_cls: metamodelica::Ref<Class::NFClass>;
    let mut prefs: metamodelica::Ref<Class::Prefixes::Prefixes>;
    let mut node_ty: metamodelica::Ref<InstNodeType>;
    let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
    let mut orig_opt: Option<metamodelica::Ref<InstNode::InstNode>>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    if !(NFInstNode::InstNode::isClass(&redeclareNode)?) {
        Error::addMultiSourceMessage(
            &(Error::INVALID_REDECLARE_AS.clone()),
            &(list![
                NFInstNode::InstNode::typeName(&originalNode)?,
                NFInstNode::InstNode::name(&originalNode)?,
                NFInstNode::InstNode::typeName(&redeclareNode)?
            ]),
            &(list![
                NFInstNode::InstNode::info(&redeclareNode),
                NFInstNode::InstNode::info(&originalNode)
            ]),
        )?;
        return Err("fail");
    }
    partialInstClass(originalNode.clone())?;
    orig_cls = NFInstNode::InstNode::getClass(originalNode.clone())?;
    partialInstClass(redeclareNode.clone())?;
    rdcl_cls = NFInstNode::InstNode::getClass(redeclareNode.clone())?;
    r#mod = Class::getModifier(&rdcl_cls);
    r#mod = Modifier::merge(outerMod, r#mod, &(literal!("")))?;
    prefs = Attributes::mergeRedeclaredClassPrefixes(
        &(Class::getPrefixes(orig_cls.clone())?),
        Class::getPrefixes(rdcl_cls.clone())?,
        &redeclareNode,
    )?;
    if SCodeUtil::isClassExtends(&(NFInstNode::InstNode::definition(redeclareNode.clone())?)) {
        orig_node = expand(originalNode.clone(), context)?;
        orig_cls = NFInstNode::InstNode::getClass(orig_node.clone())?;
        new_cls = (match &*rdcl_cls {
            Class::PARTIAL_CLASS { .. } if (Class::isBuiltin(orig_cls.clone())?) => {
                if !(SCodeUtil::isEmptyClassDef(
                    &(SCodeUtil::getClassDef(&(NFInstNode::InstNode::definition(redeclareNode.clone())?))?),
                )) {
                    Error::addSourceMessage(
                        &(Error::BUILTIN_EXTENDS_INVALID_ELEMENTS.clone()),
                        list![NFInstNode::InstNode::name(&redeclareNode)?],
                        &(NFInstNode::InstNode::info(&redeclareNode)),
                    )?;
                    return Err("fail");
                }
                Class::setPrefixes(prefs, orig_cls.clone())?
            }
            Class::PARTIAL_CLASS {
                elements: __rdcl_cls_elements,
                ..
            } => {
                node_ty = metamodelica::Ref::new(InstNodeType::BASE_CLASS {
                    parent: NFInstNode::InstNode::identityCell(NFInstNode::InstNode::parent(&orig_node)?),
                    definition: NFInstNode::InstNode::definition(orig_node.clone())?,
                    ty: NFInstNode::InstNode::nodeType(&orig_node)?,
                });
                orig_node = NFInstNode::InstNode::setNodeType(node_ty, orig_node)?;
                cls_tree = ClassTree::setClassExtends(orig_node, __rdcl_cls_elements.clone())?;
                metamodelica::Ref::new(Class::NFClass::PARTIAL_CLASS {
                    elements: cls_tree,
                    modifier: r#mod,
                    ccMod: constrainingMod,
                    prefixes: prefs,
                })
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInst.redeclareClass"));
                        __mm_s.push_str(&*literal!(" got unknown classes"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")),
                )?;
                return Err("fail");
            }
        });
    } else {
        new_cls = (::match_deref::match_deref! { match &((orig_cls.clone(), rdcl_cls.clone())) {
            (Deref @ Class::PARTIAL_BUILTIN { .. }, _) => redeclareEnum(rdcl_cls, &orig_cls, prefs, r#mod, redeclareNode.clone(), &originalNode, context)?,
            (_, Deref @ Class::PARTIAL_CLASS { .. }) => metamodelica::Ref::new(Class::NFClass::PARTIAL_CLASS { elements: var_field!((*rdcl_cls).elements, Class::NFClass::PARTIAL_CLASS).clone(), modifier: r#mod, ccMod: constrainingMod, prefixes: prefs }),
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.redeclareClass")); __mm_s.push_str(&*literal!(" got unknown classes")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    orig_opt = if (InstContext::inInstanceAPI(context)) {
        Some(originalNode.clone())
    } else {
        None
    };
    redeclaredNode = NFInstNode::InstNode::replaceClass(new_cls, redeclareNode)?;
    node_ty = metamodelica::Ref::new(InstNodeType::REDECLARED_CLASS {
        parent: NFInstNode::InstNode::scopeRef(NFInstNode::InstNode::parent(&originalNode)?),
        originalType: NFInstNode::InstNode::nodeType(&originalNode)?,
        originalNode: orig_opt,
        confidence: instLevel,
    });
    redeclaredNode = NFInstNode::InstNode::setNodeType(node_ty, redeclaredNode)?;
    Ok(redeclaredNode)
}

pub(crate) fn redeclareEnum(
    mut redeclareClass: metamodelica::Ref<Class::NFClass>,
    mut originalClass: &metamodelica::Ref<Class::NFClass>,
    mut prefixes: metamodelica::Ref<Class::Prefixes::Prefixes>,
    mut outerMod: metamodelica::Ref<Modifier::Modifier>,
    mut redeclareNode: metamodelica::Ref<InstNode::InstNode>,
    mut originalNode: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<Class::NFClass>> {
    let mut redeclaredClass: metamodelica::Ref<Class::NFClass> = redeclareClass;
    expand(redeclareNode.clone(), context)?;
    redeclaredClass = NFInstNode::InstNode::getClass(redeclareNode.clone())?;
    redeclaredClass = (::match_deref::match_deref! { match &((redeclaredClass.clone(), originalClass.clone())) {
        (_, Deref @ Class::PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { literals: Deref @ metamodelica::ListNode::Nil, .. }, .. }) if (NFInstNode::InstNode::isEnumerationType(redeclareNode.clone())?) => {
            redeclaredClass = Class::setPrefixes(prefixes, redeclaredClass)?;
            redeclaredClass = Class::mergeModifier(outerMod, redeclaredClass)?;
            redeclaredClass
        },
        (Deref @ Class::PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { literals: lits1, .. }, .. }, Deref @ Class::PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { literals: lits2, .. }, .. }) => {
            if !((lits2).is_empty() || List::isEqualOnTrue(lits1.clone(), lits2.clone(), &fnptr!(stringEq, ArcStr, ArcStr))?) {
                Error::addMultiSourceMessage(&(Error::REDECLARE_ENUM_NON_SUBTYPE.clone()), &(list![NFInstNode::InstNode::name(originalNode)?]), &(list![NFInstNode::InstNode::info(&redeclareNode), NFInstNode::InstNode::info(originalNode)]))?;
                return Err("fail");
            }
            assign_variant_field!(redeclaredClass => Class::NFClass::PARTIAL_BUILTIN;
                prefixes = prefixes,
                modifier = Modifier::merge(outerMod, var_field!((*redeclaredClass).modifier, Class::NFClass::PARTIAL_BUILTIN).clone(), &(literal!("")))?
            );
            redeclaredClass
        },
        _ => {
            Error::addMultiSourceMessage(&(Error::REDECLARE_CLASS_NON_SUBTYPE.clone()), &(list![Restriction::toString(&(Class::restriction(originalClass))), NFInstNode::InstNode::name(originalNode)?]), &(list![NFInstNode::InstNode::info(&redeclareNode), NFInstNode::InstNode::info(originalNode)]))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(redeclaredClass)
}

pub(crate) fn instComponent(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut attributes: &metamodelica::Ref<Attributes::NFAttributes>,
    mut innerMod: metamodelica::Ref<Modifier::Modifier>,
    mut useBinding: bool,
    mut instLevel: i32,
    mut context: i32,
    mut originalAttr: Option<metamodelica::Ref<Attributes::NFAttributes>>,
    mut propagatedSubs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut def: metamodelica::Ref<SCode::Element>;
    let mut comp_node: metamodelica::Ref<InstNode::InstNode>;
    let mut rdcl_node: metamodelica::Ref<InstNode::InstNode>;
    let mut outer_mod: metamodelica::Ref<Modifier::Modifier>;
    let mut inner_mod: metamodelica::Ref<Modifier::Modifier>;
    let mut cc_mod: metamodelica::Ref<Modifier::Modifier> = innerMod.clone();
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    let mut next_context: i32;
    let mut propagated_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    if NFInstNode::InstNode::isEmpty(&node) {
        return Ok(());
    }
    checkOuterComponentMod(node.clone(), context)?;
    comp_node = NFInstNode::InstNode::resolveInner(node.clone());
    comp = NFInstNode::InstNode::component(&comp_node)?;
    parent = NFInstNode::InstNode::parent(&comp_node)?;
    if !(Component::isDefinition(&comp)) {
        checkRecursiveDefinition(Component::classInstance(&comp)?, comp_node, false)?;
        return Ok(());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp) {
        Deref @ Component::COMPONENT_DEF { definition: __pa0, modifier: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    def = metamodelica::Own::own(__pa0);
    outer_mod = metamodelica::Own::own(__pa1);
    if Modifier::isRedeclare(&outer_mod) {
        let (__pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(outer_mod) {
            Deref @ Modifier::REDECLARE { element: __pa2, innerMod: __pa3, outerMod: __pa4, constrainingMod: __pa5, propagatedSubs: __pa6, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
            _ => return Err("pattern mismatch"),
        } };
        rdcl_node = metamodelica::Own::own(__pa2);
        inner_mod = metamodelica::Own::own(__pa3);
        outer_mod = metamodelica::Own::own(__pa4);
        cc_mod = metamodelica::Own::own(__pa5);
        propagated_subs = metamodelica::Own::own(__pa6);
        next_context = InstContext::set(context, InstContext::REDECLARED.clone());
        instComponentDef(
            &def,
            crate::NFModifier::Modifier::interned_NOMOD(),
            inner_mod,
            &(Attributes::DEFAULT_ATTR().clone()),
            useBinding,
            comp_node,
            parent.clone(),
            instLevel,
            originalAttr,
            &(metamodelica::nil()),
            next_context,
        )?;
        cc_mod = getConstrainingMod(&def, parent, cc_mod, instLevel)?;
        cc_mod = Modifier::merge(cc_mod, innerMod, &(literal!("")))?;
        outer_mod = Modifier::merge(
            NFInstNode::InstNode::getModifier(&rdcl_node),
            outer_mod,
            &(literal!("")),
        )?;
        NFInstNode::InstNode::setModifier(outer_mod, rdcl_node.clone())?;
        redeclareComponent(
            rdcl_node,
            node.clone(),
            &(crate::NFModifier::Modifier::interned_NOMOD()),
            cc_mod,
            &propagated_subs,
            attributes,
            node,
            instLevel,
            context,
        )?;
    } else {
        instComponentDef(
            &def,
            outer_mod,
            innerMod,
            attributes,
            useBinding,
            comp_node,
            parent,
            instLevel,
            originalAttr,
            propagatedSubs,
            context,
        )?;
    }
    Ok(())
}

pub(crate) fn instComponentDef(
    mut component: &metamodelica::Ref<SCode::Element>,
    mut outerMod: metamodelica::Ref<Modifier::Modifier>,
    mut innerMod: metamodelica::Ref<Modifier::Modifier>,
    mut attributes: &metamodelica::Ref<Attributes::NFAttributes>,
    mut useBinding: bool,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut instLevel: i32,
    mut originalAttr: Option<metamodelica::Ref<Attributes::NFAttributes>>,
    mut propagatedSubs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut context: i32,
) -> Result<()> {
    let () = (match &**component {
        SCode::Element::COMPONENT {
            info,
            attributes: __component_attributes,
            comment: __component_comment,
            condition: __component_condition,
            prefixes: __component_prefixes,
            typeSpec: __component_typeSpec,
            ..
        } => {
            let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
            let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
            let mut binding: metamodelica::Ref<Binding::NFBinding>;
            let mut condition: metamodelica::Ref<Binding::NFBinding>;
            let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
            let mut ty_attr: metamodelica::Ref<Attributes::NFAttributes>;
            let mut inst_comp: metamodelica::Ref<Component::NFComponent>;
            let mut ty_node: metamodelica::Ref<InstNode::InstNode>;
            let mut ty: metamodelica::Ref<Class::NFClass>;
            let mut elementDefinition: metamodelica::Ref<SCode::Element>;
            let mut parent_res: metamodelica::Ref<Restriction::NFRestriction>;
            let mut res: metamodelica::Ref<Restriction::NFRestriction>;
            let mut cmt: metamodelica::Ref<SCode::Comment>;
            r#mod = instElementModifier(component, &node, parent.clone(), instLevel)?;
            if !((propagatedSubs).is_empty()) {
                r#mod = Modifier::propagateSubs(r#mod, propagatedSubs)?;
            }
            r#mod = Modifier::merge(r#mod, innerMod, &(literal!("")))?;
            r#mod = Modifier::merge(outerMod, r#mod, &(literal!("")))?;
            dims = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
                for mut d in (__component_attributes.arrayDims.clone()).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Dimension::NFDimension::RAW_DIM {
                        dim: d.clone(),
                        scope: NFInstNode::InstNode::scopeRef(parent.clone()),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            binding = if (useBinding) {
                Modifier::binding(&r#mod)
            } else {
                Binding::EMPTY_BINDING().clone()
            };
            condition = Binding::fromAbsyn(
                __component_condition.clone(),
                false,
                parent.clone(),
                instLevel,
                info.clone(),
            );
            parent_res = Class::restriction(&(NFInstNode::InstNode::getClass(parent.clone())?));
            attr = Attributes::fromSCode(
                metamodelica::AsArg::as_arg(&__component_attributes),
                metamodelica::AsArg::as_arg(&__component_prefixes),
            );
            attr = Attributes::checkDeclaredComponentAttributes(attr, &parent_res, &node)?;
            attr = Attributes::mergeComponentAttributes(attributes, attr, &node, &parent_res)?;
            if (originalAttr).is_some() {
                attr = Attributes::mergeRedeclaredComponentAttributes(Util::getOption(originalAttr)?, attr, &node)?;
            }
            if !(attr.isFinal.clone()) && Modifier::isFinal(&r#mod) {
                assign_field!(attr.isFinal = true);
            }
            inst_comp = metamodelica::Ref::new(Component::NFComponent::COMPONENT {
                classInst: crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                ty: crate::NFType::interned_UNKNOWN(),
                binding: binding.clone(),
                condition: condition,
                attributes: attr.clone(),
                comment: __component_comment.clone(),
                state: ComponentState::PartiallyInstantiated.clone(),
                info: info.clone(),
            });
            NFInstNode::InstNode::updateComponent(inst_comp, node.clone())?;
            r#mod = Modifier::propagate(r#mod, node.clone(), node.clone())?;
            (ty_node, ty_attr) = instTypeSpec(
                metamodelica::AsArg::as_arg(&__component_typeSpec),
                r#mod,
                attr.clone(),
                useBinding && !(Binding::isBound(&binding)),
                parent,
                node.clone(),
                info.clone(),
                instLevel,
                context,
            )?;
            NFInstNode::InstNode::componentApply(
                node.clone(),
                &Component::setType,
                metamodelica::Ref::new(Type::NFType::UNTYPED {
                    typeNode: ty_node.clone(),
                    dimensions: metamodelica::arrayFromVec(dims.into_iter().cloned().collect()),
                }),
            )?;
            if !(NFInstNode::InstNode::isEmpty(&ty_node)) {
                ty = NFInstNode::InstNode::getClass(ty_node.clone())?;
                res = Class::restriction(&ty);
                elementDefinition = NFInstNode::InstNode::definition(ty_node.clone())?;
                if Restriction::isType(&res)
                    && SCodeUtil::optCommentHasBooleanNamedAnnotationFalse(
                        SCodeUtil::getElementComment(&elementDefinition),
                        &(literal!("absoluteValue")),
                    )
                {
                    cmt = Component::comment(&(NFInstNode::InstNode::component(&node)?))?;
                    cmt = SCodeUtil::setAnnotationInComment(
                        literal!("absoluteValue"),
                        metamodelica::Ref::new(Absyn::Exp::BOOL { value: false }),
                        cmt,
                        false,
                    )?;
                    NFInstNode::InstNode::componentApply(node.clone(), &Component::setComment, cmt)?;
                }
                if !(InstContext::inRedeclared(context)) {
                    checkPartialComponent(
                        node.clone(),
                        &attr,
                        ty_node.clone(),
                        Class::isPartial(ty.clone())?,
                        &res,
                        context,
                        info.clone(),
                    )?;
                }
                checkBindingRestriction(&res, &binding, &node, info)?;
                ty_attr = Attributes::updateVariability(ty_attr, &ty, ty_node, node.clone(), context)?;
                ty_attr = Attributes::updateComponentConnectorType(ty_attr, &res, context, node.clone())?;
                if !(referenceEq(&*(attr), &*(&*ty_attr))) {
                    NFInstNode::InstNode::componentApply(node.clone(), &Component::setAttributes, ty_attr.clone())?;
                }
                if useBinding
                    && Binding::isUnbound(&binding)
                    && !(InstContext::inFunction(context))
                    && ty_attr.variability.clone() <= Variability::PARAMETER.clone()
                    && Restriction::isType(&res)
                {
                    updateParameterBinding(node, context)?;
                }
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn instElementModifier(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut instLevel: i32,
) -> Result<metamodelica::Ref<Modifier::Modifier>> {
    let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
    let mut cc_mod: metamodelica::Ref<Modifier::Modifier>;
    r#mod = Modifier::fromElement(element, parent.clone(), instLevel)?;
    if NFInstNode::InstNode::isRedeclared(component)? {
        r#mod = propagateRedeclaredMod(&r#mod, component)?;
    } else {
        cc_mod = instConstrainingMod(element, parent, instLevel)?;
        r#mod = Modifier::merge(r#mod, cc_mod, &(literal!("")))?;
    }
    Ok(r#mod)
}

pub(crate) fn instConstrainingMod(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut instLevel: i32,
) -> Result<metamodelica::Ref<Modifier::Modifier>> {
    let mut ccMod: metamodelica::Ref<Modifier::Modifier>;
    ccMod = (::match_deref::match_deref! { match element {
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: smod, .. }) }, .. }, name: __element_name, .. } => {
            Modifier::create(metamodelica::AsArg::as_arg(&smod), __element_name.clone(), &(metamodelica::Ref::new(ModifierScope::ModifierScope::CLASS { name: __element_name.clone() })), parent, instLevel)?
        },
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: smod, .. }) }, .. }, name: __element_name, .. } => {
            Modifier::create(metamodelica::AsArg::as_arg(&smod), __element_name.clone(), &(metamodelica::Ref::new(ModifierScope::ModifierScope::COMPONENT { name: __element_name.clone() })), parent, instLevel)?
        },
        _ => {
            crate::NFModifier::Modifier::interned_NOMOD()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ccMod)
}

pub(crate) fn getConstrainingMod(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut outerMod: metamodelica::Ref<Modifier::Modifier>,
    mut instLevel: i32,
) -> Result<metamodelica::Ref<Modifier::Modifier>> {
    let mut ccMod: metamodelica::Ref<Modifier::Modifier>;
    let mut name: ArcStr;
    let mut cc_smod: metamodelica::Ref<SCode::Mod>;
    cc_smod = SCodeUtil::getConstrainingMod(element);
    if !(SCodeUtil::isEmptyMod(&cc_smod)) {
        name = SCodeUtil::elementName(element)?;
        ccMod = Modifier::create(
            &cc_smod,
            name,
            &(ModifierScope::fromElement(element)?),
            parent,
            instLevel,
        )?;
        ccMod = Modifier::merge(outerMod, ccMod, &(literal!("")))?;
    } else {
        ccMod = outerMod;
    }
    Ok(ccMod)
}

pub(crate) fn propagateRedeclaredMod(
    mut r#mod: &metamodelica::Ref<Modifier::Modifier>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Modifier::Modifier>> {
    let mut outMod: metamodelica::Ref<Modifier::Modifier>;
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    let mut rdcl_scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    outMod = (::match_deref::match_deref! { match component {
        Deref @ NFInstNode::InstNode::COMPONENT_NODE { nodeType: Deref @ NFInstNode::InstNodeType::REDECLARED_COMP { parent: __esc_rdcl_scope }, .. } => {
            rdcl_scope = (*__esc_rdcl_scope).clone();
            parent = NFInstNode::InstNode::getDerivedNode(NFInstNode::InstNode::fromCell(rdcl_scope.clone())?, true)?;
            outMod = propagateRedeclaredMod(r#mod, &parent)?;
            Modifier::propagateBinding(outMod, parent.clone(), parent)
        },
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

pub(crate) fn checkPartialComponent(
    mut compNode: metamodelica::Ref<InstNode::InstNode>,
    mut compAttr: &metamodelica::Ref<Attributes::NFAttributes>,
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut isPartial: bool,
    mut res: &metamodelica::Ref<Restriction::NFRestriction>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<()> {
    if Restriction::isFunction(res) {
        if !(isPartial) && !(InstContext::inRelaxed(context)) {
            Error::addSourceMessage(
                &(Error::META_FUNCTION_TYPE_NO_PARTIAL_PREFIX.clone()),
                list![AbsynUtil::pathString(
                    NFInstNode::InstNode::scopePath(clsNode, NFInstNode::InstNode::ScopeType::RELATIVE.clone(), false)?,
                    literal!("."),
                    true,
                    false
                )?],
                &info,
            )?;
            return Err("fail");
        }
    } else if isPartial
        && compAttr.innerOuter.clone() != InnerOuter::OUTER.clone()
        && !(InstContext::inRelaxed(context))
    {
        Error::addMultiSourceMessage(
            &(Error::PARTIAL_COMPONENT_TYPE.clone()),
            &(list![
                AbsynUtil::pathString(
                    NFInstNode::InstNode::scopePath(
                        compNode,
                        NFInstNode::InstNode::ScopeType::RELATIVE.clone(),
                        false
                    )?,
                    literal!("."),
                    true,
                    false
                )?,
                NFInstNode::InstNode::name(&clsNode)?
            ]),
            &(list![NFInstNode::InstNode::info(&clsNode), info]),
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn checkBindingRestriction(
    mut restriction: &metamodelica::Ref<Restriction::NFRestriction>,
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<()> {
    if Binding::isBound(binding) {
        let () = (match &**restriction {
            Restriction::CLOCK => (),
            Restriction::CONNECTOR { .. } => (),
            Restriction::ENUMERATION => (),
            Restriction::EXTERNAL_OBJECT => (),
            Restriction::RECORD { .. } => (),
            Restriction::TYPE => (),
            _ => {
                Error::addSourceMessage(
                    &(Error::INVALID_SPECIALIZATION_FOR_BINDING_EQUATION.clone()),
                    list![
                        NFInstNode::InstNode::name(component)?,
                        Restriction::toString(restriction)
                    ],
                    info,
                )?;
                return Err("fail");
            }
        });
    }
    Ok(())
}

pub(crate) fn redeclareComponent(
    mut redeclareNode: metamodelica::Ref<InstNode::InstNode>,
    mut originalNode: metamodelica::Ref<InstNode::InstNode>,
    mut outerMod: &metamodelica::Ref<Modifier::Modifier>,
    mut constrainingMod: metamodelica::Ref<Modifier::Modifier>,
    mut propagatedSubs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut outerAttr: &metamodelica::Ref<Attributes::NFAttributes>,
    mut redeclaredNode: metamodelica::Ref<InstNode::InstNode>,
    mut instLevel: i32,
    mut context: i32,
) -> Result<()> {
    let mut orig_comp: metamodelica::Ref<Component::NFComponent>;
    let mut rdcl_comp: metamodelica::Ref<Component::NFComponent>;
    let mut new_comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut condition: metamodelica::Ref<Binding::NFBinding>;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    let mut orig_ty: metamodelica::Ref<Type::NFType>;
    let mut rdcl_ty: metamodelica::Ref<Type::NFType>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let mut orig_node: metamodelica::Ref<InstNode::InstNode>;
    let mut rdcl_node: metamodelica::Ref<InstNode::InstNode>;
    let mut rdcl_type: metamodelica::Ref<InstNodeType>;
    if !(NFInstNode::InstNode::isComponent(&redeclareNode)?) {
        Error::addMultiSourceMessage(
            &(Error::INVALID_REDECLARE_AS.clone()),
            &(list![
                NFInstNode::InstNode::typeName(&originalNode)?,
                NFInstNode::InstNode::name(&originalNode)?,
                NFInstNode::InstNode::typeName(&redeclareNode)?
            ]),
            &(list![
                NFInstNode::InstNode::info(&redeclareNode),
                NFInstNode::InstNode::info(&originalNode)
            ]),
        )?;
        return Err("fail");
    }
    orig_node = NFInstNode::InstNode::resolveInner(originalNode);
    orig_comp = NFInstNode::InstNode::component(&orig_node)?;
    rdcl_type = metamodelica::Ref::new(InstNodeType::REDECLARED_COMP {
        parent: NFInstNode::InstNode::scopeRef(NFInstNode::InstNode::parent(&orig_node)?),
    });
    rdcl_node = NFInstNode::InstNode::setNodeType(rdcl_type, redeclareNode.clone())?;
    rdcl_node = NFInstNode::InstNode::copyInstancePtr(&orig_node, rdcl_node)?;
    rdcl_node = NFInstNode::InstNode::updateComponent(NFInstNode::InstNode::component(&redeclareNode)?, rdcl_node)?;
    instComponent(
        rdcl_node.clone(),
        outerAttr,
        constrainingMod,
        true,
        instLevel,
        context,
        Some(Component::getAttributes(&orig_comp)),
        propagatedSubs,
    )?;
    rdcl_comp = NFInstNode::InstNode::component(&rdcl_node)?;
    new_comp = (::match_deref::match_deref! { match &((&*orig_comp, &*rdcl_comp)) {
        (Deref @ Component::COMPONENT { ty: __esc_orig_ty @ Deref @ Type::UNTYPED { .. }, .. }, Deref @ Component::COMPONENT { ty: __esc_rdcl_ty @ Deref @ Type::UNTYPED { .. }, .. }) => {
            orig_ty = (*__esc_orig_ty).clone();
            rdcl_ty = (*__esc_rdcl_ty).clone();
            if !(NFInstNode::InstNode::isReplaceable(&orig_node)?) && !(InstContext::inInstanceAPI(context)) && !(Type::isEqual(&(Type::arrayElementType(metamodelica::AsArg::as_arg(&orig_ty))), &(Type::arrayElementType(metamodelica::AsArg::as_arg(&rdcl_ty))))?) {
                Error::addMultiSourceMessage(&(Error::REDECLARE_NON_REPLACEABLE.clone()), &(list![NFInstNode::InstNode::name(&orig_node)?]), &(list![NFInstNode::InstNode::info(&orig_node), NFInstNode::InstNode::info(&rdcl_node)]))?;
                return Err("fail");
            }
            binding = Modifier::binding(outerMod);
            if Binding::isUnbound(&binding) {
                binding = if (Binding::isBound(var_field!((*rdcl_comp).binding, Component::NFComponent::COMPONENT))) {var_field!((*rdcl_comp).binding, Component::NFComponent::COMPONENT).clone()} else {var_field!((*orig_comp).binding, Component::NFComponent::COMPONENT).clone()};
            }
            if Binding::isBound(var_field!((*rdcl_comp).condition, Component::NFComponent::COMPONENT)) {
                Error::addSourceMessage(&(Error::REDECLARE_CONDITION.clone()), list![NFInstNode::InstNode::name(&redeclareNode)?], &(NFInstNode::InstNode::info(&redeclareNode)))?;
                return Err("fail");
            }
            condition = var_field!((*orig_comp).condition, Component::NFComponent::COMPONENT).clone();
            attr = var_field!((*rdcl_comp).attributes, Component::NFComponent::COMPONENT).clone();
            if Type::dimensionCount(rdcl_ty.clone()) == 0 {
                rdcl_ty = metamodelica::Ref::new(Type::NFType::UNTYPED { typeNode: var_field!((*rdcl_ty).typeNode, Type::NFType::UNTYPED).clone(), dimensions: var_field!((*orig_ty).dimensions, Type::NFType::UNTYPED).clone() });
            }
            cmt = var_field!((*orig_comp).comment, Component::NFComponent::COMPONENT).clone();
            metamodelica::Ref::new(Component::NFComponent::COMPONENT { classInst: var_field!((*rdcl_comp).classInst, Component::NFComponent::COMPONENT).clone(), ty: rdcl_ty.clone(), binding: binding, condition: condition, attributes: attr, comment: cmt, state: ComponentState::PartiallyInstantiated.clone(), info: var_field!((*rdcl_comp).info, Component::NFComponent::COMPONENT).clone() })
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.redeclareComponent")); __mm_s.push_str(&*literal!(" got unknown components")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    NFInstNode::InstNode::updateComponent(new_comp, NFInstNode::InstNode::resolveInner(redeclaredNode))?;
    Ok(())
}

pub(crate) fn checkOuterComponentMod(mut node: metamodelica::Ref<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut outer_node: metamodelica::Ref<InstNode::InstNode>;
    let mut elem: metamodelica::Ref<SCode::Element>;
    let mut smod: metamodelica::Ref<SCode::Mod>;
    outer_node = NFInstNode::InstNode::resolveOuter(node);
    elem = NFInstNode::InstNode::definition(outer_node.clone())?;
    if AbsynUtil::isOnlyOuter(SCodeUtil::prefixesInnerOuter(&(SCodeUtil::elementPrefixes(&elem)?))) {
        smod = SCodeUtil::componentMod(&elem);
        if !(SCodeUtil::isEmptyMod(&smod)) {
            Error::addSourceMessage(
                &(Error::OUTER_ELEMENT_MOD.clone()),
                list![
                    SCodeDump::printModStr(smod, SCodeDump::defaultOptions.clone())?,
                    NFInstNode::InstNode::name(&outer_node)?
                ],
                &(NFInstNode::InstNode::info(&outer_node)),
            )?;
            if !(InstContext::inInstanceAPI(context)) {
                return Err("fail");
            }
        }
    }
    Ok(())
}

pub(crate) fn classConfidence(
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut instLevel: i32,
    mut prefixes: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
) -> Result<i32> {
    let mut confidence: i32 = instLevel;
    let mut node: metamodelica::Ref<InstNode::InstNode> = scope;
    let mut enclosing: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut ty: metamodelica::Ref<InstNodeType>;
    if !(hasRedeclaredScope(clsNode.clone())?
        || List::any(
            prefixes,
            &move |__a0: metamodelica::Ref<InstNode::InstNode>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isRedeclaredClass(&__a0))
            },
        )?)
    {
        return Ok(confidence);
    }
    while !(NFInstNode::InstNode::isEmpty(&node) || NFInstNode::InstNode::isTopScope(&node)) {
        enclosing = metamodelica::cons(node.clone(), enclosing);
        node = instanceScope(&node)?;
    }
    node = clsNode;
    while !(NFInstNode::InstNode::isEmpty(&node)
        || NFInstNode::InstNode::isTopScope(&node)
        || List::exist1(
            &enclosing,
            &move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                NFInstNode::InstNode::refEqual(&__a0, &__a1)
            },
            node.clone(),
        )?)
    {
        let () = (::match_deref::match_deref! { match &(&*node) {
            Deref @ NFInstNode::InstNode::CLASS_NODE { nodeType: __esc_ty @ Deref @ NFInstNode::InstNodeType::REDECLARED_CLASS { .. }, .. } => {
                ty = (*__esc_ty).clone();
                confidence = std::cmp::min(confidence, var_field!((*ty).confidence, InstNodeType::REDECLARED_CLASS).clone());
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        node = instanceScope(&node)?;
    }
    for mut p in &**prefixes {
        if !(List::exist1(
            &enclosing,
            &move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                NFInstNode::InstNode::refEqual(&__a0, &__a1)
            },
            p.clone(),
        )?) {
            let () = (::match_deref::match_deref! { match &(p.clone()) {
                Deref @ NFInstNode::InstNode::CLASS_NODE { nodeType: __esc_ty @ Deref @ NFInstNode::InstNodeType::REDECLARED_CLASS { .. }, .. } => {
                    ty = (*__esc_ty).clone();
                    confidence = std::cmp::min(confidence, var_field!((*ty).confidence, InstNodeType::REDECLARED_CLASS).clone());
                    ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
    }
    Ok(confidence)
}

pub(crate) fn isRedeclaredClass(mut node: &metamodelica::Ref<InstNode::InstNode>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match node {
        Deref @ NFInstNode::InstNode::CLASS_NODE { nodeType: Deref @ NFInstNode::InstNodeType::REDECLARED_CLASS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn hasRedeclaredScope(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<bool> {
    let mut res: bool = false;
    let mut scope: metamodelica::Ref<InstNode::InstNode> = node;
    while !(NFInstNode::InstNode::isEmpty(&scope) || NFInstNode::InstNode::isTopScope(&scope)) {
        if isRedeclaredClass(&scope) {
            res = true;
            return Ok(res);
        }
        scope = instanceScope(&scope)?;
    }
    Ok(res)
}

pub(crate) fn instanceScope(
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    scope = (::match_deref::match_deref! { match node {
        Deref @ NFInstNode::InstNode::CLASS_NODE { nodeType: Deref @ NFInstNode::InstNodeType::BASE_CLASS { parent: ext_scope, .. }, .. } => {
            NFInstNode::InstNode::borrow(ext_scope.clone())?
        },
        Deref @ NFInstNode::InstNode::CLASS_NODE { nodeType: Deref @ NFInstNode::InstNodeType::REDECLARED_CLASS { parent: ext_scope, .. }, .. } => {
            NFInstNode::InstNode::borrow(ext_scope.clone())?
        },
        _ => {
            NFInstNode::InstNode::borrowParent(node)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(scope)
}

pub(crate) fn markTypeModifier(
    mut r#mod: metamodelica::Ref<Modifier::Modifier>,
    mut res: &metamodelica::Ref<Restriction::NFRestriction>,
    mut confidence: i32,
) -> Result<metamodelica::Ref<Modifier::Modifier>> {
    let mut r#mod: metamodelica::Ref<Modifier::Modifier> = r#mod;
    r#mod = (match &**res {
        Restriction::TYPE => Modifier::setSource(r#mod, Binding::Source::TYPE.clone(), confidence)?,
        Restriction::ENUMERATION => Modifier::setSource(r#mod, Binding::Source::TYPE.clone(), confidence)?,
        _ => r#mod,
    });
    Ok(r#mod)
}

pub(crate) fn instTypeSpec(
    mut typeSpec: &metamodelica::Ref<Absyn::TypeSpec>,
    mut modifier: metamodelica::Ref<Modifier::Modifier>,
    mut attributes: metamodelica::Ref<Attributes::NFAttributes>,
    mut useBinding: bool,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut parent: metamodelica::Ref<InstNode::InstNode>,
    mut info: SourceInfo,
    mut instLevel: i32,
    mut context: i32,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<Attributes::NFAttributes>,
)> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut outAttributes: metamodelica::Ref<Attributes::NFAttributes> =
        <metamodelica::Ref<Attributes::NFAttributes> as ::std::default::Default>::default();
    let mut prefixes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    node = 'mc: {
        let __mc_input = &**typeSpec;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TPATH { .. } => {
                    let mut node: metamodelica::Ref<InstNode::InstNode> = node.clone();
                    let mut outAttributes: metamodelica::Ref<Attributes::NFAttributes> = outAttributes.clone();
                    let mut prefixes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = prefixes.clone();
                    (node, prefixes) = Lookup::lookupClassName(var_field!((**typeSpec).path, Absyn::TypeSpec::TPATH).clone(), scope.clone(), context, info.clone(), true)?;
                    if instLevel >= 100 {
                        checkRecursiveDefinition(node.clone(), parent.clone(), true)?;
                    }
                    node = expand(node.clone(), context)?;
                    (node, outAttributes) = instClass(node.clone(), modifier.clone(), attributes.clone(), useBinding, instLevel, classConfidence(node.clone(), scope.clone(), instLevel, &prefixes)?, parent.clone(), context)?;
                    Ok((node.clone(), node.clone(), outAttributes.clone(), prefixes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            node = __wb0;
            outAttributes = __wb1;
            prefixes = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TPATH { .. } => {
                    if !((InstContext::inInstanceAPI(context))) { return Err("guard") }
                    let mut outAttributes: metamodelica::Ref<Attributes::NFAttributes> = outAttributes.clone();
                    outAttributes = attributes.clone();
                    Ok((crate::NFInstNode::InstNode::interned_EMPTY_NODE(), outAttributes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outAttributes = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TCOMPLEX { .. } => {
                    metamodelica::print(literal!("NFInst.instTypeSpec: TCOMPLEX not implemented.\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((node, outAttributes))
}

pub(crate) fn checkRecursiveDefinition(
    mut componentType: metamodelica::Ref<InstNode::InstNode>,
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut limitReached: bool,
) -> Result<()> {
    let mut parent: metamodelica::Ref<InstNode::InstNode> = NFInstNode::InstNode::parent(&component)?;
    let mut parent_type: metamodelica::Ref<InstNode::InstNode>;
    if !(Class::isFunction(&(NFInstNode::InstNode::getClass(parent.clone())?))) {
        while !(NFInstNode::InstNode::isEmpty(&parent)) {
            parent_type = NFInstNode::InstNode::classScope(parent.clone())?;
            if referenceEq(
                &*(NFInstNode::InstNode::definition(componentType.clone())?),
                &*(NFInstNode::InstNode::definition(parent_type)?),
            ) {
                Error::addSourceMessage(
                    &(Error::RECURSIVE_DEFINITION.clone()),
                    list![
                        NFInstNode::InstNode::name(&component)?,
                        NFInstNode::InstNode::name(
                            &(NFInstNode::InstNode::classScope(NFInstNode::InstNode::parent(&component)?)?)
                        )?
                    ],
                    &(NFInstNode::InstNode::info(&component)),
                )?;
                NFInstNode::InstNode::componentApply(
                    component.clone(),
                    &Component::setClassInstance,
                    crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                )?;
                return Err("fail");
            }
            parent = NFInstNode::InstNode::parent(&parent)?;
        }
    }
    if limitReached {
        Error::addSourceMessage(
            &(Error::INST_RECURSION_LIMIT_REACHED.clone()),
            list![AbsynUtil::pathString(
                NFInstNode::InstNode::scopePath(
                    component.clone(),
                    NFInstNode::InstNode::ScopeType::RELATIVE.clone(),
                    false
                )?,
                literal!("."),
                true,
                false
            )?],
            &(NFInstNode::InstNode::info(&component)),
        )?;
        NFInstNode::InstNode::componentApply(
            component,
            &Component::setClassInstance,
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn updateParameterBinding(mut node: metamodelica::Ref<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    if InstContext::inRedeclared(context) {
        return Ok(());
    }
    comp = NFInstNode::InstNode::component(&node)?;
    if !(Component::isFixed(&comp)?) || NFInstNode::InstNode::hasBinding(node.clone())? {
        return Ok(());
    }
    binding = Component::getTypeAttributeBinding(&comp, literal!("start"));
    if Binding::isBound(&binding) && !(Binding::hasTypeOrigin(&binding)?) {
        if !(InstContext::inRelaxed(context)) {
            Error::addSourceMessage(
                &(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone()),
                list![
                    AbsynUtil::pathString(
                        NFInstNode::InstNode::scopePath(
                            node.clone(),
                            NFInstNode::InstNode::ScopeType::RELATIVE.clone(),
                            false
                        )?,
                        literal!("."),
                        true,
                        false
                    )?,
                    Binding::toString(&binding, &(literal!("")))?
                ],
                &(NFInstNode::InstNode::info(&node)),
            )?;
        }
        binding = Binding::unpropagate(binding, &node)?;
        if Binding::isEach(&binding) {
            binding = Binding::expandEach(binding, &node)?;
        }
        comp = Component::setBinding(binding, comp)?;
        NFInstNode::InstNode::updateComponent(comp, node)?;
    }
    Ok(())
}

pub(crate) fn instDimension(
    mut dimension: metamodelica::Ref<Dimension::NFDimension>,
    mut context: i32,
    mut settings: &metamodelica::Ref<InstSettings::InstSettings>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dimension: metamodelica::Ref<Dimension::NFDimension> = dimension;
    dimension = (match &*dimension {
        Dimension::RAW_DIM {
            dim,
            scope: __dimension_scope,
        } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
            'mc: {
                let __mc_input = dim.clone();
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ Absyn::Subscript::NOSUB { .. } => {
                            Ok(crate::NFDimension::interned_UNKNOWN())
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
                            let mut exp: metamodelica::Ref<Expression::NFExpression>;
                            exp = instExp(var_field!((**dim).subscript, Absyn::Subscript::SUBSCRIPT).clone(), &(NFInstNode::InstNode::fromCell(__dimension_scope.clone())?), context, info)?;
                            if settings.resizableArrays.clone() {
                                exp = Expression::map(exp.clone(), (std::sync::Arc::new(instResizable) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                            }
                            Ok(metamodelica::Ref::new(Dimension::NFDimension::UNTYPED { dimension: exp.clone(), isProcessing: false }))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            if !((InstContext::inRelaxed(context))) { return Err("guard") }
                            Ok(crate::NFDimension::interned_UNKNOWN())
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                return Err("matchcontinue: no arm matched");
            }
        }
        _ => dimension,
    });
    Ok(dimension)
}

pub(crate) fn instResizable(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } if (NFInstNode::InstNode::isComponent(&(ComponentRef::node(var_field!((*exp).cref, Expression::NFExpression::CREF))?))? && Component::variability(&(NFInstNode::InstNode::component(&(ComponentRef::node(var_field!((*exp).cref, Expression::NFExpression::CREF))?))?))? == Variability::PARAMETER.clone()) => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut comp: metamodelica::Ref<Component::NFComponent>;
            let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
            node = ComponentRef::node(var_field!((*exp).cref, Expression::NFExpression::CREF))?;
            comp = NFInstNode::InstNode::component(&node)?;
            let () = (match &*comp {
        Component::COMPONENT { attributes: __esc_attr, .. } => {
            attr = (*__esc_attr).clone();
            assign_field!(
                attr.variability = Variability::NON_STRUCTURAL_PARAMETER.clone(),
                attr.isResizable = true
            );
            assign_variant_field!(comp => Component::NFComponent::COMPONENT; attributes = attr.clone());
            NFInstNode::InstNode::updateComponent(comp, node)?;
            ()
        },
        _ => (),
    });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn instExpressions(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut sections: metamodelica::Ref<Sections::NFSections>,
    mut connectBreaks: &metamodelica::Ref<ConnectBreakTree::Tree>,
    mut context: i32,
    mut settings: &metamodelica::Ref<InstSettings::InstSettings>,
) -> Result<metamodelica::Ref<Sections::NFSections>> {
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    let mut cls: metamodelica::Ref<Class::NFClass> = NFInstNode::InstNode::getClass(node.clone())?;
    let mut inst_cls: metamodelica::Ref<Class::NFClass>;
    let mut local_comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut dims: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>;
    let mut info: SourceInfo;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut next_context: i32;
    let mut connect_breaks: metamodelica::Ref<ConnectBreakTree::Tree>;
    let mut local_connect_breaks: metamodelica::List<Mutable::Mutable<ConnectBreakTree::Entry>>;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { elements: __esc_cls_tree, restriction: Deref @ Restriction::TYPE, prefixes: __cls_prefixes, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            exts = ClassTree::getExtends(metamodelica::AsArg::as_arg(&cls_tree));
            let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range0 {
                instExpressions(ext.clone(), &(ext), sections.clone(), connectBreaks, context, settings)?;
            }
            if metamodelica::arrayLength(exts.clone()) == 1 {
                ty = metamodelica::Ref::new(Type::NFType::COMPLEX { cls: NFInstNode::InstNode::identityCell(node.clone()), complexTy: metamodelica::Ref::new(ComplexType::NFComplexType::EXTENDS_TYPE { baseClass: NFInstNode::InstNode::identityCell(({let __elt = (*metamodelica::index_checked(&exts.borrow(), 1)?).clone(); __elt})) }) });
            } else if SCodeUtil::hasBooleanNamedAnnotationInClass(&(NFInstNode::InstNode::definition(node.clone())?), &(literal!("__OpenModelica_builtinType"))) {
                ty = metamodelica::Ref::new(Type::NFType::COMPLEX { cls: NFInstNode::InstNode::identityCell(node.clone()), complexTy: crate::NFComplexType::interned_CLASS() });
            } else {
                Error::addSourceMessage(&(Error::MISSING_TYPE_BASETYPE.clone()), list![NFInstNode::InstNode::name(&node)?], &(NFInstNode::InstNode::info(&node)))?;
                return Err("fail");
            }
            cls_tree = ClassTree::flatten(cls_tree.clone())?;
            inst_cls = metamodelica::Ref::new(Class::NFClass::INSTANCED_CLASS { ty: ty, elements: cls_tree.clone(), sections: crate::NFSections::interned_EMPTY(), prefixes: __cls_prefixes.clone(), restriction: var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone() });
            NFInstNode::InstNode::updateClass(inst_cls, node)?;
            ()
        },
        Deref @ Class::EXPANDED_CLASS { elements: __esc_cls_tree, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            (connect_breaks, local_connect_breaks) = ConnectBreakTree::appendBreaksInNode(&node, connectBreaks.clone())?;
            if settings.mergeExtendsSections.clone() {
                let __range0 = ClassTree::getExtends(metamodelica::AsArg::as_arg(&cls_tree)).borrow().iter().cloned().collect::<Vec<_>>();
                for mut ext in __range0 {
                    sections = instExpressions(ext.clone(), &(ext), sections, &connect_breaks, context, settings)?;
                }
            } else {
                let __range1 = ClassTree::getExtends(metamodelica::AsArg::as_arg(&cls_tree)).borrow().iter().cloned().collect::<Vec<_>>();
                for mut ext in __range1 {
                    instExpressions(ext.clone(), &(ext), sections.clone(), &connect_breaks, context, settings)?;
                }
            }
            ClassTree::applyLocalComponents(metamodelica::AsArg::as_arg(&cls_tree), &({ let __pe_b1 = context; let __pe_b2 = settings.clone(); move |__pe_a0| instComponentExpressions(__pe_a0, __pe_b1.clone(), &__pe_b2) }))?;
            assign_variant_field!(cls => Class::NFClass::EXPANDED_CLASS; elements = ClassTree::flatten(cls_tree.clone())?);
            NFInstNode::InstNode::updateClass(cls.clone(), node.clone())?;
            next_context = if (Restriction::isFunction(var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS))) {InstContext::FUNCTION.clone()} else {InstContext::CLASS.clone()};
            next_context = InstContext::set(context, next_context);
            sections = instSections(node.clone(), scope.clone(), &connect_breaks, next_context, sections)?;
            ConnectBreakTree::checkUnmatchedBreaks(&local_connect_breaks)?;
            ty = makeComplexType(var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS), node.clone(), &cls)?;
            inst_cls = metamodelica::Ref::new(Class::NFClass::INSTANCED_CLASS { ty: ty.clone(), elements: var_field!((*cls).elements, Class::NFClass::EXPANDED_CLASS).clone(), sections: sections.clone(), prefixes: var_field!((*cls).prefixes, Class::NFClass::EXPANDED_CLASS).clone(), restriction: var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone() });
            NFInstNode::InstNode::updateClass(inst_cls, node)?;
            instComplexType(&ty, context)?;
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { dims: __esc_dims, baseClass: __cls_baseClass, restriction: __cls_restriction, .. } => {
            dims = (*__esc_dims).clone();
            sections = instExpressions(__cls_baseClass.clone(), scope, sections, connectBreaks, context, settings)?;
            info = NFInstNode::InstNode::info(&node);
            for mut i in 1..=metamodelica::arrayLength(dims.clone()) {
                {
                    let __cell0 = instDimension(({let __elt = (*metamodelica::index_checked(&dims.borrow(), i)?).clone(); __elt}), context, settings, &info)?;
                    let __idx0 = i;
                    *metamodelica::index_mut_checked(&mut dims.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
            if Restriction::isRecord(metamodelica::AsArg::as_arg(&__cls_restriction)) {
                instRecordConstructor(node, context)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { elements: Deref @ ClassTree::FLAT_TREE { components: __esc_local_comps, .. }, .. } => {
            local_comps = (*__esc_local_comps).clone();
            let __range0 = local_comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                instComponentExpressions(comp, context, settings)?;
            }
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        Deref @ Class::INSTANCED_CLASS { .. } => (),
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instExpressions")); __mm_s.push_str(&*literal!(" got invalid class")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sections)
}

pub(crate) fn makeComplexType(
    mut restriction: &metamodelica::Ref<Restriction::NFRestriction>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut cls: &metamodelica::Ref<Class::NFClass>,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cty: metamodelica::Ref<ComplexType::NFComplexType>;
    cty = (match &**restriction {
        Restriction::RECORD { .. } => makeRecordComplexType(
            NFInstNode::InstNode::classScope(NFInstNode::InstNode::getDerivedNode(node.clone(), true)?)?,
            cls,
        )?,
        _ => crate::NFComplexType::interned_CLASS(),
    });
    ty = metamodelica::Ref::new(Type::NFType::COMPLEX {
        cls: NFInstNode::InstNode::identityCell(node),
        complexTy: cty,
    });
    Ok(ty)
}

pub(crate) fn makeRecordComplexType(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut cls: &metamodelica::Ref<Class::NFClass>,
) -> Result<metamodelica::Ref<ComplexType::NFComplexType>> {
    let mut ty: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut indexMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>> = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    cls_node = if (SCodeUtil::isOperatorRecord(&(NFInstNode::InstNode::definition(node.clone())?))) {
        NFInstNode::InstNode::classScope(node)?
    } else {
        NFInstNode::InstNode::classScope(NFInstNode::InstNode::getDerivedNode(node, true)?)?
    };
    ty = metamodelica::Ref::new(ComplexType::NFComplexType::RECORD {
        constructor: NFInstNode::InstNode::identityCell(cls_node),
        fields: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        indexMap: indexMap,
    });
    Ok(ty)
}

pub(crate) fn instComplexType(mut ty: &metamodelica::Ref<Type::NFType>, mut context: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match ty {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: cell, .. }, .. } if (!(NFInstNode::InstNode::isModel(NFInstNode::InstNode::borrow(cell.clone())?)?)) => {
            instRecordConstructor(NFInstNode::InstNode::borrow(cell.clone())?, context)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn instRecordConstructor(mut node: metamodelica::Ref<InstNode::InstNode>, mut context: i32) -> Result<()> {
    let mut cache: metamodelica::Ref<CachedData::CachedData>;
    cache = NFInstNode::InstNode::getFuncCache(&node)?;
    let () = (match &*cache {
        NFInstNode::CachedData::FUNCTION { .. } => (),
        _ => {
            NFInstNode::InstNode::cacheInitFunc(node.clone())?;
            if SCodeUtil::isOperatorRecord(&(NFInstNode::InstNode::definition(node.clone())?)) {
                OperatorOverloading::instConstructor(
                    NFInstNode::InstNode::fullPath(node.clone(), false)?,
                    node.clone(),
                    context,
                    NFInstNode::InstNode::info(&node),
                )?;
            } else {
                Record::instDefaultConstructor(
                    NFInstNode::InstNode::fullPath(node.clone(), false)?,
                    node.clone(),
                    context,
                    &(NFInstNode::InstNode::info(&node)),
                )?;
            }
            ()
        }
    });
    Ok(())
}

pub(crate) fn instBuiltinAttribute(
    mut attribute: metamodelica::Ref<Modifier::Modifier>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<Modifier::Modifier>> {
    let mut attribute: metamodelica::Ref<Modifier::Modifier> = attribute;
    let () = (match &*attribute {
        Modifier::MODIFIER { binding, .. } => {
            assign_variant_field!(attribute => Modifier::Modifier::MODIFIER; binding = instBinding(binding.clone(), context)?);
            ()
        }
        Modifier::REDECLARE { .. } => {
            Error::addSourceMessage(
                &(Error::INVALID_REDECLARE_IN_BASIC_TYPE.clone()),
                list![Modifier::name(&attribute)?],
                &(Modifier::info(&attribute)),
            )?;
            return Err("fail");
        }
        _ => (),
    });
    Ok(attribute)
}

pub(crate) fn instComponentExpressions(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut settings: &metamodelica::Ref<InstSettings::InstSettings>,
) -> Result<()> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut c: metamodelica::Ref<Component::NFComponent>;
    let mut dims: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>;
    if NFInstNode::InstNode::isEmpty(&component) {
        return Ok(());
    }
    node = NFInstNode::InstNode::resolveInner(component.clone());
    c = NFInstNode::InstNode::component(&node)?;
    let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Component::COMPONENT { ty: Deref @ Type::UNTYPED { dimensions: __esc_dims, .. }, state: __c_state, .. } if (__c_state.clone() == ComponentState::PartiallyInstantiated.clone()) => {
            dims = (*__esc_dims).clone();
            assign_variant_field!(c => Component::NFComponent::COMPONENT; state = ComponentState::FullyInstantiated.clone());
            NFInstNode::InstNode::updateComponent(c.clone(), node.clone())?;
            assign_variant_field!(c => Component::NFComponent::COMPONENT;
                binding = instBinding(var_field!((*c).binding, Component::NFComponent::COMPONENT).clone(), context)?,
                condition = instBinding(var_field!((*c).condition, Component::NFComponent::COMPONENT).clone(), context)?
            );
            if !(NFInstNode::InstNode::isEmpty(var_field!((*c).classInst, Component::NFComponent::COMPONENT))) {
                instExpressions(var_field!((*c).classInst, Component::NFComponent::COMPONENT).clone(), &node, crate::NFSections::interned_EMPTY(), &(ConnectBreakTree::new()), context, settings)?;
            }
            for mut i in 1..=metamodelica::arrayLength(dims.clone()) {
                {
                    let __cell0 = instDimension(({let __elt = (*metamodelica::index_checked(&dims.borrow(), i)?).clone(); __elt}), context, settings, var_field!((*c).info, Component::NFComponent::COMPONENT))?;
                    let __idx0 = i;
                    *metamodelica::index_mut_checked(&mut dims.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
            NFInstNode::InstNode::updateComponent(c, node)?;
            ()
        },
        Deref @ Component::COMPONENT { .. } => (),
        Deref @ Component::ENUM_LITERAL { .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { modifier: Deref @ Modifier::NOMOD, .. } => (),
        Deref @ Component::TYPE_ATTRIBUTE { modifier: __c_modifier, .. } => {
            assign_variant_field!(c => Component::NFComponent::TYPE_ATTRIBUTE; modifier = instBuiltinAttribute(__c_modifier.clone(), &component, context)?);
            NFInstNode::InstNode::updateComponent(c, node)?;
            ()
        },
        _ => {
            if !(InstContext::inRelaxed(context)) {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instComponentExpressions")); __mm_s.push_str(&*literal!(" got invalid component")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")))?;
                return Err("fail");
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn instBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut context: i32,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    if InstContext::inInstanceAPI(context) {
        ErrorExt::setCheckpoint(literal!("NFInst.instBinding"));
        match '__try0: {
            binding = unwrap_break_err!(instBinding(binding.clone(), InstContext::unset(context, InstContext::INSTANCE_API.clone())), '__try0);
            Ok::<_, &'static str>((binding.clone(),))
        } {
            Ok((__try0_o0,)) => {
                binding = __try0_o0;
            }
            Err(_) => {
                binding = metamodelica::Ref::new(Binding::NFBinding::INVALID_BINDING {
                    binding: binding.clone(),
                    errors: ErrorExt::getCheckpointMessages(),
                });
            }
        }
        ErrorExt::delCheckpoint(literal!("NFInst.instBinding"));
    } else {
        binding = (::match_deref::match_deref! { match &(binding.clone()) {
            Deref @ Binding::RAW_BINDING { bindingExp: Deref @ Absyn::Exp::BREAK { .. }, .. } => {
                crate::NFBinding::interned_UNBOUND()
            },
            Deref @ Binding::RAW_BINDING { bindingExp: __binding_bindingExp, confidence: __binding_confidence, eachType: __binding_eachType, info: __binding_info, scope: __binding_scope, source: __binding_source, subs: __binding_subs } => {
                let mut bind_exp: metamodelica::Ref<Expression::NFExpression>;
                bind_exp = instExp(__binding_bindingExp.clone(), &(NFInstNode::InstNode::fromCell(__binding_scope.clone())?), context, metamodelica::AsArg::as_arg(&__binding_info))?;
                if !((__binding_subs).is_empty()) {
                    bind_exp = metamodelica::Ref::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: bind_exp, subscripts: __binding_subs.clone(), ty: crate::NFType::interned_UNKNOWN(), split: true });
                }
                metamodelica::Ref::new(Binding::NFBinding::UNTYPED_BINDING { bindingExp: bind_exp, isProcessing: false, scope: __binding_scope.clone(), eachType: __binding_eachType.clone(), source: __binding_source.clone(), confidence: __binding_confidence.clone(), info: __binding_info.clone() })
            },
            _ => {
                binding
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(binding)
}

pub(crate) fn instExpOpt(
    mut absynExp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    exp = (::match_deref::match_deref! { match &(absynExp) {
        None => {
            None
        },
        Some(aexp) => {
            Some(instExp(aexp.clone(), scope, context, info)?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn instExp<'__b>(
    mut absynExp: metamodelica::Ref<Absyn::Exp>,
    mut scope: &'__b metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &'__b SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(absynExp.clone()) {
            Deref @ Absyn::Exp::INTEGER { value: __absynExp_value } => {
                return Ok(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: __absynExp_value.clone() }))
            },
            Deref @ Absyn::Exp::REAL { value: __absynExp_value } => {
                return Ok(metamodelica::Ref::new(Expression::NFExpression::REAL { value: stringReal(__absynExp_value.clone())? }))
            },
            Deref @ Absyn::Exp::STRING { value: __absynExp_value } => {
                return Ok(metamodelica::Ref::new(Expression::NFExpression::STRING { value: System::unescapedString(__absynExp_value.clone()) }))
            },
            Deref @ Absyn::Exp::BOOL { value: __absynExp_value } => {
                return Ok(metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: __absynExp_value.clone() }))
            },
            Deref @ Absyn::Exp::CREF { componentRef: __absynExp_componentRef } => {
                return Ok(instCref(__absynExp_componentRef.clone(), scope.clone(), context, info.clone())?)
            },
            Deref @ Absyn::Exp::ARRAY { arrayExp: __absynExp_arrayExp } => {
                let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
                arr = Array::mapList(metamodelica::AsArg::as_arg(&__absynExp_arrayExp), &({ let __pe_b1 = scope.clone(); let __pe_b2 = context; let __pe_b3 = info.clone(); move |__pe_a0| instExp(__pe_a0, &__pe_b1, __pe_b2.clone(), &__pe_b3) }))?;
                return Ok(Expression::makeArrayCheckLiteral(crate::NFType::interned_UNKNOWN(), arr.clone())?)
            },
            Deref @ Absyn::Exp::MATRIX { matrix: __absynExp_matrix } => {
                let mut expll: metamodelica::List<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>>;
                expll = ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> = metamodelica::nil();
            for mut el in (__absynExp_matrix.clone()).into_iter().cloned() {
                let __x = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut e in (el.clone()).into_iter().cloned() {
                let __x = instExp(e.clone(), scope, context, info)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                return Ok(metamodelica::Ref::new(Expression::NFExpression::MATRIX { elements: expll }))
            },
            Deref @ Absyn::Exp::RANGE { start: __absynExp_start, step: __absynExp_step, stop: __absynExp_stop } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut e3: metamodelica::Ref<Expression::NFExpression>;
                let mut oe: Option<metamodelica::Ref<Expression::NFExpression>>;
                e1 = instExp(__absynExp_start.clone(), scope, context, info)?;
                oe = instExpOpt(__absynExp_step.clone(), scope, context, info)?;
                e3 = instExp(__absynExp_stop.clone(), scope, context, info)?;
                return Ok(metamodelica::Ref::new(Expression::NFExpression::RANGE { ty: crate::NFType::interned_UNKNOWN(), start: e1, step: oe, stop: e3 }))
            },
            Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Cons { head: absynExp1, tail: Deref @ metamodelica::ListNode::Nil } } => {
                { (absynExp, scope, context, info) = (absynExp1.clone(), scope, context, info); continue '__tco; }
            },
            Deref @ Absyn::Exp::TUPLE { expressions: __absynExp_expressions } => {
                let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                expl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut e in (__absynExp_expressions.clone()).into_iter().cloned() {
                let __x = instExp(e.clone(), scope, context, info)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                return Ok(metamodelica::Ref::new(Expression::NFExpression::TUPLE { ty: crate::NFType::interned_UNKNOWN(), elements: expl }))
            },
            Deref @ Absyn::Exp::BINARY { exp1: __absynExp_exp1, exp2: __absynExp_exp2, op: __absynExp_op } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut e2: metamodelica::Ref<Expression::NFExpression>;
                let mut op: metamodelica::Ref<Operator::NFOperator>;
                e1 = instExp(__absynExp_exp1.clone(), scope, context, info)?;
                e2 = instExp(__absynExp_exp2.clone(), scope, context, info)?;
                op = Operator::fromAbsyn(__absynExp_op.clone());
                return Ok(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: e1, operator: op, exp2: e2 }))
            },
            Deref @ Absyn::Exp::UNARY { exp: __absynExp_exp, op: __absynExp_op } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut op: metamodelica::Ref<Operator::NFOperator>;
                e1 = instExp(__absynExp_exp.clone(), scope, context, info)?;
                op = Operator::fromAbsyn(__absynExp_op.clone());
                return Ok(Expression::makeUnary(op, e1))
            },
            Deref @ Absyn::Exp::LBINARY { exp1: __absynExp_exp1, exp2: __absynExp_exp2, op: __absynExp_op } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut e2: metamodelica::Ref<Expression::NFExpression>;
                let mut op: metamodelica::Ref<Operator::NFOperator>;
                e1 = instExp(__absynExp_exp1.clone(), scope, context, info)?;
                e2 = instExp(__absynExp_exp2.clone(), scope, context, info)?;
                op = Operator::fromAbsyn(__absynExp_op.clone());
                return Ok(metamodelica::Ref::new(Expression::NFExpression::LBINARY { exp1: e1, operator: op, exp2: e2 }))
            },
            Deref @ Absyn::Exp::LUNARY { exp: __absynExp_exp, op: __absynExp_op } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut op: metamodelica::Ref<Operator::NFOperator>;
                e1 = instExp(__absynExp_exp.clone(), scope, context, info)?;
                op = Operator::fromAbsyn(__absynExp_op.clone());
                return Ok(metamodelica::Ref::new(Expression::NFExpression::LUNARY { operator: op, exp: e1 }))
            },
            Deref @ Absyn::Exp::RELATION { exp1: __absynExp_exp1, exp2: __absynExp_exp2, op: __absynExp_op } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut e2: metamodelica::Ref<Expression::NFExpression>;
                let mut op: metamodelica::Ref<Operator::NFOperator>;
                e1 = instExp(__absynExp_exp1.clone(), scope, context, info)?;
                e2 = instExp(__absynExp_exp2.clone(), scope, context, info)?;
                op = Operator::fromAbsyn(__absynExp_op.clone());
                return Ok(metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: e1, operator: op, exp2: e2, index: -1 }))
            },
            Deref @ Absyn::Exp::IFEXP { elseBranch: __absynExp_elseBranch, elseIfBranch: __absynExp_elseIfBranch, ifExp: __absynExp_ifExp, trueBranch: __absynExp_trueBranch } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut e2: metamodelica::Ref<Expression::NFExpression>;
                let mut e3: metamodelica::Ref<Expression::NFExpression>;
                e3 = instExp(__absynExp_elseBranch.clone(), scope, context, info)?;
                for mut branch in &*__absynExp_elseIfBranch.clone().reverse() {
                    e1 = instExp(Util::tuple21(branch.clone()), scope, context, info)?;
                    e2 = instExp(Util::tuple22(branch.clone()), scope, context, info)?;
                    e3 = metamodelica::Ref::new(Expression::NFExpression::IF { ty: crate::NFType::interned_UNKNOWN(), condition: e1, trueBranch: e2, falseBranch: e3 });
                }
                e1 = instExp(__absynExp_ifExp.clone(), scope, context, info)?;
                e2 = instExp(__absynExp_trueBranch.clone(), scope, context, info)?;
                return Ok(metamodelica::Ref::new(Expression::NFExpression::IF { ty: crate::NFType::interned_UNKNOWN(), condition: e1, trueBranch: e2, falseBranch: e3 }))
            },
            Deref @ Absyn::Exp::CALL { functionArgs: __absynExp_functionArgs, function_: __absynExp_function_, .. } => {
                return Ok(Call::instantiate(__absynExp_function_.clone(), metamodelica::AsArg::as_arg(&__absynExp_functionArgs), scope.clone(), context, info.clone())?)
            },
            Deref @ Absyn::Exp::PARTEVALFUNCTION { functionArgs: __absynExp_functionArgs, function_: __absynExp_function_ } => {
                return Ok(instPartEvalFunction(__absynExp_function_.clone(), metamodelica::AsArg::as_arg(&__absynExp_functionArgs), scope.clone(), context, info.clone())?)
            },
            Deref @ Absyn::Exp::END { .. } => {
                return Ok(crate::NFExpression::interned_END())
            },
            Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: __absynExp_exp, .. } => {
                { (absynExp, scope, context, info) = (__absynExp_exp.clone(), scope, context, info); continue '__tco; }
            },
            Deref @ Absyn::Exp::SUBSCRIPTED_EXP { exp: __absynExp_exp, subscripts: __absynExp_subscripts } => {
                return Ok(metamodelica::Ref::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: instExp(__absynExp_exp.clone(), scope, context, info)?, subscripts: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut s in (__absynExp_subscripts.clone()).into_iter().cloned() {
                let __x = instSubscript(&(metamodelica::Ref::new(Subscript::NFSubscript::RAW_SUBSCRIPT { subscript: s.clone() })), scope, context, info)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), ty: crate::NFType::interned_UNKNOWN(), split: false }))
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInst.instExp")); __mm_s.push_str(&*literal!(" got unknown expression: ")); __mm_s.push_str(&*Dump::printExpStr(absynExp)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn instCref(
    mut absynCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut crefExp: metamodelica::Ref<Expression::NFExpression>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut found_scope: metamodelica::Ref<InstNode::InstNode>;
    (cref, found_scope) = (match &*absynCref {
        Absyn::ComponentRef::WILD { .. } => (crate::NFComponentRef::interned_WILD(), scope.clone()),
        Absyn::ComponentRef::ALLWILD { .. } => (crate::NFComponentRef::interned_WILD(), scope.clone()),
        _ => Lookup::lookupComponent(absynCref, scope.clone(), context, info.clone())?,
    });
    cref = instCrefSubscripts(cref, &scope, context, &info)?;
    crefExp = (match &*cref {
        ComponentRef::CREF { .. } => {
            (match &*(ComponentRef::node(&cref)?) {
                NFInstNode::InstNode::COMPONENT_NODE { .. } => {
                    instCrefComponent(cref.clone(), &(ComponentRef::node(&cref)?), found_scope, &info)?
                }
                NFInstNode::InstNode::CLASS_NODE { .. } => {
                    if (Class::isFunction(&(NFInstNode::InstNode::getClass(ComponentRef::node(&cref)?)?))) {
                        instCrefFunction(cref, found_scope, context, info)?
                    } else {
                        instCrefTypename(&(cref.clone()), ComponentRef::node(&cref)?, &info)?
                    }
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFInst.instCref"));
                            __mm_s.push_str(&*literal!(" got invalid instance node"));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")),
                    )?;
                    return Err("fail");
                }
            })
        }
        _ => metamodelica::Ref::new(Expression::NFExpression::CREF {
            ty: crate::NFType::interned_UNKNOWN(),
            cref: cref,
        }),
    });
    Ok(crefExp)
}

pub(crate) fn instCrefComponent(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut crefExp: metamodelica::Ref<Expression::NFExpression>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    comp = NFInstNode::InstNode::component(node)?;
    crefExp = (match &*comp {
        _ if (ComponentRef::isIterator(&cref)) => {
            checkUnsubscriptableCref(&cref, info)?;
            metamodelica::Ref::new(Expression::NFExpression::CREF {
                ty: crate::NFType::interned_UNKNOWN(),
                cref: cref.clone(),
            })
        }
        Component::ENUM_LITERAL {
            literal: __comp_literal,
            ..
        } => {
            checkUnsubscriptableCref(&cref, info)?;
            __comp_literal.clone()
        }
        Component::TYPE_ATTRIBUTE { .. } => {
            Error::addSourceMessage(
                &(Error::LOOKUP_VARIABLE_ERROR.clone()),
                list![
                    NFInstNode::InstNode::name(node)?,
                    NFInstNode::InstNode::name(&(NFInstNode::InstNode::parent(node)?))?
                ],
                info,
            )?;
            return Err("fail");
        }
        _ => metamodelica::Ref::new(Expression::NFExpression::CREF {
            ty: crate::NFType::interned_UNKNOWN(),
            cref: ComponentRef::appendScope(scope, cref.clone(), false)?,
        }),
    });
    Ok(crefExp)
}

pub(crate) fn instCrefFunction(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut crefExp: metamodelica::Ref<Expression::NFExpression>;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    fn_ref = ComponentRef::appendScope(scope, cref, true)?;
    (fn_ref, _, _) = Function::instFunctionRef(fn_ref, context, info)?;
    crefExp = metamodelica::Ref::new(Expression::NFExpression::CREF {
        ty: crate::NFType::interned_UNKNOWN(),
        cref: fn_ref,
    });
    Ok(crefExp)
}

pub(crate) fn instCrefTypename(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut crefExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    checkUnsubscriptableCref(cref, info)?;
    ty = NFInstNode::InstNode::getType(node.clone())?;
    ty = (match &*ty {
        Type::BOOLEAN => metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: ty,
            dimensions: list![crate::NFDimension::interned_BOOLEAN()],
        }),
        Type::ENUMERATION { .. } => metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: ty.clone(),
            dimensions: list![metamodelica::Ref::new(Dimension::NFDimension::ENUM { enumType: ty })],
        }),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFInst.instCrefTypename"));
                    __mm_s.push_str(&*literal!(" got unknown class node "));
                    __mm_s.push_str(&*NFInstNode::InstNode::name(&node)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")),
            )?;
            return Err("fail");
        }
    });
    crefExp = metamodelica::Ref::new(Expression::NFExpression::TYPENAME { ty: ty });
    Ok(crefExp)
}

pub(crate) fn checkUnsubscriptableCref(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut info: &SourceInfo,
) -> Result<()> {
    if ComponentRef::hasSubscripts(cref)? {
        Error::addSourceMessage(
            &(Error::WRONG_NUMBER_OF_SUBSCRIPTS.clone()),
            list![
                ComponentRef::toString(cref)?,
                ArcStr::from(::std::format!("{}", ((ComponentRef::getSubscripts(cref)).len() as i32))),
                literal!("0")
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn instCrefSubscripts(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let () = (match &*cref {
        ComponentRef::CREF { .. } => {
            let mut rest_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            if !((var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF)).is_empty()) {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; subscripts = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                    for mut s in (var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone()).into_iter().cloned() {
                        let __x = instSubscript(&(s.clone()), scope, context, info)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
            }
            rest_cr = instCrefSubscripts(
                var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(),
                scope,
                context,
                info,
            )?;
            if !(referenceEq(
                &*(&*rest_cr),
                &*(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone()),
            )) {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = rest_cr);
            }
            ()
        }
        _ => (),
    });
    Ok(cref)
}

pub(crate) fn instSubscript(
    mut subscript: &metamodelica::Ref<Subscript::NFSubscript>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Subscript::NFSubscript>> {
    let mut outSubscript: metamodelica::Ref<Subscript::NFSubscript>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut absynSub: metamodelica::Ref<Absyn::Subscript>;
    let __pa0 = ::match_deref::match_deref! { match &((*subscript)) {
        Deref @ Subscript::RAW_SUBSCRIPT { subscript: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    absynSub = metamodelica::Own::own(__pa0);
    outSubscript = (match &*absynSub {
        Absyn::Subscript::NOSUB { .. } => crate::NFSubscript::interned_WHOLE(),
        Absyn::Subscript::SUBSCRIPT {
            subscript: __absynSub_subscript,
        } => {
            exp = instExp(__absynSub_subscript.clone(), scope, context, info)?;
            Subscript::fromExp(exp)
        }
    });
    Ok(outSubscript)
}

pub(crate) fn instPartEvalFunction(
    mut func: metamodelica::Ref<Absyn::ComponentRef>,
    mut funcArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut arg_names: metamodelica::List<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &((*funcArgs)) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    nargs = metamodelica::Own::own(__pa0);
    outExp = instCref(func, scope.clone(), context, info.clone())?;
    if !((nargs).is_empty()) {
        fn_ref = Expression::toCref(&outExp)?;
        args = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut arg in (nargs.clone()).into_iter().cloned() {
                let __x = instExp(arg.argValue.clone(), &scope, context, &info)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        arg_names = ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut arg in (nargs).into_iter().cloned() {
                let __x = arg.argName.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        outExp = metamodelica::Ref::new(Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION {
            r#fn: fn_ref,
            args: args,
            argNames: arg_names,
            ty: crate::NFType::interned_UNKNOWN(),
        });
    }
    Ok(outExp)
}

pub(crate) fn instSections(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut connectBreaks: &metamodelica::Ref<ConnectBreakTree::Tree>,
    mut context: i32,
    mut sections: metamodelica::Ref<Sections::NFSections>,
) -> Result<metamodelica::Ref<Sections::NFSections>> {
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    let mut el: metamodelica::Ref<SCode::Element> = NFInstNode::InstNode::definition(node.clone())?;
    let mut def: metamodelica::Ref<SCode::ClassDef>;
    sections = (::match_deref::match_deref! { match &(&*el) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { .. }, .. } => instSections2(var_field!((*el).classDef, SCode::Element::CLASS), scope, connectBreaks, context, sections)?,
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: __esc_def @ Deref @ SCode::ClassDef::PARTS { .. }, .. }, .. } => {
            def = (*__esc_def).clone();
            instSections2(metamodelica::AsArg::as_arg(&def), scope, connectBreaks, context, sections)?
        },
        _ => sections,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sections)
}

pub(crate) fn instSections2(
    mut parts: &metamodelica::Ref<SCode::ClassDef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut connectBreaks: &metamodelica::Ref<ConnectBreakTree::Tree>,
    mut context: i32,
    mut sections: metamodelica::Ref<Sections::NFSections>,
) -> Result<metamodelica::Ref<Sections::NFSections>> {
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    sections = (::match_deref::match_deref! { match &((parts.clone(), sections.clone())) {
        (Deref @ SCode::ClassDef::PARTS { externalDecl: Some(ext_decl), .. }, Deref @ Sections::EXTERNAL { .. }) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), literal!("nonStdMultipleExternalDeclarations"))?) => {
            instExternalDecl(metamodelica::AsArg::as_arg(&ext_decl), scope, context)?
        },
        (_, Deref @ Sections::EXTERNAL { .. }) if (SCodeUtil::classDefHasSections(parts, true)) => {
            Error::addMultiSourceMessage(&(Error::MULTIPLE_SECTIONS_IN_FUNCTION.clone()), &(list![NFInstNode::InstNode::name(&scope)?]), &(list![var_field!((*sections).info, Sections::NFSections::EXTERNAL).clone(), NFInstNode::InstNode::info(&scope)]))?;
            return Err("fail")
        },
        (Deref @ SCode::ClassDef::PARTS { externalDecl: Some(ext_decl), .. }, _) => {
            if SCodeUtil::classDefHasSections(parts, false) {
                Error::addSourceMessage(&(Error::MULTIPLE_SECTIONS_IN_FUNCTION.clone()), list![NFInstNode::InstNode::name(&scope)?], &(NFInstNode::InstNode::info(&scope)))?;
                return Err("fail");
            }
            instExternalDecl(metamodelica::AsArg::as_arg(&ext_decl), scope, context)?
        },
        (Deref @ SCode::ClassDef::PARTS { .. }, _) => {
            let mut eq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            let mut ieq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            let mut alg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
            let mut ialg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
            let mut icontext: i32;
            icontext = InstContext::set(context, InstContext::INITIAL.clone());
            eq = instEquations(var_field!((**parts).normalEquationLst, SCode::ClassDef::PARTS).clone(), scope.clone(), connectBreaks, context)?;
            ieq = instEquations(var_field!((**parts).initialEquationLst, SCode::ClassDef::PARTS).clone(), scope.clone(), connectBreaks, icontext)?;
            alg = instAlgorithmSections(var_field!((**parts).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), scope.clone(), context)?;
            ialg = instAlgorithmSections(var_field!((**parts).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), scope, icontext)?;
            Sections::join(Sections::new(eq, ieq, alg, ialg), sections)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(sections)
}

pub(crate) fn instExternalDecl(
    mut extDecl: &metamodelica::Ref<SCode::ExternalDecl>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<Sections::NFSections>> {
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    sections = (match &**extDecl {
        SCode::ExternalDecl { .. } => {
            let mut name: ArcStr;
            let mut lang: ArcStr;
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut ret_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut info: SourceInfo;
            info = NFInstNode::InstNode::info(&scope);
            name = Util::getOptionOrDefault(extDecl.funcName.clone(), NFInstNode::InstNode::name(&scope)?);
            lang = Util::getOptionOrDefault(extDecl.lang.clone(), literal!("C"));
            checkExternalDeclLanguage(lang.clone(), &info)?;
            args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (extDecl.args.clone()).into_iter().cloned() {
                    let __x = instExp(arg.clone(), &scope, context, &info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if (extDecl.output_).is_some() {
                (ret_cref, _) = Lookup::lookupLocalComponent(
                    Util::getOption(extDecl.output_.clone())?,
                    scope,
                    context,
                    info.clone(),
                )?;
            } else {
                ret_cref = crate::NFComponentRef::interned_EMPTY();
            }
            metamodelica::Ref::new(Sections::NFSections::EXTERNAL {
                name: name,
                args: args,
                outputRef: ret_cref,
                language: lang,
                ann: extDecl.annotation_.clone(),
                explicit: (extDecl.funcName).is_some(),
                info: info,
            })
        }
    });
    Ok(sections)
}

pub(crate) fn checkExternalDeclLanguage(mut language: ArcStr, mut info: &SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(language.clone()) {
        Deref @ "C" => (),
        Deref @ "FORTRAN 77" => (),
        Deref @ "Fortran 77" => (),
        Deref @ "builtin" => (),
        _ => {
            Error::addSourceMessage(&(Error::INVALID_EXTERNAL_LANGUAGE.clone()), list![language], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn instEquations(
    mut scodeEql: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut connectBreaks: &metamodelica::Ref<ConnectBreakTree::Tree>,
    mut context: i32,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut instEql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    if InstContext::inInstanceAPI(context) {
        for mut eq in &*filterInstanceAPIEquations(scodeEql)? {
            if '__try0: {
                instEql = unwrap_break_err!(instEquation(metamodelica::AsArg::as_arg(&eq), scope.clone(), connectBreaks, context, instEql.clone()), '__try0);
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
    } else {
        for mut eq in &*scodeEql {
            instEql = instEquation(
                metamodelica::AsArg::as_arg(&eq),
                scope.clone(),
                connectBreaks,
                context,
                instEql,
            )?;
        }
    }
    instEql = metamodelica::Dangerous::listReverseInPlace(instEql);
    Ok(instEql)
}

pub(crate) fn filterInstanceAPIEquations(
    mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    let mut outEql: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
    let mut name: ArcStr;
    for mut eq in &*eql {
        let mut eq = eq.clone();
        outEql = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ SCode::Equation::EQ_CONNECT { .. } => metamodelica::cons(eq, outEql),
            Deref @ SCode::Equation::EQ_NORETCALL { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name, .. }, .. }, .. } if (metamodelica::stringEq(&name, &(literal!("transition"))) || metamodelica::stringEq(&name, &(literal!("initialState")))) => metamodelica::cons(eq, outEql),
            Deref @ SCode::Equation::EQ_FOR { eEquationLst: __eq_eEquationLst, .. } => {
                assign_variant_field!(eq => SCode::Equation::EQ_FOR; eEquationLst = filterInstanceAPIEquations(__eq_eEquationLst.clone())?);
                if ((var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR)).is_empty()) {outEql} else {metamodelica::cons(eq, outEql)}
            },
            Deref @ SCode::Equation::EQ_IF { thenBranch: __eq_thenBranch, .. } => {
                assign_variant_field!(eq => SCode::Equation::EQ_IF;
                    thenBranch = ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>> = metamodelica::nil();
            for mut eql in (__eq_thenBranch.clone()).into_iter().cloned() {
                let __x = filterInstanceAPIEquations(eql.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
                    elseBranch = filterInstanceAPIEquations(var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone())?
                );
                if (List::all(var_field!((*eq).thenBranch, SCode::Equation::EQ_IF), &fnptr!(listEmpty, _))? && (var_field!((*eq).elseBranch, SCode::Equation::EQ_IF)).is_empty()) {outEql} else {metamodelica::cons(eq, outEql)}
            },
            _ => outEql,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outEql = metamodelica::Dangerous::listReverseInPlace(outEql);
    Ok(outEql)
}

pub(crate) fn instEquation(
    mut scodeEq: &metamodelica::Ref<SCode::Equation>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut connectBreaks: &metamodelica::Ref<ConnectBreakTree::Tree>,
    mut context: i32,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    equations = (match &**scodeEq {
        SCode::Equation::EQ_EQUALS {
            info,
            comment: __scodeEq_comment,
            expLeft: __scodeEq_expLeft,
            expRight: __scodeEq_expRight,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            exp1 = instExp(__scodeEq_expLeft.clone(), &scope, context, info)?;
            exp2 = instExp(__scodeEq_expRight.clone(), &scope, context, info)?;
            metamodelica::cons(
                Equation::makeEquality(
                    exp1,
                    exp2,
                    crate::NFType::interned_UNKNOWN(),
                    makeSource(__scodeEq_comment.clone(), info.clone()),
                    scope,
                    Equation::ScalarizeMode::NO_PREFERENCE.clone(),
                ),
                equations,
            )
        }
        SCode::Equation::EQ_CONNECT {
            info,
            comment: __scodeEq_comment,
            crefLeft: __scodeEq_crefLeft,
            crefRight: __scodeEq_crefRight,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            let mut lhs_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut rhs_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut next_context: i32;
            if InstContext::inInitial(context) {
                Error::addSourceMessage(&(Error::CONNECT_IN_INITIAL_EQUATION.clone()), metamodelica::nil(), info)?;
                return Err("fail");
            } else if InstContext::inWhen(context) {
                Error::addSourceMessage(
                    &(Error::CONNECT_IN_WHEN.clone()),
                    list![
                        Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&__scodeEq_crefLeft))?,
                        Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&__scodeEq_crefRight))?
                    ],
                    info,
                )?;
                return Err("fail");
            }
            if !(ConnectBreakTree::isConnectBroken(
                __scodeEq_crefLeft.clone(),
                __scodeEq_crefRight.clone(),
                scope.clone(),
                connectBreaks,
            )?) {
                next_context = InstContext::set(context, InstContext::CONNECT.clone());
                lhs_cr = instConnectorCref(__scodeEq_crefLeft.clone(), scope.clone(), next_context, info.clone())?;
                rhs_cr = instConnectorCref(__scodeEq_crefRight.clone(), scope.clone(), next_context, info.clone())?;
                if !(NFInstNode::InstNode::isEmpty(&(ComponentRef::node(&lhs_cr)?))
                    || NFInstNode::InstNode::isEmpty(&(ComponentRef::node(&rhs_cr)?)))
                {
                    exp1 = metamodelica::Ref::new(Expression::NFExpression::CREF {
                        ty: crate::NFType::interned_UNKNOWN(),
                        cref: lhs_cr,
                    });
                    exp2 = metamodelica::Ref::new(Expression::NFExpression::CREF {
                        ty: crate::NFType::interned_UNKNOWN(),
                        cref: rhs_cr,
                    });
                    equations = metamodelica::cons(
                        metamodelica::Ref::new(Equation::NFEquation::CONNECT {
                            lhs: exp1,
                            rhs: exp2,
                            scope: NFInstNode::InstNode::identityCell(scope),
                            source: makeSource(__scodeEq_comment.clone(), info.clone()),
                        }),
                        equations,
                    );
                }
            }
            equations
        }
        SCode::Equation::EQ_FOR {
            info,
            comment: __scodeEq_comment,
            eEquationLst: __scodeEq_eEquationLst,
            index: __scodeEq_index,
            range: __scodeEq_range,
        } => {
            let mut oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
            let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            let mut for_scope: metamodelica::Ref<InstNode::InstNode>;
            let mut iter: metamodelica::Ref<InstNode::InstNode>;
            let mut next_context: i32;
            oexp = instExpOpt(__scodeEq_range.clone(), &scope, context, info)?;
            checkIteratorShadowing(
                __scodeEq_index.clone(),
                &scope,
                var_field!((**scodeEq).info, SCode::Equation::EQ_FOR).clone(),
            )?;
            (for_scope, iter) = addIteratorToScope(
                __scodeEq_index.clone(),
                scope.clone(),
                var_field!((**scodeEq).info, SCode::Equation::EQ_FOR).clone(),
                crate::NFType::interned_UNKNOWN(),
            )?;
            next_context = InstContext::set(context, InstContext::FOR.clone());
            eql = instEquations(__scodeEq_eEquationLst.clone(), for_scope, connectBreaks, next_context)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::FOR {
                    iterator: iter,
                    range: oexp,
                    body: eql,
                    scope: NFInstNode::InstNode::identityCell(scope),
                    source: makeSource(__scodeEq_comment.clone(), info.clone()),
                }),
                equations,
            )
        }
        SCode::Equation::EQ_IF {
            info,
            comment: __scodeEq_comment,
            condition: __scodeEq_condition,
            elseBranch: __scodeEq_elseBranch,
            thenBranch: __scodeEq_thenBranch,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            let mut branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
            let mut next_context: i32;
            expl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut c in (__scodeEq_condition.clone()).into_iter().cloned() {
                    let __x = instExp(c.clone(), &scope, context, info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            next_context = InstContext::set(context, InstContext::IF.clone());
            branches = metamodelica::nil();
            for mut branch in &*__scodeEq_thenBranch.clone() {
                eql = instEquations(branch.clone(), scope.clone(), connectBreaks, next_context)?;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expl) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                exp1 = metamodelica::Own::own(__pa0);
                expl = metamodelica::Own::own(__pa1);
                branches = metamodelica::cons(
                    Equation::makeBranch(exp1, eql, Prefixes::Variability::CONTINUOUS.clone()),
                    branches,
                );
            }
            if !((__scodeEq_elseBranch).is_empty()) {
                eql = instEquations(__scodeEq_elseBranch.clone(), scope.clone(), connectBreaks, next_context)?;
                branches = metamodelica::cons(
                    Equation::makeBranch(
                        metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
                        eql,
                        Prefixes::Variability::CONTINUOUS.clone(),
                    ),
                    branches,
                );
            }
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::IF {
                    branches: branches.reverse(),
                    scope: NFInstNode::InstNode::identityCell(scope),
                    source: makeSource(__scodeEq_comment.clone(), info.clone()),
                }),
                equations,
            )
        }
        SCode::Equation::EQ_WHEN {
            info,
            comment: __scodeEq_comment,
            condition: __scodeEq_condition,
            eEquationLst: __scodeEq_eEquationLst,
            elseBranches: __scodeEq_elseBranches,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            let mut branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
            let mut next_context: i32;
            if InstContext::inWhen(context) {
                Error::addSourceMessageAndFail(&(Error::NESTED_WHEN.clone()), metamodelica::nil(), info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            } else if InstContext::inInitial(context) {
                Error::addSourceMessageAndFail(&(Error::INITIAL_WHEN.clone()), metamodelica::nil(), info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            next_context = InstContext::set(context, InstContext::WHEN.clone());
            exp1 = instExp(__scodeEq_condition.clone(), &scope, context, info)?;
            eql = instEquations(
                __scodeEq_eEquationLst.clone(),
                scope.clone(),
                connectBreaks,
                next_context,
            )?;
            branches = list![Equation::makeBranch(
                exp1,
                eql,
                Prefixes::Variability::CONTINUOUS.clone()
            )];
            for mut branch in &*__scodeEq_elseBranches.clone() {
                exp1 = instExp(Util::tuple21(branch.clone()), &scope, context, info)?;
                eql = instEquations(
                    Util::tuple22(branch.clone()),
                    scope.clone(),
                    connectBreaks,
                    next_context,
                )?;
                branches = metamodelica::cons(
                    Equation::makeBranch(exp1, eql, Prefixes::Variability::CONTINUOUS.clone()),
                    branches,
                );
            }
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::WHEN {
                    branches: branches.reverse(),
                    scope: NFInstNode::InstNode::identityCell(scope),
                    source: makeSource(__scodeEq_comment.clone(), info.clone()),
                }),
                equations,
            )
        }
        SCode::Equation::EQ_ASSERT {
            info,
            comment: __scodeEq_comment,
            condition: __scodeEq_condition,
            level: __scodeEq_level,
            message: __scodeEq_message,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            let mut exp3: metamodelica::Ref<Expression::NFExpression>;
            exp1 = instExp(__scodeEq_condition.clone(), &scope, context, info)?;
            exp2 = instExp(__scodeEq_message.clone(), &scope, context, info)?;
            exp3 = instExp(__scodeEq_level.clone(), &scope, context, info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::ASSERT {
                    condition: exp1,
                    message: exp2,
                    level: exp3,
                    scope: NFInstNode::InstNode::identityCell(scope),
                    source: makeSource(__scodeEq_comment.clone(), info.clone()),
                }),
                equations,
            )
        }
        SCode::Equation::EQ_TERMINATE {
            info,
            comment: __scodeEq_comment,
            message: __scodeEq_message,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            exp1 = instExp(__scodeEq_message.clone(), &scope, context, info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::TERMINATE {
                    message: exp1,
                    scope: NFInstNode::InstNode::identityCell(scope),
                    source: makeSource(__scodeEq_comment.clone(), info.clone()),
                }),
                equations,
            )
        }
        SCode::Equation::EQ_REINIT {
            info,
            comment: __scodeEq_comment,
            cref: __scodeEq_cref,
            expReinit: __scodeEq_expReinit,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            if !(InstContext::inWhen(context)) {
                Error::addSourceMessage(&(Error::REINIT_NOT_IN_WHEN.clone()), metamodelica::nil(), info)?;
                return Err("fail");
            }
            exp1 = instExp(__scodeEq_cref.clone(), &scope, context, info)?;
            exp2 = instExp(__scodeEq_expReinit.clone(), &scope, context, info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::REINIT {
                    cref: exp1,
                    reinitExp: exp2,
                    scope: NFInstNode::InstNode::identityCell(scope),
                    source: makeSource(__scodeEq_comment.clone(), info.clone()),
                }),
                equations,
            )
        }
        SCode::Equation::EQ_NORETCALL {
            info,
            comment: __scodeEq_comment,
            exp: __scodeEq_exp,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            exp1 = instExp(__scodeEq_exp.clone(), &scope, context, info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::NORETCALL {
                    exp: exp1,
                    scope: NFInstNode::InstNode::identityCell(scope),
                    source: makeSource(__scodeEq_comment.clone(), info.clone()),
                }),
                equations,
            )
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFInst.instEquation"));
                    __mm_s.push_str(&*literal!(" got unknown equation"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(equations)
}

pub(crate) fn instConnectorCref(
    mut absynCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut found_scope: metamodelica::Ref<InstNode::InstNode>;
    (cref, found_scope) = Lookup::lookupConnector(absynCref, scope.clone(), context, info.clone())?;
    cref = instCrefSubscripts(cref, &scope, context, &info)?;
    cref = ComponentRef::appendScope(found_scope, cref, false)?;
    Ok(cref)
}

pub(crate) fn makeSource(
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut info: SourceInfo,
) -> metamodelica::Ref<DAE::ElementSource> {
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    source = metamodelica::Ref::new(DAE::ElementSource {
        info: info,
        partOfLst: metamodelica::nil(),
        instance: openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(),
        connectEquationOptLst: metamodelica::nil(),
        typeLst: metamodelica::nil(),
        operations: metamodelica::nil(),
        comment: list![comment],
    });
    source
}

pub(crate) fn instAlgorithmSections(
    mut algorithmSections: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>> {
    let mut algs: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    if InstContext::inInstanceAPI(context) {
        algs = metamodelica::nil();
    } else {
        algs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut alg in (algorithmSections).into_iter().cloned() {
                let __x = instAlgorithmSection(&(alg.clone()), scope.clone(), context)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    Ok(algs)
}

pub(crate) fn instAlgorithmSection(
    mut algorithmSection: &metamodelica::Ref<SCode::AlgorithmSection>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    statements = instStatements(algorithmSection.statements.clone(), scope.clone(), context)?;
    alg = metamodelica::Ref::new(Algorithm::NFAlgorithm {
        statements: statements,
        inputs: metamodelica::nil(),
        outputs: metamodelica::nil(),
        stmtDiffInfo: None,
        scope: NFInstNode::InstNode::identityCell(scope),
        source: DAE::emptyElementSource().clone(),
    });
    Ok(alg)
}

pub(crate) fn instStatements(
    mut scodeStmtl: metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    statements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        for mut stmt in (scodeStmtl).into_iter().cloned() {
            let __x = instStatement(&(stmt.clone()), scope.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(statements)
}

pub(crate) fn instStatement(
    mut scodeStmt: &metamodelica::Ref<SCode::Statement>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut statement: metamodelica::Ref<Statement::NFStatement>;
    statement = (match &**scodeStmt {
        SCode::Statement::ALG_ASSIGN {
            info,
            assignComponent: __scodeStmt_assignComponent,
            comment: __scodeStmt_comment,
            value: __scodeStmt_value,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            exp1 = instExp(__scodeStmt_assignComponent.clone(), &scope, context, info)?;
            checkAssignmentRestriction(&exp1, info)?;
            exp2 = instExp(__scodeStmt_value.clone(), &scope, context, info)?;
            metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                lhs: exp1,
                rhs: exp2,
                ty: crate::NFType::interned_UNKNOWN(),
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
            })
        }
        SCode::Statement::ALG_FOR {
            info,
            comment: __scodeStmt_comment,
            forBody: __scodeStmt_forBody,
            index: __scodeStmt_index,
            range: __scodeStmt_range,
        } => {
            let mut oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
            let mut stmtl: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut for_scope: metamodelica::Ref<InstNode::InstNode>;
            let mut iter: metamodelica::Ref<InstNode::InstNode>;
            let mut next_context: i32;
            oexp = instExpOpt(__scodeStmt_range.clone(), &scope, context, info)?;
            (for_scope, iter) = addIteratorToScope(
                __scodeStmt_index.clone(),
                scope,
                info.clone(),
                crate::NFType::interned_UNKNOWN(),
            )?;
            next_context = InstContext::set(context, InstContext::FOR.clone());
            stmtl = instStatements(__scodeStmt_forBody.clone(), for_scope, next_context)?;
            metamodelica::Ref::new(Statement::NFStatement::FOR {
                iterator: iter,
                range: oexp,
                body: stmtl,
                forType: crate::NFStatement::ForType::NORMAL,
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
                sub_iters: metamodelica::nil(),
            })
        }
        SCode::Statement::ALG_PARFOR {
            info,
            comment: __scodeStmt_comment,
            index: __scodeStmt_index,
            parforBody: __scodeStmt_parforBody,
            range: __scodeStmt_range,
        } => {
            let mut oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
            let mut stmtl: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut for_scope: metamodelica::Ref<InstNode::InstNode>;
            let mut iter: metamodelica::Ref<InstNode::InstNode>;
            let mut next_context: i32;
            oexp = instExpOpt(__scodeStmt_range.clone(), &scope, context, info)?;
            (for_scope, iter) = addIteratorToScope(
                __scodeStmt_index.clone(),
                scope,
                info.clone(),
                crate::NFType::interned_UNKNOWN(),
            )?;
            next_context = InstContext::set(context, InstContext::FOR.clone());
            stmtl = instStatements(__scodeStmt_parforBody.clone(), for_scope, next_context)?;
            metamodelica::Ref::new(Statement::NFStatement::FOR {
                iterator: iter,
                range: oexp,
                body: stmtl,
                forType: Statement::ForType::PARALLEL {
                    vars: metamodelica::nil(),
                },
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
                sub_iters: metamodelica::nil(),
            })
        }
        SCode::Statement::ALG_IF {
            info,
            boolExpr: __scodeStmt_boolExpr,
            comment: __scodeStmt_comment,
            elseBranch: __scodeStmt_elseBranch,
            elseIfBranch: __scodeStmt_elseIfBranch,
            trueBranch: __scodeStmt_trueBranch,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut stmtl: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut branches: metamodelica::List<(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
            )>;
            let mut next_context: i32;
            branches = metamodelica::nil();
            next_context = InstContext::set(context, InstContext::FOR.clone());
            for mut branch in &*metamodelica::cons(
                (__scodeStmt_boolExpr.clone(), __scodeStmt_trueBranch.clone()),
                __scodeStmt_elseIfBranch.clone(),
            ) {
                exp1 = instExp(Util::tuple21(branch.clone()), &scope, context, info)?;
                stmtl = instStatements(Util::tuple22(branch.clone()), scope.clone(), next_context)?;
                branches = metamodelica::cons((exp1, stmtl), branches);
            }
            if !((__scodeStmt_elseBranch).is_empty()) {
                stmtl = instStatements(__scodeStmt_elseBranch.clone(), scope, next_context)?;
                branches = metamodelica::cons(
                    (
                        metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
                        stmtl,
                    ),
                    branches,
                );
            }
            metamodelica::Ref::new(Statement::NFStatement::IF {
                branches: branches.reverse(),
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
            })
        }
        SCode::Statement::ALG_WHEN_A {
            info,
            branches: __scodeStmt_branches,
            comment: __scodeStmt_comment,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut stmtl: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut branches: metamodelica::List<(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
            )>;
            let mut next_context: i32;
            if !(InstContext::inValidWhenScope(context)) {
                if InstContext::inWhen(context) {
                    Error::addSourceMessageAndFail(&(Error::NESTED_WHEN.clone()), metamodelica::nil(), info)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                } else if InstContext::inInitial(context) {
                    Error::addSourceMessageAndFail(&(Error::INITIAL_WHEN.clone()), metamodelica::nil(), info)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                } else {
                    Error::addSourceMessageAndFail(
                        &(Error::INVALID_WHEN_STATEMENT_CONTEXT.clone()),
                        metamodelica::nil(),
                        info,
                    )?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
            }
            branches = metamodelica::nil();
            for mut branch in &*__scodeStmt_branches.clone() {
                exp1 = instExp(Util::tuple21(branch.clone()), &scope, context, info)?;
                next_context = InstContext::set(context, InstContext::WHEN.clone());
                stmtl = instStatements(Util::tuple22(branch.clone()), scope.clone(), next_context)?;
                branches = metamodelica::cons((exp1, stmtl), branches);
            }
            metamodelica::Ref::new(Statement::NFStatement::WHEN {
                branches: branches.reverse(),
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
            })
        }
        SCode::Statement::ALG_ASSERT {
            info,
            comment: __scodeStmt_comment,
            condition: __scodeStmt_condition,
            level: __scodeStmt_level,
            message: __scodeStmt_message,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            let mut exp3: metamodelica::Ref<Expression::NFExpression>;
            exp1 = instExp(__scodeStmt_condition.clone(), &scope, context, info)?;
            exp2 = instExp(__scodeStmt_message.clone(), &scope, context, info)?;
            exp3 = instExp(__scodeStmt_level.clone(), &scope, context, info)?;
            metamodelica::Ref::new(Statement::NFStatement::ASSERT {
                condition: exp1,
                message: exp2,
                level: exp3,
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
            })
        }
        SCode::Statement::ALG_TERMINATE {
            info,
            comment: __scodeStmt_comment,
            message: __scodeStmt_message,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            exp1 = instExp(__scodeStmt_message.clone(), &scope, context, info)?;
            metamodelica::Ref::new(Statement::NFStatement::TERMINATE {
                message: exp1,
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
            })
        }
        SCode::Statement::ALG_REINIT {
            info,
            comment: __scodeStmt_comment,
            cref: __scodeStmt_cref,
            newValue: __scodeStmt_newValue,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            if !(Flags::isConfigFlagSet(
                Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
                literal!("reinitInAlgorithms"),
            )?) {
                Error::addSourceMessage(&(Error::REINIT_IN_ALGORITHM.clone()), metamodelica::nil(), info)?;
                return Err("fail");
            }
            if !(InstContext::inWhen(context)) {
                Error::addSourceMessage(&(Error::REINIT_NOT_IN_WHEN.clone()), metamodelica::nil(), info)?;
                return Err("fail");
            }
            exp1 = instExp(__scodeStmt_cref.clone(), &scope, context, info)?;
            exp2 = instExp(__scodeStmt_newValue.clone(), &scope, context, info)?;
            metamodelica::Ref::new(Statement::NFStatement::REINIT {
                cref: exp1,
                reinitExp: exp2,
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
            })
        }
        SCode::Statement::ALG_NORETCALL {
            info,
            comment: __scodeStmt_comment,
            exp: __scodeStmt_exp,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            exp1 = instExp(__scodeStmt_exp.clone(), &scope, context, info)?;
            metamodelica::Ref::new(Statement::NFStatement::NORETCALL {
                exp: exp1,
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
            })
        }
        SCode::Statement::ALG_WHILE {
            info,
            boolExpr: __scodeStmt_boolExpr,
            comment: __scodeStmt_comment,
            whileBody: __scodeStmt_whileBody,
        } => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut stmtl: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut next_context: i32;
            exp1 = instExp(__scodeStmt_boolExpr.clone(), &scope, context, info)?;
            next_context = InstContext::set(context, InstContext::WHILE.clone());
            stmtl = instStatements(__scodeStmt_whileBody.clone(), scope, next_context)?;
            metamodelica::Ref::new(Statement::NFStatement::WHILE {
                condition: exp1,
                body: stmtl,
                source: makeSource(__scodeStmt_comment.clone(), info.clone()),
            })
        }
        SCode::Statement::ALG_RETURN {
            comment: __scodeStmt_comment,
            info: __scodeStmt_info,
        } => {
            if !(InstContext::inFunction(context)) {
                Error::addSourceMessage(
                    &(Error::RETURN_OUTSIDE_FUNCTION.clone()),
                    metamodelica::nil(),
                    metamodelica::AsArg::as_arg(&__scodeStmt_info),
                )?;
                return Err("fail");
            }
            metamodelica::Ref::new(Statement::NFStatement::RETURN {
                source: makeSource(__scodeStmt_comment.clone(), __scodeStmt_info.clone()),
            })
        }
        SCode::Statement::ALG_BREAK {
            comment: __scodeStmt_comment,
            info: __scodeStmt_info,
        } => {
            if !(InstContext::inLoop(context)) {
                Error::addSourceMessage(
                    &(Error::BREAK_OUTSIDE_LOOP.clone()),
                    metamodelica::nil(),
                    metamodelica::AsArg::as_arg(&__scodeStmt_info),
                )?;
                return Err("fail");
            }
            metamodelica::Ref::new(Statement::NFStatement::BREAK {
                source: makeSource(__scodeStmt_comment.clone(), __scodeStmt_info.clone()),
            })
        }
        SCode::Statement::ALG_FAILURE {
            comment: __scodeStmt_comment,
            info: __scodeStmt_info,
            stmts: __scodeStmt_stmts,
        } => {
            let mut stmtl: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            stmtl = instStatements(__scodeStmt_stmts.clone(), scope, context)?;
            metamodelica::Ref::new(Statement::NFStatement::FAILURE {
                body: stmtl,
                source: makeSource(__scodeStmt_comment.clone(), __scodeStmt_info.clone()),
            })
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFInst.instStatement"));
                    __mm_s.push_str(&*literal!(" got unknown statement"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFInst.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(statement)
}

pub(crate) fn checkAssignmentRestriction(
    mut lhs: &metamodelica::Ref<Expression::NFExpression>,
    mut info: &SourceInfo,
) -> Result<()> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut res: metamodelica::Ref<Restriction::NFRestriction>;
    let () = (match &**lhs {
        Expression::CREF { cref: __lhs_cref, .. }
            if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__lhs_cref))) =>
        {
            Error::addSourceMessage(
                &(Error::ASSIGN_ITERATOR_ERROR.clone()),
                list![ComponentRef::toString(metamodelica::AsArg::as_arg(&__lhs_cref))?],
                info,
            )?;
            return Err("fail");
        }
        Expression::CREF { cref: __lhs_cref, .. }
            if (ComponentRef::isCref(metamodelica::AsArg::as_arg(&__lhs_cref))) =>
        {
            node = ComponentRef::node(metamodelica::AsArg::as_arg(&__lhs_cref))?;
            res = Class::restriction(&(NFInstNode::InstNode::getClass(node.clone())?));
            let () = (match &*res {
                Restriction::CLOCK => (),
                Restriction::CONNECTOR { .. } => (),
                Restriction::ENUMERATION => (),
                Restriction::RECORD { .. } => (),
                Restriction::TYPE => (),
                _ => {
                    Error::addSourceMessage(
                        &(Error::INVALID_SPECIALIZATION_IN_ASSIGNMENT.clone()),
                        list![NFInstNode::InstNode::name(&node)?, Restriction::toString(&res)],
                        info,
                    )?;
                    return Err("fail");
                }
            });
            ()
        }
        Expression::TUPLE {
            elements: __lhs_elements,
            ..
        } => {
            for mut e in &*__lhs_elements.clone() {
                checkAssignmentRestriction(metamodelica::AsArg::as_arg(&e), info)?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn addIteratorToScope(
    mut name: ArcStr,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut info: SourceInfo,
    mut iter_type: metamodelica::Ref<Type::NFType>,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut scope: metamodelica::Ref<InstNode::InstNode> = scope;
    let mut iterator: metamodelica::Ref<InstNode::InstNode>;
    let mut iter_comp: metamodelica::Ref<Component::NFComponent>;
    scope = NFInstNode::InstNode::openImplicitScope(scope);
    iter_comp = metamodelica::Ref::new(Component::NFComponent::ITERATOR {
        ty: iter_type,
        variability: Variability::CONTINUOUS.clone(),
        info: info,
    });
    iterator = NFInstNode::InstNode::fromComponent(name, iter_comp, scope.clone());
    scope = NFInstNode::InstNode::addIterator(iterator.clone(), scope)?;
    Ok((scope, iterator))
}

pub(crate) fn checkIteratorShadowing(
    mut name: ArcStr,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut info: SourceInfo,
) -> Result<()> {
    let () = (match &**scope {
        NFInstNode::InstNode::IMPLICIT_SCOPE {
            locals: __scope_locals, ..
        } => {
            for mut iter in &*__scope_locals.clone() {
                if metamodelica::stringEq(
                    &(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&iter))?),
                    &name,
                ) {
                    Error::addMultiSourceMessage(
                        &(Error::SHADOWED_ITERATOR.clone()),
                        &(list![name]),
                        &(list![NFInstNode::InstNode::info(metamodelica::AsArg::as_arg(&iter)), info]),
                    )?;
                    return Ok(());
                }
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub fn insertGeneratedInners(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut topScope: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<()> {
    let mut generated_inners: metamodelica::Ref<
        UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<InstNode::InstNode>>,
    >;
    let mut inner_comps: metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
    let mut n: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut name: ArcStr;
    let mut r#str: ArcStr;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut base_node: metamodelica::Ref<InstNode::InstNode>;
    let __pa0 = ::match_deref::match_deref! { match &(NFInstNode::InstNode::nodeType(topScope)?) {
        Deref @ NFInstNode::InstNodeType::TOP_SCOPE { generatedInners: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    generated_inners = metamodelica::Own::own(__pa0);
    if UnorderedMap::isEmpty(generated_inners.clone()) {
        return Ok(());
    }
    inner_comps = metamodelica::nil();
    let __range1 = UnorderedMap::valueArray(generated_inners)
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut n in __range1 {
        name = NFInstNode::InstNode::name(&n)?;
        checkTopLevelOuter(name.clone(), &n, node.clone(), context)?;
        if !(InstContext::inInstanceAPI(context)) {
            Error::addSourceMessage(
                &(Error::MISSING_INNER_ADDED.clone()),
                list![NFInstNode::InstNode::typeName(&n)?, name],
                &(NFInstNode::InstNode::info(&n)),
            )?;
        }
        if NFInstNode::InstNode::isComponent(&n)? {
            instComponent(
                n.clone(),
                &(Attributes::DEFAULT_ATTR().clone()),
                crate::NFModifier::Modifier::interned_NOMOD(),
                true,
                0,
                InstContext::CLASS.clone(),
                None,
                &(metamodelica::nil()),
            )?;
            if !(InstContext::inInstanceAPI(context)) {
                if '__try2: {
                    let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(SCodeUtil::lookupElementAnnotationBinding(&(unwrap_break_err!(NFInstNode::InstNode::definition(unwrap_break_err!(NFInstNode::InstNode::classScope(n.clone()), '__try2)), '__try2)), &(literal!("missingInnerMessage"))), '__try2)) {
                        Some(Deref @ Absyn::Exp::STRING { value: __pa3 }) => __pa3.clone(),
                        _ => break '__try2 Err::<_, _>("pattern mismatch"),
                    } };
                    r#str = metamodelica::Own::own(__pa3);
                    unwrap_break_err!(Error::addSourceMessage(&(Error::MISSING_INNER_MESSAGE.clone()), list![System::unescapedString(r#str.clone())], &(NFInstNode::InstNode::info(&n))), '__try2);
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
            }
            inner_comps = metamodelica::cons(Mutable::create(n), inner_comps);
        }
    }
    if !((inner_comps).is_empty()) {
        base_node = Class::lastBaseClass(node)?;
        cls = NFInstNode::InstNode::getClass(base_node.clone())?;
        cls_tree = ClassTree::appendComponentsToInstTree(inner_comps, Class::classTree(cls.clone())?)?;
        NFInstNode::InstNode::updateClass(Class::setClassTree(cls_tree, cls)?, base_node)?;
    }
    Ok(())
}

pub(crate) fn checkTopLevelOuter(
    mut name: ArcStr,
    mut outerNode: &metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<()> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut is_error: bool;
    if InstContext::inInstanceAPI(context) {
        return Ok(());
    }
    match '__try0: {
        (node, _) = unwrap_break_err!(Lookup::lookupSimpleName(name.clone(), scope.clone(), context), '__try0);
        if unwrap_break_err!(NFInstNode::InstNode::isInner(&node), '__try0) {
            is_error = !(InstContext::inRelaxed(context)
                || unwrap_break_err!(Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), literal!("nonStdTopLevelOuter")), '__try0));
            if is_error {
                unwrap_break_err!(Error::addSourceMessageAsError(Error::TOP_LEVEL_OUTER.clone(), list![name.clone()], &(NFInstNode::InstNode::info(&node))), '__try0);
            } else {
                unwrap_break_err!(Error::addSourceMessage(&(Error::TOP_LEVEL_OUTER.clone()), list![name.clone()], &(NFInstNode::InstNode::info(&node))), '__try0);
            }
        } else {
            unwrap_break_err!(Error::addMultiSourceMessage(&(Error::MISSING_INNER_NAME_CONFLICT.clone()), &(list![name.clone()]), &(list![NFInstNode::InstNode::info(&node), NFInstNode::InstNode::info(outerNode)])), '__try0);
            is_error = true;
        }
        Ok::<_, &'static str>((is_error.clone(),))
    } {
        Ok((__try0_o0,)) => {
            is_error = __try0_o0;
        }
        Err(_) => {
            is_error = false;
        }
    }
    if is_error {
        return Err("fail");
    }
    Ok(())
}

pub fn updateImplicitVariability(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut parentEval: bool,
    mut context: i32,
) -> Result<()> {
    let mut cls: metamodelica::Ref<Class::NFClass> = NFInstNode::InstNode::getClass(node.clone())?;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let () = (::match_deref::match_deref! { match &(&*cls) {
        Deref @ Class::INSTANCED_CLASS { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, sections: __cls_sections, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                updateImplicitVariabilityComp(c, parentEval, context)?;
            }
            Sections::apply(metamodelica::AsArg::as_arg(&__cls_sections), &({ let __pe_b1 = false; move |__pe_a0| updateImplicitVariabilityEq(&__pe_a0, __pe_b1.clone()) }), &move |__a0: metamodelica::Ref<Algorithm::NFAlgorithm>| updateImplicitVariabilityAlg(&__a0), &({ let __pe_b1 = false; move |__pe_a0| updateImplicitVariabilityEq(&__pe_a0, __pe_b1.clone()) }), &move |__a0: metamodelica::Ref<Algorithm::NFAlgorithm>| updateImplicitVariabilityAlg(&__a0))?;
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { baseClass: __cls_baseClass, .. } => {
            let __range0 = var_field!((*cls).dims, Class::NFClass::EXPANDED_DERIVED).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut dim in __range0 {
                Structural::markDimension(&dim)?;
            }
            updateImplicitVariability(__cls_baseClass.clone(), parentEval, context)?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                updateImplicitVariabilityComp(c, parentEval, context)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn updateImplicitVariabilityComp(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut parentEval: bool,
    mut context: i32,
) -> Result<()> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut c: metamodelica::Ref<Component::NFComponent>;
    if NFInstNode::InstNode::isEmpty(&component) {
        return Ok(());
    }
    node = NFInstNode::InstNode::resolveOuter(component.clone());
    c = NFInstNode::InstNode::component(&node)?;
    let () = (match &*c.clone() {
        Component::COMPONENT {
            binding,
            condition,
            attributes: __c_attributes,
            classInst: __c_classInst,
            ty: __c_ty,
            ..
        } => {
            let mut opt_eval: Option<bool>;
            let mut eval: bool;
            opt_eval = Component::getEvaluateAnnotation(&c)?;
            eval = Util::getOptionOrDefault(opt_eval.clone(), false);
            if (opt_eval).is_some() && !(eval) && __c_attributes.variability.clone() == Variability::PARAMETER.clone() {
                NFInstNode::InstNode::updateComponent(
                    Component::setVariability(Variability::NON_STRUCTURAL_PARAMETER.clone(), c),
                    node,
                )?;
            } else {
                if Structural::isStructuralComponent(
                    &c,
                    metamodelica::AsArg::as_arg(&__c_attributes),
                    binding.clone(),
                    node.clone(),
                    eval,
                    parentEval,
                    context,
                )? {
                    Structural::markComponent(c, node)?;
                }
            }
            for mut dim in &*Type::arrayDims(__c_ty.clone()) {
                Structural::markDimension(metamodelica::AsArg::as_arg(&dim))?;
            }
            if Binding::isBound(metamodelica::AsArg::as_arg(&binding)) {
                Structural::markExpSize(Binding::getUntypedExp(metamodelica::AsArg::as_arg(&binding))?)?;
            }
            if Binding::isBound(metamodelica::AsArg::as_arg(&condition)) {
                Structural::markExp(&(Binding::getUntypedExp(metamodelica::AsArg::as_arg(&condition))?))?;
            }
            if !(NFInstNode::InstNode::isEmpty(metamodelica::AsArg::as_arg(&__c_classInst))) {
                updateImplicitVariability(__c_classInst.clone(), eval || parentEval, context)?;
            }
            ()
        }
        Component::TYPE_ATTRIBUTE {
            modifier: __c_modifier, ..
        } if (listMember(
            NFInstNode::InstNode::name(&component)?,
            list![literal!("fixed"), literal!("stateSelect")],
        )) =>
        {
            let mut binding: metamodelica::Ref<Binding::NFBinding>;
            binding = Modifier::binding(metamodelica::AsArg::as_arg(&__c_modifier));
            if Binding::isBound(&binding) {
                Structural::markExp(&(Binding::getUntypedExp(&binding)?))?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn updateImplicitVariabilityEql(
    mut eql: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut inWhen: bool,
) -> Result<()> {
    for mut eq in &**eql {
        updateImplicitVariabilityEq(metamodelica::AsArg::as_arg(&eq), inWhen)?;
    }
    Ok(())
}

pub(crate) fn updateImplicitVariabilityEq(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut inWhen: bool,
) -> Result<()> {
    let () = (match &**eq {
        Equation::EQUALITY { lhs: __eq_lhs, .. } => {
            if inWhen {
                markImplicitWhenExp(__eq_lhs.clone())?;
            }
            ()
        }
        Equation::CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            Structural::markSubscriptsInExp(metamodelica::AsArg::as_arg(&__eq_lhs))?;
            Structural::markSubscriptsInExp(metamodelica::AsArg::as_arg(&__eq_rhs))?;
            ()
        }
        Equation::FOR { body: __eq_body, .. } => {
            updateImplicitVariabilityEql(metamodelica::AsArg::as_arg(&__eq_body), inWhen)?;
            ()
        }
        Equation::IF {
            branches: __eq_branches,
            ..
        } => {
            for mut branch in &*__eq_branches.clone() {
                let () = (match &*branch.clone() {
                    Equation::Branch::BRANCH {
                        body: __branch_body, ..
                    } => {
                        updateImplicitVariabilityEql(metamodelica::AsArg::as_arg(&__branch_body), inWhen)?;
                        ()
                    }
                    _ => return Err("match: no arm matched"),
                });
            }
            ()
        }
        Equation::WHEN {
            branches: __eq_branches,
            ..
        } => {
            for mut branch in &*__eq_branches.clone() {
                let () = (match &*branch.clone() {
                    Equation::Branch::BRANCH {
                        body: __branch_body, ..
                    } => {
                        updateImplicitVariabilityEql(metamodelica::AsArg::as_arg(&__branch_body), true)?;
                        ()
                    }
                    _ => return Err("match: no arm matched"),
                });
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn updateImplicitVariabilityAlg(mut alg: &metamodelica::Ref<Algorithm::NFAlgorithm>) -> Result<()> {
    updateImplicitVariabilityStmts(&alg.statements, false)?;
    Ok(())
}

pub(crate) fn updateImplicitVariabilityStmts(
    mut stmtl: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut inWhen: bool,
) -> Result<()> {
    for mut s in &**stmtl {
        updateImplicitVariabilityStmt(metamodelica::AsArg::as_arg(&s), inWhen)?;
    }
    Ok(())
}

pub(crate) fn updateImplicitVariabilityStmt(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut inWhen: bool,
) -> Result<()> {
    let () = (match &**stmt {
        Statement::ASSIGNMENT { lhs: __stmt_lhs, .. } => {
            if inWhen {
                markImplicitWhenExp(__stmt_lhs.clone())?;
            }
            ()
        }
        Statement::FOR { body: __stmt_body, .. } => {
            if inWhen {
                updateImplicitVariabilityStmts(metamodelica::AsArg::as_arg(&__stmt_body), true)?;
            }
            ()
        }
        Statement::IF {
            branches: __stmt_branches,
            ..
        } => {
            if inWhen {
                for mut branch in &*__stmt_branches.clone() {
                    updateImplicitVariabilityStmts(&(Util::tuple22(branch.clone())), true)?;
                }
            }
            ()
        }
        Statement::WHEN {
            branches: __stmt_branches,
            ..
        } => {
            for mut branch in &*__stmt_branches.clone() {
                updateImplicitVariabilityStmts(&(Util::tuple22(branch.clone())), true)?;
            }
            ()
        }
        Statement::WHILE { body: __stmt_body, .. } => {
            if inWhen {
                updateImplicitVariabilityStmts(metamodelica::AsArg::as_arg(&__stmt_body), true)?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn markImplicitWhenExp(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<()> {
    Expression::apply(exp, &move |__a0: metamodelica::Ref<Expression::NFExpression>| {
        markImplicitWhenExp_traverser(&__a0)
    })?;
    Ok(())
}

pub(crate) fn markImplicitWhenExp_traverser(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match exp {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut comp: metamodelica::Ref<Component::NFComponent>;
            node = ComponentRef::node(var_field!((**exp).cref, Expression::NFExpression::CREF))?;
            if NFInstNode::InstNode::isComponent(&node)? {
                comp = NFInstNode::InstNode::component(&node)?;
                if Component::variability(&comp)? == Variability::CONTINUOUS.clone() {
                    comp = Component::setVariability(Variability::IMPLICITLY_DISCRETE.clone(), comp);
                    NFInstNode::InstNode::updateComponent(comp, node)?;
                }
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkPartialClass(mut node: &metamodelica::Ref<InstNode::InstNode>, mut context: i32) -> Result<()> {
    if NFInstNode::InstNode::isPartial(node)? && !(InstContext::inRelaxed(context)) {
        Error::addSourceMessage(
            &(Error::INST_PARTIAL_CLASS.clone()),
            list![NFInstNode::InstNode::name(node)?],
            &(NFInstNode::InstNode::info(node)),
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn checkInstanceRestriction(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut path: metamodelica::Ref<Path>,
    mut context: i32,
) -> Result<()> {
    let mut elem: metamodelica::Ref<SCode::Element>;
    if InstContext::inRelaxed(context) {
        return Ok(());
    }
    elem = NFInstNode::InstNode::definition(node.clone())?;
    if SCodeUtil::isFunction(&elem) || SCodeUtil::isPackage(&elem) {
        Error::addSourceMessage(
            &(Error::INST_INVALID_RESTRICTION.clone()),
            list![
                AbsynUtil::pathString(path, literal!("."), true, false)?,
                SCodeDump::restrString(&(SCodeUtil::getClassRestriction(&elem)?))?
            ],
            &(NFInstNode::InstNode::info(&node)),
        )?;
        return Err("fail");
    }
    Ok(())
}
