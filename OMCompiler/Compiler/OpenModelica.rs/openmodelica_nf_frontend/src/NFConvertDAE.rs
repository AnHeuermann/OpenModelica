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
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

pub fn convert(
    mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
) -> Result<(DAE::DAElist, metamodelica::Ref<AvlTreePathFunction::Tree>)> {
    let mut dae: DAE::DAElist;
    let mut daeFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>;
    daeFunctions = convertFunctionTree(functions)?;
    dae = convertModel(flatModel)?;
    execStat(&(literal!("NFConvertDAE.convert")))?;
    Ok((dae, daeFunctions))
}

pub fn convertModel(mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist;
    let mut elems: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut class_elem: metamodelica::Ref<DAE::Element>;
    elems = convertVariables(flatModel.variables.clone(), metamodelica::nil())?;
    elems = convertEquations(flatModel.equations.clone(), elems)?;
    elems = convertInitialEquations(flatModel.initialEquations.clone(), elems)?;
    elems = convertAlgorithms(flatModel.algorithms.clone(), elems)?;
    elems = convertInitialAlgorithms(flatModel.initialAlgorithms.clone(), elems)?;
    class_elem = metamodelica::Ref::new(DAE::Element::COMP {
        ident: FlatModel::fullName(flatModel)?,
        dAElist: elems,
        source: flatModel.source.clone(),
        comment: ElementSource::getOptComment(&flatModel.source)?,
    });
    dae = DAE::DAElist {
        elementLst: list![class_elem],
    };
    Ok(dae)
}

pub fn convertStatements(
    mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    elements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
        for mut s in (statements).into_iter().cloned() {
            let __x = convertStatement(&(s.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(elements)
}

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct VariableConversionSettings {
    pub isFunctionParameter: bool,
    pub addTypeToSource: bool,
}

impl metamodelica::gc::MMTrace for VariableConversionSettings {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.isFunctionParameter, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.addTypeToSource, __mmv)?;
        Ok(())
    }
}
impl Default for VariableConversionSettings {
    fn default() -> Self {
        Self {
            isFunctionParameter: Default::default(),
            addTypeToSource: Default::default(),
        }
    }
}

pub type VARIABLE_CONVERSION_SETTINGS = VariableConversionSettings;

pub(crate) static FUNCTION_VARIABLE_CONVERSION_SETTINGS: VariableConversionSettings = VariableConversionSettings {
    isFunctionParameter: true,
    addTypeToSource: false,
};

fn convertVariables(
    mut variables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    let mut settings: VariableConversionSettings;
    let mut rest: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut last_rest: metamodelica::Ref<ComponentRef::NFComponentRef> = crate::NFComponentRef::interned_EMPTY();
    let mut encrypted: bool;
    let mut rest_encrypted: bool = false;
    settings = VariableConversionSettings {
        isFunctionParameter: false,
        addTypeToSource: Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?,
    };
    for mut var in &*variables.reverse() {
        if ComponentRef::isCref(&var.name) {
            rest = ComponentRef::rest(&var.name)?;
            if !(referenceEq(&*(&*rest), &*(&*last_rest))) {
                last_rest = rest.clone();
                rest_encrypted = Variable::isEncryptedName(rest)?;
            }
            encrypted = rest_encrypted || Variable::isEncryptedNode(&(ComponentRef::node(&var.name)?));
        } else {
            encrypted = false;
        }
        elements = metamodelica::cons(
            convertVariable(metamodelica::AsArg::as_arg(&var), settings, encrypted)?,
            elements,
        );
    }
    Ok(elements)
}

fn convertVariable(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut settings: VariableConversionSettings,
    mut encrypted: bool,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut daeVar: metamodelica::Ref<DAE::Element>;
    let mut var_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut binding_exp: Option<metamodelica::Ref<DAE::Exp>>;
    binding_exp = Binding::toDAEExp(&var.binding)?;
    var_attr = convertVarAttributes(&var.typeAttributes, &var.ty, &var.attributes)?;
    daeVar = makeDAEVar(
        &var.name,
        var.ty.clone(),
        binding_exp,
        &var.attributes,
        var.visibility.clone(),
        var_attr,
        var.comment.clone(),
        settings,
        var.info.clone(),
        encrypted,
    )?;
    Ok(daeVar)
}

fn makeDAEVar(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut binding: Option<metamodelica::Ref<DAE::Exp>>,
    mut attr: &metamodelica::Ref<Attributes::NFAttributes>,
    mut vis: Visibility,
    mut vattr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut settings: VariableConversionSettings,
    mut info: SourceInfo,
    mut encrypted: bool,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut var: metamodelica::Ref<DAE::Element>;
    let mut dcref: metamodelica::Ref<DAE::ComponentRef>;
    let mut dty: metamodelica::Ref<DAE::Type>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    dcref = ComponentRef::toDAE(cref)?;
    dty = Type::toDAE(
        &(if (settings.isFunctionParameter.clone()) {
            Type::arrayElementType(&ty)
        } else {
            ty
        }),
        true,
    )?;
    source = ElementSource::createElementSource(
        info,
        None,
        &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
        (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
    );
    if settings.addTypeToSource.clone() {
        source = addComponentTypeToSource(cref, source)?;
    }
    var = (match &**attr {
        Attributes::ATTRIBUTES { .. } => metamodelica::Ref::new(DAE::Element::VAR {
            componentRef: dcref.clone(),
            kind: Prefixes::variabilityToDAE(attr.variability.clone()),
            direction: Prefixes::directionToDAE(attr.direction.clone()),
            parallelism: Prefixes::parallelismToDAE(attr.parallelism.clone())?,
            protection: Prefixes::visibilityToDAE(vis),
            ty: dty,
            binding: binding,
            dims: ComponentReferenceBasics::crefDims(&dcref)?,
            connectorType: Prefixes::ConnectorType::toDAE(attr.connectorType.clone()),
            source: source,
            variableAttributesOption: vattr,
            comment: Some(comment),
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            encrypted: encrypted,
        }),
        _ => metamodelica::Ref::new(DAE::Element::VAR {
            componentRef: dcref,
            kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE,
            direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
            parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
            protection: Prefixes::visibilityToDAE(vis),
            ty: dty,
            binding: binding,
            dims: metamodelica::nil(),
            connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
            source: source,
            variableAttributesOption: vattr,
            comment: Some(comment),
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            encrypted: encrypted,
        }),
    });
    Ok(var)
}

fn addComponentTypeToSource<'__b>(
    mut cref: &'__b metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    '__tco: loop {
        match &**cref {
            ComponentRef::CREF { .. } => {
                source = addComponentLevelTypeToSource(InstNode::parent(&(ComponentRef::node(cref)?))?, source)?;
                {
                    (cref, source) = (
                        var_field!((**cref).restCref, ComponentRef::NFComponentRef::CREF),
                        source,
                    );
                    continue '__tco;
                }
            }
            _ => return Ok(source),
        }
    }
}

fn addComponentLevelTypeToSource(
    mut parentNode: metamodelica::Ref<InstNode::InstNode>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    let mut concrete: metamodelica::Ref<InstNode::InstNode>;
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    let mut concretePath: metamodelica::Ref<Absyn::Path>;
    let mut p: metamodelica::Ref<Absyn::Path>;
    let mut chain: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    let mut visPath: Option<metamodelica::Ref<Absyn::Path>> = None;
    concrete = InstNode::classScope(InstNode::getDerivedNode(parentNode.clone(), true)?)?;
    concretePath = InstNode::scopePath(concrete, InstNode::ScopeType::RELATIVE.clone(), false)?;
    if !(isVisualizerLeafName(&concretePath)) {
        n = InstNode::classScope(parentNode)?;
        while InstNode::isBaseClass(&n) {
            p = InstNode::scopePath(n.clone(), InstNode::ScopeType::RELATIVE.clone(), true)?;
            chain = metamodelica::cons(p.clone(), chain);
            if (visPath).is_none() && isVisualizerLeafName(&p) {
                visPath = Some(p);
            }
            n = InstNode::classScope(InstNode::getDerivedNode(n, false)?)?;
        }
    }
    if (visPath).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(visPath) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        p = metamodelica::Own::own(__pa0);
        chain = listAppend(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
                for mut c in (chain).into_iter().cloned() {
                    if !(!(AbsynUtil::pathEqual(&(c.clone()), &p))) {
                        continue;
                    }
                    let __x = c.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            list![concretePath],
        );
        for mut c in &*chain.reverse() {
            source = ElementSource::addElementSourceType(source, c.clone())?;
        }
        source = ElementSource::addElementSourceType(source, p)?;
    } else {
        source = ElementSource::addElementSourceType(source, concretePath)?;
    }
    Ok(source)
}

fn isVisualizerLeafName(mut path: &metamodelica::Ref<Absyn::Path>) -> bool {
    let mut isVisualizer: bool;
    isVisualizer = (::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(path)) {
        Deref @ "Shape" => true,
        Deref @ "Vector" => true,
        Deref @ "Surface" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isVisualizer
}

fn convertVarAttributes(
    mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut compAttrs: &metamodelica::Ref<Attributes::NFAttributes>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut attributes: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut is_final: bool;
    let mut is_final_opt: Option<bool>;
    is_final = compAttrs.isFinal.clone() || compAttrs.variability.clone() == Variability::STRUCTURAL_PARAMETER.clone();
    if (attrs).is_empty() && !(is_final) {
        attributes = None;
        return Ok(attributes);
    }
    is_final_opt = Some(is_final);
    attributes = (match &*(Type::arrayElementType(ty)) {
        Type::REAL => convertRealVarAttributes(attrs, is_final_opt)?,
        Type::INTEGER => convertIntVarAttributes(attrs, is_final_opt)?,
        Type::BOOLEAN => convertBoolVarAttributes(attrs, is_final_opt)?,
        Type::STRING => convertStringVarAttributes(attrs, is_final_opt)?,
        Type::ENUMERATION { .. } => convertEnumVarAttributes(attrs, is_final_opt)?,
        _ => None,
    });
    Ok(attributes)
}

fn convertRealVarAttributes(
    mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    mut isFinal: Option<bool>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut attributes: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut name: ArcStr;
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let mut quantity: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut unit: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut displayUnit: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut min: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut max: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut fixed: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut nominal: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut state_select: Option<DAE::StateSelect> = None;
    let mut uncertain: Option<DAE::Uncertainty> = None;
    let mut start_origin: Option<DAE::StartOrigin> = None;
    for mut attr in &**attrs {
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
            Deref @ "displayUnit" => {
                displayUnit = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "fixed" => {
                fixed = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "max" => {
                max = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "min" => {
                min = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "nominal" => {
                nominal = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "quantity" => {
                quantity = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "start" => {
                start = convertVarAttribute(&b)?;
                start_origin = convertStartOrigin(b)?;
                ()
            },
            Deref @ "stateSelect" => {
                state_select = convertStateSelectAttribute(&b)?;
                ()
            },
            Deref @ "unbounded" => (),
            Deref @ "uncertain" => {
                uncertain = convertUncertaintyAttribute(&b)?;
                ()
            },
            Deref @ "unit" => {
                unit = convertVarAttribute(&b)?;
                ()
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertRealVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    attributes = Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL {
        quantity: quantity,
        unit: unit,
        displayUnit: displayUnit,
        min: min,
        max: max,
        start: start,
        fixed: fixed,
        nominal: nominal,
        stateSelectOption: state_select,
        uncertainOption: uncertain,
        distributionOption: None,
        equationBound: None,
        isProtected: None,
        finalPrefix: isFinal,
        startOrigin: start_origin,
    }));
    Ok(attributes)
}

fn convertIntVarAttributes(
    mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    mut isFinal: Option<bool>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut attributes: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut name: ArcStr;
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let mut quantity: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut min: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut max: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut fixed: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start_origin: Option<DAE::StartOrigin> = None;
    for mut attr in &**attrs {
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
            Deref @ "quantity" => {
                quantity = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "min" => {
                min = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "max" => {
                max = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "start" => {
                start = convertVarAttribute(&b)?;
                start_origin = convertStartOrigin(b)?;
                ()
            },
            Deref @ "fixed" => {
                fixed = convertVarAttribute(&b)?;
                ()
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertIntVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    attributes = Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT {
        quantity: quantity,
        min: min,
        max: max,
        start: start,
        fixed: fixed,
        uncertainOption: None,
        distributionOption: None,
        equationBound: None,
        isProtected: None,
        finalPrefix: isFinal,
        startOrigin: start_origin,
    }));
    Ok(attributes)
}

fn convertBoolVarAttributes(
    mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    mut isFinal: Option<bool>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut attributes: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut name: ArcStr;
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let mut quantity: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut fixed: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start_origin: Option<DAE::StartOrigin> = None;
    for mut attr in &**attrs {
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
            Deref @ "quantity" => {
                quantity = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "start" => {
                start = convertVarAttribute(&b)?;
                start_origin = convertStartOrigin(b)?;
                ()
            },
            Deref @ "fixed" => {
                fixed = convertVarAttribute(&b)?;
                ()
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertBoolVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    attributes = Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL {
        quantity: quantity,
        start: start,
        fixed: fixed,
        equationBound: None,
        isProtected: None,
        finalPrefix: isFinal,
        startOrigin: start_origin,
    }));
    Ok(attributes)
}

fn convertStringVarAttributes(
    mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    mut isFinal: Option<bool>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut attributes: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut name: ArcStr;
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let mut quantity: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut fixed: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start_origin: Option<DAE::StartOrigin> = None;
    for mut attr in &**attrs {
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
            Deref @ "quantity" => {
                quantity = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "start" => {
                start = convertVarAttribute(&b)?;
                start_origin = convertStartOrigin(b)?;
                ()
            },
            Deref @ "fixed" => {
                fixed = convertVarAttribute(&b)?;
                ()
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertStringVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    attributes = Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING {
        quantity: quantity,
        start: start,
        fixed: fixed,
        equationBound: None,
        isProtected: None,
        finalPrefix: isFinal,
        startOrigin: start_origin,
    }));
    Ok(attributes)
}

fn convertEnumVarAttributes(
    mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    mut isFinal: Option<bool>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut attributes: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut name: ArcStr;
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let mut quantity: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut min: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut max: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut fixed: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start_origin: Option<DAE::StartOrigin> = None;
    for mut attr in &**attrs {
        (name, b) = attr.clone();
        let () = (::match_deref::match_deref! { match &(name.clone()) {
            Deref @ "fixed" => {
                fixed = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "max" => {
                max = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "min" => {
                min = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "quantity" => {
                quantity = convertVarAttribute(&b)?;
                ()
            },
            Deref @ "start" => {
                start = convertVarAttribute(&b)?;
                start_origin = convertStartOrigin(b)?;
                ()
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertEnumVarAttributes")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    attributes = Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION {
        quantity: quantity,
        min: min,
        max: max,
        start: start,
        fixed: fixed,
        equationBound: None,
        isProtected: None,
        finalPrefix: isFinal,
        startOrigin: start_origin,
    }));
    Ok(attributes)
}

fn convertVarAttribute(
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut attribute: Option<metamodelica::Ref<DAE::Exp>> =
        Some(Expression::toDAE(Binding::getTypedExp(binding)?, false)?);
    Ok(attribute)
}

fn convertStateSelectAttribute(
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
) -> Result<Option<DAE::StateSelect>> {
    let mut stateSelect: Option<DAE::StateSelect>;
    let mut name: ArcStr;
    name = getStateSelectName(Expression::arrayFirstScalar(Binding::getTypedExp(binding)?)?)?;
    stateSelect = Some(lookupStateSelectMember(&name)?);
    Ok(stateSelect)
}

fn getStateSelectName(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<ArcStr> {
    '__tco: loop {
        let mut e: metamodelica::Ref<Expression::NFExpression>;
        ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::ENUM_LITERAL { name: __exp_name, .. } => return Ok(__exp_name.clone()),
            Deref @ Expression::CREF { cref: __exp_cref, .. } => return Ok(ComponentRef::nodeName(metamodelica::AsArg::as_arg(&__exp_cref))?),
            Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: __esc_e, .. } } => {
                e = (*__esc_e).clone();
                { exp = e.clone(); continue '__tco; }
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.getStateSelectName")); __mm_s.push_str(&*literal!(" got invalid StateSelect expression ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lookupStateSelectMember(mut name: &ArcStr) -> Result<DAE::StateSelect> {
    let mut stateSelect: DAE::StateSelect;
    stateSelect = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "never" => openmodelica_frontend_types::DAE::StateSelect::NEVER,
        Deref @ "avoid" => openmodelica_frontend_types::DAE::StateSelect::AVOID,
        Deref @ "default" => openmodelica_frontend_types::DAE::StateSelect::DEFAULT,
        Deref @ "prefer" => openmodelica_frontend_types::DAE::StateSelect::PREFER,
        Deref @ "always" => openmodelica_frontend_types::DAE::StateSelect::ALWAYS,
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.lookupStateSelectMember")); __mm_s.push_str(&*literal!(" got unknown StateSelect literal ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stateSelect)
}

fn convertUncertaintyAttribute(
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
) -> Result<Option<DAE::Uncertainty>> {
    let mut stateSelect: Option<DAE::Uncertainty>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut name: ArcStr;
    let mut exp: metamodelica::Ref<Expression::NFExpression> =
        Expression::arrayFirstScalar(Binding::getTypedExp(binding)?)?;
    name = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ENUM_LITERAL { name: __exp_name, .. } => __exp_name.clone(),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => ComponentRef::nodeName(var_field!((*exp).cref, Expression::NFExpression::CREF))?,
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertUncertaintyAttribute")); __mm_s.push_str(&*literal!(" got invalid Uncertainty expression ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stateSelect = Some(lookupUncertaintyMember(&name)?);
    Ok(stateSelect)
}

fn lookupUncertaintyMember(mut name: &ArcStr) -> Result<DAE::Uncertainty> {
    let mut stateSelect: DAE::Uncertainty;
    stateSelect = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "given" => openmodelica_frontend_types::DAE::Uncertainty::GIVEN,
        Deref @ "sought" => openmodelica_frontend_types::DAE::Uncertainty::SOUGHT,
        Deref @ "refine" => openmodelica_frontend_types::DAE::Uncertainty::REFINE,
        Deref @ "propagate" => openmodelica_frontend_types::DAE::Uncertainty::PROPAGATE,
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.lookupUncertaintyMember")); __mm_s.push_str(&*literal!(" got unknown Uncertainty literal ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stateSelect)
}

fn convertStartOrigin(mut binding: metamodelica::Ref<Binding::NFBinding>) -> Result<Option<DAE::StartOrigin>> {
    let mut startOrigin: Option<DAE::StartOrigin>;
    startOrigin = Some(if (Binding::isFromType(&binding)) {
        DAE::StartOrigin::TYPE_CONFIDENCE {
            level: Binding::confidence(&binding),
        }
    } else {
        DAE::StartOrigin::CONFIDENCE {
            actual: Binding::actualConfidence(binding.clone())?,
            raw: Binding::confidence(&binding),
        }
    });
    Ok(startOrigin)
}

fn convertEquations(
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    for mut eq in &*equations.reverse() {
        elements = convertEquation(metamodelica::AsArg::as_arg(&eq), elements)?;
    }
    Ok(elements)
}

fn convertEquation(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    elements = (::match_deref::match_deref! { match eq {
        Deref @ Equation::EQUALITY { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: rhs @ Deref @ Expression::CREF { .. }, source: __eq_source, ty: __eq_ty, .. } if (Type::isScalarBuiltin(__eq_ty.clone())?) => {
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
            cr1 = ComponentRef::toDAE(var_field!((**lhs).cref, Expression::NFExpression::CREF))?;
            cr2 = ComponentRef::toDAE(var_field!((**rhs).cref, Expression::NFExpression::CREF))?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::EQUEQUATION { cr1: cr1, cr2: cr2, source: __eq_source.clone() }), elements)
        },
        Deref @ Equation::EQUALITY { lhs: __eq_lhs, rhs: __eq_rhs, source: __eq_source, ty: __eq_ty, .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            e1 = Expression::toDAE(__eq_lhs.clone(), false)?;
            e2 = Expression::toDAE(__eq_rhs.clone(), false)?;
            metamodelica::cons(if (Type::isComplex(metamodelica::AsArg::as_arg(&__eq_ty))) {metamodelica::Ref::new(DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source: __eq_source.clone() })} else if (Type::isArray(metamodelica::AsArg::as_arg(&__eq_ty))) {metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
        for mut d in (Type::arrayDims(__eq_ty.clone())).into_iter().cloned() {
            let __x = Dimension::toDAE(&(d.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), exp: e1, array: e2, source: __eq_source.clone() })} else {metamodelica::Ref::new(DAE::Element::EQUATION { exp: e1, scalar: e2, source: __eq_source.clone() })}, elements)
        },
        Deref @ Equation::FOR { .. } => {
            metamodelica::cons(convertForEquation(eq, false)?, elements)
        },
        Deref @ Equation::IF { branches: __eq_branches, source: __eq_source, .. } => {
            metamodelica::cons(convertIfEquation(metamodelica::AsArg::as_arg(&__eq_branches), __eq_source.clone(), false)?, elements)
        },
        Deref @ Equation::WHEN { branches: __eq_branches, source: __eq_source, .. } => {
            metamodelica::cons(convertWhenEquation(__eq_branches.clone(), __eq_source.clone())?, elements)
        },
        Deref @ Equation::ASSERT { condition: __eq_condition, level: __eq_level, message: __eq_message, source: __eq_source, .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut e3: metamodelica::Ref<DAE::Exp>;
            e1 = Expression::toDAE(__eq_condition.clone(), false)?;
            e2 = Expression::toDAE(__eq_message.clone(), false)?;
            e3 = Expression::toDAE(__eq_level.clone(), false)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::ASSERT { condition: e1, message: e2, level: e3, source: __eq_source.clone() }), elements)
        },
        Deref @ Equation::TERMINATE { message: __eq_message, source: __eq_source, .. } => {
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::TERMINATE { message: Expression::toDAE(__eq_message.clone(), false)?, source: __eq_source.clone() }), elements)
        },
        Deref @ Equation::REINIT { cref: __eq_cref, reinitExp: __eq_reinitExp, source: __eq_source, .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            cr1 = ComponentRef::toDAE(&(Expression::toCref(metamodelica::AsArg::as_arg(&__eq_cref))?))?;
            e1 = Expression::toDAE(__eq_reinitExp.clone(), false)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::REINIT { componentRef: cr1, exp: e1, source: __eq_source.clone() }), elements)
        },
        Deref @ Equation::NORETCALL { exp: __eq_exp, source: __eq_source, .. } => {
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::NORETCALL { exp: Expression::toDAE(__eq_exp.clone(), false)?, source: __eq_source.clone() }), elements)
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertEquation")); __mm_s.push_str(&*literal!(" got unknown equation ")); __mm_s.push_str(&*Equation::toString(eq, literal!(""))?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

fn convertForEquation(
    mut forEquation: &metamodelica::Ref<Equation::NFEquation>,
    mut isInitial: bool,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut forDAE: metamodelica::Ref<DAE::Element>;
    let mut iterator: metamodelica::Ref<InstNode::InstNode>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut dbody: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*forEquation)) {
        Deref @ Equation::FOR { iterator: __pa0, range: Some(__pa1), body: __pa2, source: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iterator = metamodelica::Own::own(__pa0);
    range = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    source = metamodelica::Own::own(__pa3);
    if isInitial {
        dbody = convertInitialEquations(body, metamodelica::nil())?;
    } else {
        dbody = convertEquations(body, metamodelica::nil())?;
    }
    let __pa4 = ::match_deref::match_deref! { match &(InstNode::component(&iterator)?) {
        Deref @ Component::ITERATOR { ty: __pa4, .. } => __pa4.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa4);
    if isInitial {
        forDAE = metamodelica::Ref::new(DAE::Element::INITIAL_FOR_EQUATION {
            type_: Type::toDAE(&ty, true)?,
            iterIsArray: Type::isArray(&ty),
            iter: InstNode::name(&iterator)?,
            index: 0,
            range: Expression::toDAE(range, false)?,
            equations: dbody,
            source: source,
        });
    } else {
        forDAE = metamodelica::Ref::new(DAE::Element::FOR_EQUATION {
            type_: Type::toDAE(&ty, true)?,
            iterIsArray: Type::isArray(&ty),
            iter: InstNode::name(&iterator)?,
            index: 0,
            range: Expression::toDAE(range, false)?,
            equations: dbody,
            source: source,
        });
    }
    Ok(forDAE)
}

fn convertIfEquation(
    mut ifBranches: &metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut isInitial: bool,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut ifEquation: metamodelica::Ref<DAE::Element>;
    let mut conds: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut branches: metamodelica::List<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> =
        metamodelica::nil();
    let mut dconds: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut dbranches: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
    let mut else_branch: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    for mut branch in &**ifBranches {
        (conds, branches) = (match &*branch.clone() {
            Equation::Branch::BRANCH {
                body: __branch_body,
                condition: __branch_condition,
                ..
            } => (
                metamodelica::cons(__branch_condition.clone(), conds),
                metamodelica::cons(__branch_body.clone(), branches),
            ),
            Equation::Branch::INVALID_BRANCH { .. } => {
                Equation::Branch::triggerErrors(metamodelica::AsArg::as_arg(&branch))?;
                return Err("fail");
            }
        });
    }
    dbranches = if (isInitial) {
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>> =
                metamodelica::nil();
            for mut b in (branches).into_iter().cloned() {
                let __x = convertInitialEquations(b.clone(), metamodelica::nil())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    } else {
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>> =
                metamodelica::nil();
            for mut b in (branches).into_iter().cloned() {
                let __x = convertEquations(b.clone(), metamodelica::nil())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    };
    if Expression::isTrue(&((conds).head().cloned()?)) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dbranches) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        else_branch = metamodelica::Own::own(__pa0);
        dbranches = metamodelica::Own::own(__pa1);
        conds = (conds).rest()?;
    } else {
        else_branch = metamodelica::nil();
    }
    dconds = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut c in (conds).into_iter().cloned() {
            let __x = Expression::toDAE(c.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc
    });
    dbranches = metamodelica::Dangerous::listReverseInPlace(dbranches);
    ifEquation = if (isInitial) {
        metamodelica::Ref::new(DAE::Element::INITIAL_IF_EQUATION {
            condition1: dconds,
            equations2: dbranches,
            equations3: else_branch,
            source: source,
        })
    } else {
        metamodelica::Ref::new(DAE::Element::IF_EQUATION {
            condition1: dconds,
            equations2: dbranches,
            equations3: else_branch,
            source: source,
        })
    };
    Ok(ifEquation)
}

fn convertWhenEquation(
    mut whenBranches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut whenEquation: metamodelica::Ref<DAE::Element>;
    let mut cond: metamodelica::Ref<DAE::Exp>;
    let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut when_eq: Option<metamodelica::Ref<DAE::Element>> = None;
    for mut b in &*whenBranches.reverse() {
        when_eq = (match &*b.clone() {
            Equation::Branch::BRANCH {
                body: __b_body,
                condition: __b_condition,
                ..
            } => {
                cond = Expression::toDAE(__b_condition.clone(), false)?;
                els = convertEquations(__b_body.clone(), metamodelica::nil())?;
                Some(metamodelica::Ref::new(DAE::Element::WHEN_EQUATION {
                    condition: cond,
                    equations: els,
                    elsewhen_: when_eq,
                    source: source.clone(),
                }))
            }
            _ => return Err("match: no arm matched"),
        });
    }
    let __pa0 = ::match_deref::match_deref! { match &(when_eq) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    whenEquation = metamodelica::Own::own(__pa0);
    Ok(whenEquation)
}

fn convertInitialEquations(
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    for mut eq in &*equations.reverse() {
        elements = convertInitialEquation(metamodelica::AsArg::as_arg(&eq), elements)?;
    }
    Ok(elements)
}

fn convertInitialEquation(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    elements = (match &**eq {
        Equation::EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            source: __eq_source,
            ty: __eq_ty,
            ..
        } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            e1 = Expression::toDAE(__eq_lhs.clone(), false)?;
            e2 = Expression::toDAE(__eq_rhs.clone(), false)?;
            metamodelica::cons(
                if (Type::isComplex(metamodelica::AsArg::as_arg(&__eq_ty))) {
                    metamodelica::Ref::new(DAE::Element::INITIAL_COMPLEX_EQUATION {
                        lhs: e1,
                        rhs: e2,
                        source: __eq_source.clone(),
                    })
                } else if (Type::isArray(metamodelica::AsArg::as_arg(&__eq_ty))) {
                    metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION {
                        dimension: ({
                            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
                            for mut d in (Type::arrayDims(__eq_ty.clone())).into_iter().cloned() {
                                let __x = Dimension::toDAE(&(d.clone()))?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        exp: e1,
                        array: e2,
                        source: __eq_source.clone(),
                    })
                } else {
                    metamodelica::Ref::new(DAE::Element::INITIALEQUATION {
                        exp1: e1,
                        exp2: e2,
                        source: __eq_source.clone(),
                    })
                },
                elements,
            )
        }
        Equation::FOR { .. } => metamodelica::cons(convertForEquation(eq, true)?, elements),
        Equation::IF {
            branches: __eq_branches,
            source: __eq_source,
            ..
        } => metamodelica::cons(
            convertIfEquation(metamodelica::AsArg::as_arg(&__eq_branches), __eq_source.clone(), true)?,
            elements,
        ),
        Equation::ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            source: __eq_source,
            ..
        } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut e3: metamodelica::Ref<DAE::Exp>;
            e1 = Expression::toDAE(__eq_condition.clone(), false)?;
            e2 = Expression::toDAE(__eq_message.clone(), false)?;
            e3 = Expression::toDAE(__eq_level.clone(), false)?;
            metamodelica::cons(
                metamodelica::Ref::new(DAE::Element::INITIAL_ASSERT {
                    condition: e1,
                    message: e2,
                    level: e3,
                    source: __eq_source.clone(),
                }),
                elements,
            )
        }
        Equation::TERMINATE {
            message: __eq_message,
            source: __eq_source,
            ..
        } => metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::INITIAL_TERMINATE {
                message: Expression::toDAE(__eq_message.clone(), false)?,
                source: __eq_source.clone(),
            }),
            elements,
        ),
        Equation::NORETCALL {
            exp: __eq_exp,
            source: __eq_source,
            ..
        } => metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::INITIAL_NORETCALL {
                exp: Expression::toDAE(__eq_exp.clone(), false)?,
                source: __eq_source.clone(),
            }),
            elements,
        ),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFConvertDAE.convertInitialEquation"));
                    __mm_s.push_str(&*literal!(" got unknown equation "));
                    __mm_s.push_str(&*Equation::toString(eq, literal!(""))?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(elements)
}

fn convertAlgorithms(
    mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    for mut alg in &*algorithms.reverse() {
        elements = convertAlgorithm(metamodelica::AsArg::as_arg(&alg), elements)?;
    }
    Ok(elements)
}

fn convertAlgorithm(
    mut alg: &metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut dalg: metamodelica::Ref<DAE::Algorithm>;
    stmts = convertStatements(alg.statements.clone())?;
    dalg = metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts });
    elements = metamodelica::cons(
        metamodelica::Ref::new(DAE::Element::ALGORITHM {
            algorithm_: dalg,
            source: alg.source.clone(),
        }),
        elements,
    );
    Ok(elements)
}

fn convertStatement(mut stmt: &metamodelica::Ref<Statement::NFStatement>) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut elem: metamodelica::Ref<DAE::Statement>;
    elem = (match &**stmt {
        Statement::ASSIGNMENT { .. } => convertAssignment(stmt)?,
        Statement::FUNCTION_ARRAY_INIT {
            name: __stmt_name,
            source: __stmt_source,
            ty: __stmt_ty,
        } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            ty = Type::toDAE(metamodelica::AsArg::as_arg(&__stmt_ty), true)?;
            metamodelica::Ref::new(DAE::Statement::STMT_ARRAY_INIT {
                name: __stmt_name.clone(),
                ty: ty,
                source: __stmt_source.clone(),
            })
        }
        Statement::FOR { .. } => convertForStatement(stmt)?,
        Statement::IF {
            branches: __stmt_branches,
            source: __stmt_source,
        } => convertIfStatement(__stmt_branches.clone(), __stmt_source.clone())?,
        Statement::WHEN {
            branches: __stmt_branches,
            source: __stmt_source,
        } => convertWhenStatement(__stmt_branches.clone(), __stmt_source.clone())?,
        Statement::ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut e3: metamodelica::Ref<DAE::Exp>;
            e1 = Expression::toDAE(__stmt_condition.clone(), false)?;
            e2 = Expression::toDAE(__stmt_message.clone(), false)?;
            e3 = Expression::toDAE(__stmt_level.clone(), false)?;
            metamodelica::Ref::new(DAE::Statement::STMT_ASSERT {
                cond: e1,
                msg: e2,
                level: e3,
                source: __stmt_source.clone(),
            })
        }
        Statement::TERMINATE {
            message: __stmt_message,
            source: __stmt_source,
        } => metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE {
            msg: Expression::toDAE(__stmt_message.clone(), false)?,
            source: __stmt_source.clone(),
        }),
        Statement::REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            e1 = Expression::toDAE(__stmt_cref.clone(), false)?;
            e2 = Expression::toDAE(__stmt_reinitExp.clone(), false)?;
            metamodelica::Ref::new(DAE::Statement::STMT_REINIT {
                var: e1,
                value: e2,
                source: __stmt_source.clone(),
            })
        }
        Statement::NORETCALL {
            exp: __stmt_exp,
            source: __stmt_source,
        } => metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL {
            exp: Expression::toDAE(__stmt_exp.clone(), false)?,
            source: __stmt_source.clone(),
        }),
        Statement::WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut body: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            e1 = Expression::toDAE(__stmt_condition.clone(), false)?;
            body = convertStatements(__stmt_body.clone())?;
            metamodelica::Ref::new(DAE::Statement::STMT_WHILE {
                exp: e1,
                statementLst: body,
                source: __stmt_source.clone(),
            })
        }
        Statement::RETURN { source: __stmt_source } => metamodelica::Ref::new(DAE::Statement::STMT_RETURN {
            source: __stmt_source.clone(),
        }),
        Statement::BREAK { source: __stmt_source } => metamodelica::Ref::new(DAE::Statement::STMT_BREAK {
            source: __stmt_source.clone(),
        }),
        Statement::FAILURE {
            body: __stmt_body,
            source: __stmt_source,
        } => metamodelica::Ref::new(DAE::Statement::STMT_FAILURE {
            body: convertStatements(__stmt_body.clone())?,
            source: __stmt_source.clone(),
        }),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFConvertDAE.convertStatement"));
                    __mm_s.push_str(&*literal!(" got unknown statement "));
                    __mm_s.push_str(&*Statement::toString(stmt, literal!(""))?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(elem)
}

fn convertAssignment(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut daeStmt: metamodelica::Ref<DAE::Statement>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut dty: metamodelica::Ref<DAE::Type>;
    let mut dlhs: metamodelica::Ref<DAE::Exp>;
    let mut drhs: metamodelica::Ref<DAE::Exp>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*stmt)) {
        Deref @ Statement::ASSIGNMENT { lhs: __pa0, rhs: __pa1, ty: __pa2, source: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    lhs = metamodelica::Own::own(__pa0);
    rhs = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    src = metamodelica::Own::own(__pa3);
    if Type::isTuple(&ty) {
        let __pa4 = ::match_deref::match_deref! { match &(lhs.clone()) {
            Deref @ Expression::TUPLE { elements: __pa4, .. } => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        expl = metamodelica::Own::own(__pa4);
        daeStmt = (::match_deref::match_deref! { match &(expl.clone()) {
            Deref @ metamodelica::ListNode::Nil => metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: Expression::toDAE(rhs, false)?, source: src }),
            Deref @ metamodelica::ListNode::Cons { head: __esc_lhs, tail: Deref @ metamodelica::ListNode::Nil } => {
                lhs = (*__esc_lhs).clone();
                dty = Type::toDAE(&ty, true)?;
                dlhs = Expression::toDAE(lhs.clone(), false)?;
                drhs = metamodelica::Ref::new(DAE::Exp::TSUB { exp: Expression::toDAE(rhs, false)?, ix: 1, ty: dty.clone() });
                if Type::isArray(&ty) {
                    daeStmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: dty, lhs: dlhs, exp: drhs, source: src });
                } else {
                    daeStmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: dty, exp1: dlhs, exp: drhs, source: src });
                }
                daeStmt
            },
            _ => {
                dty = Type::toDAE(&ty, true)?;
                drhs = Expression::toDAE(rhs, false)?;
                metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: dty, expExpLst: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut e in (expl).into_iter().cloned() {
                let __x = Expression::toDAE(e.clone(), false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), exp: drhs, source: src })
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    } else {
        dty = Type::toDAE(&ty, true)?;
        dlhs = Expression::toDAE(lhs, false)?;
        drhs = Expression::toDAE(rhs, false)?;
        if Type::isArray(&ty) {
            daeStmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR {
                type_: dty,
                lhs: dlhs,
                exp: drhs,
                source: src,
            });
        } else {
            daeStmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
                type_: dty,
                exp1: dlhs,
                exp: drhs,
                source: src,
            });
        }
    }
    Ok(daeStmt)
}

fn convertForStatement(
    mut forStmt: &metamodelica::Ref<Statement::NFStatement>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut forDAE: metamodelica::Ref<DAE::Statement>;
    let mut iterator: metamodelica::Ref<InstNode::InstNode>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut dbody: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut for_type: Statement::ForType;
    let mut loop_vars: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>;
    let mut sub_iters_dae: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    )>;
    let mut sub_iters: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
    )>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*forStmt)) {
        Deref @ Statement::FOR { iterator: __pa0, range: Some(__pa1), body: __pa2, forType: __pa3, source: __pa4, sub_iters: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iterator = metamodelica::Own::own(__pa0);
    range = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    for_type = metamodelica::Own::own(__pa3);
    source = metamodelica::Own::own(__pa4);
    sub_iters = metamodelica::Own::own(__pa5);
    dbody = convertStatements(body)?;
    ty = InstNode::getType(iterator.clone())?;
    sub_iters_dae = ({
        let mut __acc: metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        )> = metamodelica::nil();
        for mut si in (sub_iters).into_iter().cloned() {
            let __x = (
                ComponentRef::toDAE(&(Util::tuple21(si.clone())))?,
                metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                        for mut e in (Util::tuple22(si.clone())
                            .borrow()
                            .iter()
                            .cloned()
                            .collect::<metamodelica::List<_>>())
                        .into_iter()
                        .cloned()
                        {
                            let __x = Expression::toDAE(e.clone(), false)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                ),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    forDAE = (match for_type.clone() {
        Statement::ForType::NORMAL => metamodelica::Ref::new(DAE::Statement::STMT_FOR {
            type_: Type::toDAE(&ty, true)?,
            iterIsArray: Type::isArray(&ty),
            iter: InstNode::name(&iterator)?,
            range: Expression::toDAE(range, false)?,
            statementLst: dbody,
            source: source,
            sub_iters: sub_iters_dae,
        }),
        Statement::ForType::PARALLEL { .. } => {
            loop_vars = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)> =
                    metamodelica::nil();
                for mut v in (var_field!(for_type.vars, Statement::ForType::PARALLEL).clone())
                    .into_iter()
                    .cloned()
                {
                    let __x = convertForStatementParallelVar(&(v.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            metamodelica::Ref::new(DAE::Statement::STMT_PARFOR {
                type_: Type::toDAE(&ty, true)?,
                iterIsArray: Type::isArray(&ty),
                iter: InstNode::name(&iterator)?,
                range: Expression::toDAE(range, false)?,
                statementLst: dbody,
                loopPrlVars: loop_vars,
                source: source,
            })
        }
    });
    Ok(forDAE)
}

fn convertForStatementParallelVar(
    mut var: &(metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo),
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)> {
    let mut outVar: (metamodelica::Ref<DAE::ComponentRef>, SourceInfo);
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut dcref: metamodelica::Ref<DAE::ComponentRef>;
    let mut info: SourceInfo;
    (cref, info) = var.clone();
    dcref = ComponentRef::toDAE(&cref)?;
    outVar = (dcref, info);
    Ok(outVar)
}

fn convertIfStatement(
    mut ifBranches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut ifStatement: metamodelica::Ref<DAE::Statement>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut dcond: metamodelica::Ref<DAE::Exp>;
    let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut dstmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut first: bool = true;
    let mut single: bool = ((ifBranches).len() as i32) == 1;
    let mut else_stmt: metamodelica::Ref<DAE::Else> = openmodelica_frontend_types::DAE::Else::interned_NOELSE();
    for mut b in &*ifBranches.reverse() {
        (cond, stmts) = b.clone();
        dcond = Expression::toDAE(cond.clone(), false)?;
        dstmts = convertStatements(stmts)?;
        if first && !(single) && Expression::isTrue(&cond) {
            else_stmt = metamodelica::Ref::new(DAE::Else::ELSE { statementLst: dstmts });
        } else {
            else_stmt = metamodelica::Ref::new(DAE::Else::ELSEIF {
                exp: dcond,
                statementLst: dstmts,
                else_: else_stmt,
            });
        }
        first = false;
    }
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(else_stmt) {
        Deref @ DAE::Else::ELSEIF { exp: __pa0, statementLst: __pa1, else_: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    dcond = metamodelica::Own::own(__pa0);
    dstmts = metamodelica::Own::own(__pa1);
    else_stmt = metamodelica::Own::own(__pa2);
    ifStatement = metamodelica::Ref::new(DAE::Statement::STMT_IF {
        exp: dcond,
        statementLst: dstmts,
        else_: else_stmt,
        source: source,
    });
    Ok(ifStatement)
}

fn convertWhenStatement(
    mut whenBranches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut whenStatement: metamodelica::Ref<DAE::Statement>;
    let mut co: metamodelica::Ref<Expression::NFExpression>;
    let mut conditions: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut cond: metamodelica::Ref<DAE::Exp>;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut when_stmt: Option<metamodelica::Ref<DAE::Statement>> = None;
    for mut b in &*whenBranches.reverse() {
        co = Util::tuple21(b.clone());
        conditions = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut c in (UnorderedSet::toList(Expression::extractCrefs(co.clone())?))
                .into_iter()
                .cloned()
            {
                if !(Type::isBoolean(&(ComponentRef::getSubscriptedType(&(c.clone()), false)?))) {
                    continue;
                }
                let __x = c.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        cond = Expression::toDAE(co, false)?;
        stmts = convertStatements(Util::tuple22(b.clone()))?;
        when_stmt = Some(metamodelica::Ref::new(DAE::Statement::STMT_WHEN {
            exp: cond,
            conditions: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut c in (conditions).into_iter().cloned() {
                    let __x = ComponentRef::toDAE(&(c.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            initialCall: false,
            statementLst: stmts,
            elseWhen: when_stmt,
            source: source.clone(),
        }));
    }
    let __pa0 = ::match_deref::match_deref! { match &(when_stmt) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    whenStatement = metamodelica::Own::own(__pa0);
    Ok(whenStatement)
}

fn convertInitialAlgorithms(
    mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    for mut alg in &*algorithms.reverse() {
        elements = convertInitialAlgorithm(metamodelica::AsArg::as_arg(&alg), elements)?;
    }
    Ok(elements)
}

fn convertInitialAlgorithm(
    mut alg: &metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut dalg: metamodelica::Ref<DAE::Algorithm>;
    stmts = convertStatements(alg.statements.clone())?;
    dalg = metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts });
    elements = metamodelica::cons(
        metamodelica::Ref::new(DAE::Element::INITIALALGORITHM {
            algorithm_: dalg,
            source: alg.source.clone(),
        }),
        elements,
    );
    Ok(elements)
}

pub fn convertFunctionTree(
    mut funcs: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
) -> Result<metamodelica::Ref<AvlTreePathFunction::Tree>> {
    let mut dfuncs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    dfuncs = (match &**funcs {
        Flatten::FunctionTreeImpl::Tree::NODE {
            height: __funcs_height,
            key: __funcs_key,
            left: __funcs_left,
            right: __funcs_right,
            value: __funcs_value,
        } => {
            let mut left: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut right: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut r#fn: DAE::Function;
            r#fn = convertFunction(metamodelica::AsArg::as_arg(&__funcs_value))?;
            left = convertFunctionTree(metamodelica::AsArg::as_arg(&__funcs_left))?;
            right = convertFunctionTree(metamodelica::AsArg::as_arg(&__funcs_right))?;
            metamodelica::Ref::new(AvlTreePathFunction::Tree::NODE {
                key: __funcs_key.clone(),
                value: Some(r#fn),
                height: __funcs_height.clone(),
                left: left,
                right: right,
            })
        }
        Flatten::FunctionTreeImpl::Tree::LEAF {
            key: __funcs_key,
            value: __funcs_value,
        } => {
            let mut r#fn: DAE::Function;
            r#fn = convertFunction(metamodelica::AsArg::as_arg(&__funcs_value))?;
            metamodelica::Ref::new(AvlTreePathFunction::Tree::LEAF {
                key: __funcs_key.clone(),
                value: Some(r#fn),
            })
        }
        Flatten::FunctionTreeImpl::Tree::EMPTY => {
            openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY()
        }
    });
    Ok(dfuncs)
}

fn convertFunction(mut func: &metamodelica::Ref<Function::Function>) -> Result<DAE::Function> {
    let mut dfunc: DAE::Function;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut elems: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut def: DAE::FunctionDefinition;
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    cls = InstNode::getClass(Function::instance(func)?)?;
    dfunc = (::match_deref::match_deref! { match &(cls) {
        Deref @ Class::TYPED_DERIVED { restriction: Deref @ Restriction::FUNCTION, .. } if (Function::isPartialDerivative(func)) => {
            def = DAE::FunctionDefinition::FUNCTION_PARTIAL_DERIVATIVE { derivedFunction: Function::getDerivedFunctionName(func)?, derivedVars: Function::getDerivedInputNames(func)? };
            Function::toDAE(func, def)?
        },
        Deref @ Class::INSTANCED_CLASS { sections: __esc_sections, restriction: Deref @ Restriction::FUNCTION, .. } => {
            sections = (*__esc_sections).clone();
            elems = convertFunctionParams(&func.inputs, metamodelica::nil())?;
            elems = convertFunctionParams(&(({
        let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        for mut o in (func.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::fromHandle(&(o.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), elems)?;
            elems = convertFunctionParams(&func.locals, elems)?;
            def = (match &*sections.clone() {
        Sections::SECTIONS { algorithms: __sections_algorithms, .. } => {
            elems = convertAlgorithms(__sections_algorithms.clone(), elems)?;
            DAE::FunctionDefinition::FUNCTION_DEF { body: elems.reverse() }
        },
        Sections::EXTERNAL { .. } => convertExternalDecl(metamodelica::AsArg::as_arg(&sections), elems.reverse())?,
        _ => DAE::FunctionDefinition::FUNCTION_DEF { body: elems.reverse() },
    });
            Function::toDAE(func, def)?
        },
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::RECORD_CONSTRUCTOR, .. } => DAE::Function::RECORD_CONSTRUCTOR { path: Function::name(func), type_: Function::makeDAEType(func, false)?, source: DAE::emptyElementSource().clone() },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConvertDAE.convertFunction")); __mm_s.push_str(&*literal!(" got unknown function")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dfunc)
}

fn convertFunctionParams(
    mut params: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>> = elements;
    for mut p in &**params {
        elements = metamodelica::cons(convertFunctionParam(p.clone())?, elements);
    }
    Ok(elements)
}

fn convertFunctionParam(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut element: metamodelica::Ref<DAE::Element>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut info: SourceInfo;
    let mut var_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut binding: Option<metamodelica::Ref<DAE::Exp>>;
    let mut ty_attr: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>;
    comp = InstNode::component(&node)?;
    element = (match &*comp {
        Component::COMPONENT {
            ty: __esc_ty,
            info: __esc_info,
            attributes: __esc_attr,
            binding: __comp_binding,
            classInst: __comp_classInst,
            comment: __comp_comment,
            ..
        } => {
            ty = (*__esc_ty).clone();
            info = (*__esc_info).clone();
            attr = (*__esc_attr).clone();
            cref = ComponentRef::fromNode(
                node.clone(),
                ty.clone(),
                metamodelica::nil(),
                ComponentRef::Origin::CREF.clone(),
            )?;
            binding = Binding::toDAEExp(metamodelica::AsArg::as_arg(&__comp_binding))?;
            cls = InstNode::getClass(__comp_classInst.clone())?;
            ty_attr = ({
                let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> =
                    metamodelica::nil();
                for mut m in (Class::getTypeAttributes(cls)).into_iter().cloned() {
                    let __x = (Modifier::name(&(m.clone()))?, Modifier::binding(&(m.clone())));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            var_attr = convertVarAttributes(
                &ty_attr,
                metamodelica::AsArg::as_arg(&ty),
                metamodelica::AsArg::as_arg(&attr),
            )?;
            makeDAEVar(
                &cref,
                ty.clone(),
                binding,
                metamodelica::AsArg::as_arg(&attr),
                InstNode::visibility(&node),
                var_attr,
                __comp_comment.clone(),
                FUNCTION_VARIABLE_CONVERSION_SETTINGS.clone(),
                info.clone(),
                false,
            )?
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFConvertDAE.convertFunctionParam"));
                    __mm_s.push_str(&*literal!(" got invalid component."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFConvertDAE.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(element)
}

fn convertExternalDecl(
    mut extDecl: &metamodelica::Ref<Sections::NFSections>,
    mut parameters: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<DAE::FunctionDefinition> {
    let mut funcDef: DAE::FunctionDefinition;
    let mut decl: DAE::ExternalDecl;
    let mut args: metamodelica::List<DAE::ExtArg>;
    let mut ret_arg: DAE::ExtArg;
    funcDef = (match &**extDecl {
        Sections::EXTERNAL {
            ann: __extDecl_ann,
            args: __extDecl_args,
            language: __extDecl_language,
            name: __extDecl_name,
            outputRef: __extDecl_outputRef,
            ..
        } => {
            args = ({
                let mut __acc: metamodelica::List<DAE::ExtArg> = metamodelica::nil();
                for mut e in (__extDecl_args.clone()).into_iter().cloned() {
                    let __x = convertExternalDeclArg(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            ret_arg = convertExternalDeclOutput(metamodelica::AsArg::as_arg(&__extDecl_outputRef))?;
            decl = DAE::ExternalDecl {
                name: __extDecl_name.clone(),
                args: args,
                returnArg: ret_arg,
                language: __extDecl_language.clone(),
                ann: __extDecl_ann.clone(),
            };
            DAE::FunctionDefinition::FUNCTION_EXT {
                body: parameters,
                externalDecl: decl,
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(funcDef)
}

fn convertExternalDeclArg(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<DAE::ExtArg> {
    let mut arg: DAE::ExtArg;
    arg = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: cref @ Deref @ ComponentRef::CREF { .. }, ty: __exp_ty } => {
            let mut dir: Absyn::Direction;
            dir = Prefixes::directionToAbsyn(Component::direction(&(InstNode::component(&(ComponentRef::node(metamodelica::AsArg::as_arg(&cref))?))?)));
            DAE::ExtArg::EXTARG { componentRef: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&cref))?, direction: dir, type_: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)? }
        },
        Deref @ Expression::SIZE { exp: Deref @ Expression::CREF { cref: cref @ Deref @ ComponentRef::CREF { .. }, .. }, dimIndex: Some(e) } => {
            DAE::ExtArg::EXTARGSIZE { componentRef: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&cref))?, type_: Type::toDAE(var_field!((**cref).ty, ComponentRef::NFComponentRef::CREF), true)?, exp: Expression::toDAE(e.clone(), false)? }
        },
        _ => {
            DAE::ExtArg::EXTARGEXP { exp: Expression::toDAE(exp.clone(), false)?, type_: Type::toDAE(&(Expression::typeOf(exp)), true)? }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

fn convertExternalDeclOutput(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<DAE::ExtArg> {
    let mut arg: DAE::ExtArg;
    arg = (match &**cref {
        ComponentRef::CREF { ty: __cref_ty, .. } => {
            let mut dir: Absyn::Direction;
            dir = Prefixes::directionToAbsyn(Component::direction(
                &(InstNode::component(&(ComponentRef::node(cref)?))?),
            ));
            DAE::ExtArg::EXTARG {
                componentRef: ComponentRef::toDAE(cref)?,
                direction: dir,
                type_: Type::toDAE(metamodelica::AsArg::as_arg(&__cref_ty), true)?,
            }
        }
        _ => openmodelica_frontend_types::DAE::ExtArg::NOEXTARG,
    });
    Ok(arg)
}

pub(crate) fn makeTypeVars(
    mut complexCls: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Var>>> {
    let mut typeVars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    typeVars = {
        let mut cls = InstNode::getClass(complexCls)?;
        (::match_deref::match_deref! { match &(cls) {
            Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::RECORD { .. }, .. } => ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
            for mut c in (ClassTree::getComponents(&(var_field!((*cls).elements, Class::NFClass::INSTANCED_CLASS).clone()))?).borrow().iter() {
                let __x = makeTypeRecordVar(&(c.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
            Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::RECORD_CONSTRUCTOR, .. } => ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
            for mut c in (ClassTree::getComponents(&(var_field!((*cls).elements, Class::NFClass::INSTANCED_CLASS).clone()))?).borrow().iter() {
                if !(!(InstNode::isOutput(&(c.clone())))) { continue; }
                let __x = makeTypeRecordVar(&(c.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
            Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { .. }, .. } => ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
            for mut c in (ClassTree::getComponents(&(var_field!((*cls).elements, Class::NFClass::INSTANCED_CLASS).clone()))?).borrow().iter() {
                if !(!(InstNode::isOnlyOuter(&(c.clone()))?)) { continue; }
                let __x = makeTypeVar(c.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
            _ => metamodelica::nil(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    };
    Ok(typeVars)
}

pub(crate) fn makeTypeVar(mut component: metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut typeVar: metamodelica::Ref<DAE::Var>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    comp = InstNode::component(&(InstNode::resolveOuter(component.clone())))?;
    attr = Component::getAttributes(&comp);
    typeVar = metamodelica::Ref::new(DAE::Var {
        name: InstNode::name(&component)?,
        attributes: Attributes::toDAE(&attr, InstNode::visibility(&component))?,
        ty: Type::toDAE(&(Component::getType(&comp)?), true)?,
        binding: Binding::toDAE(&(Component::getBinding(&comp)))?,
        bind_from_outside: false,
        constOfForIteratorRange: None,
    });
    Ok(typeVar)
}

pub(crate) fn makeTypeRecordVar(
    mut component: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut typeVar: metamodelica::Ref<DAE::Var>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    let mut vis: Visibility;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut bind_from_outside: bool;
    let mut ty: metamodelica::Ref<Type::NFType>;
    comp = InstNode::component(component)?;
    attr = Component::getAttributes(&comp);
    if Component::isFinal(&comp)? {
        vis = Visibility::PROTECTED.clone();
    } else {
        vis = InstNode::visibility(component);
    }
    binding = Component::getBinding(&comp);
    binding = Binding::mapExp(
        binding,
        (std::sync::Arc::new(stripScopePrefixExp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    binding = Flatten::flattenBinding(binding, &(Flatten::EMPTY_PREFIX().clone()), false)?;
    bind_from_outside = Binding::source(&binding) == Binding::Source::MODIFIER.clone();
    ty = Component::getType(&comp)?;
    ty = Type::mapDims(ty, &stripScopePrefixFromDim)?;
    typeVar = metamodelica::Ref::new(DAE::Var {
        name: InstNode::name(component)?,
        attributes: Attributes::toDAE(&attr, vis)?,
        ty: Type::toDAE(&ty, true)?,
        binding: Binding::toDAE(&binding)?,
        bind_from_outside: bind_from_outside,
        constOfForIteratorRange: None,
    });
    Ok(typeVar)
}

fn stripScopePrefixFromDim(
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension> = dim;
    dim = Dimension::mapExp(
        dim,
        (std::sync::Arc::new(fnptr!(
            stripScopePrefixCrefExp,
            metamodelica::Ref<Expression::NFExpression>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(dim)
}

fn stripScopePrefixExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new(fnptr!(
            stripScopePrefixCrefExp,
            metamodelica::Ref<Expression::NFExpression>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

fn stripScopePrefixCrefExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = stripScopePrefixCref(__exp_cref.clone()));
            ()
        }
        _ => (),
    });
    exp
}

fn stripScopePrefixCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> metamodelica::Ref<ComponentRef::NFComponentRef> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    if ComponentRef::isSimple(&cref) {
        return cref;
    }
    let () = (match &*cref {
        ComponentRef::CREF { .. } => {
            if ComponentRef::isFromCref(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF)) {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = stripScopePrefixCref(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone()));
            } else {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = crate::NFComponentRef::interned_EMPTY());
            }
            ()
        }
        _ => (),
    });
    cref
}
