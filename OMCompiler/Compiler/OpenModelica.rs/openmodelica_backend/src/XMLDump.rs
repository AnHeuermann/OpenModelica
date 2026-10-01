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
// =============================================================================
// For any request about the implementation of this package,
// please contact Filippo Donida (donida@elet.polimi.it).
// =============================================================================
// =============================================================================
// Important discrete states are not recognised as states.
// The varKind shoud be varVariability and another method
// and also the relative structure for the variable should
// be implemented to output the information like: state,
// dummy der, dummy state,...
// =============================================================================
// =============================================================================
// With a delaration like:
// parameter Real a = 1;
// record is everytime empty.  Why?
// =============================================================================
// =============================================================================
// In order to compile the XMLDump module (XMLDump.mo package)
// XMLDump.mo has been added to the MetaModelica source list in
// Compiler/.cmake/meta_modelica_source_list.cmake.
// =============================================================================
// =============================================================================
// Probably it's better to put a link to the corresponging
// algorithm/variable/when/zeroCross/...
// One solution could be to add an attribute like: Algorithm_Number
// to the algorith tab, like:
// <ALGORITHM LABEL=algorithm_Number>
// and then when dumping the algorithm reference in this function put
// the corresponding tag:
// <ANCHOR id=algorithm_Number/>
// within the equation element.
// =============================================================================
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

use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_types::ZeroCrossings;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// for stringReplace
pub(crate) const HEADER: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>";

pub(crate) const DAE_OPEN: &'static str = "dae xmlns:p1=\"http://www.w3.org/1998/Math/MathML\"\n                                                xmlns:xlink=\"http://www.w3.org/1999/xlink\"\n                                                xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n                                                xsi:noNamespaceSchemaLocation=\"http://home.dei.polimi.it/donida/Projects/AutoEdit/Images/DAE.xsd\"";

pub(crate) const DAE_CLOSE: &'static str = "dae";

pub(crate) const LABEL: &'static str = "label";

pub(crate) const ANCHOR: &'static str = "anchor";

pub(crate) const ALGORITHM_NAME: &'static str = "algorithmName";

/*
This String is used in:
  1 - dunmAbsynPathList - function to print a list of paths:
      <ELEMENT>
        Content
      </ELEMENT>
      <ELEMENT>
        ...
  2 - dumpCrefIdxLst to print a list of BackendDAE.CrefIndex:
      <ELEMENT ID=...>CrefIndex</ELEMENT>
      ...
  3 - dumpStrLst to print a list of String
      <ELEMENT>FirstStringOfList</ELEMENT>
      ...
      <ELEMENT>LastStringOfList</ELEMENT>
*/
pub(crate) const ELEMENT: &'static str = "element";

pub(crate) const ELEMENT_: &'static str = "Element";

pub(crate) const INDEX: &'static str = "index";

pub(crate) const INTERVAL: &'static str = "interval";

pub(crate) const START: &'static str = "start";

pub(crate) const VALUE: &'static str = "value";

pub(crate) const LIST_: &'static str = "List";

//Is the Dimension attribute of a list element.
pub(crate) const DIMENSION: &'static str = "dimension";

//Is the reference attribute for an element.
pub(crate) const ID: &'static str = "id";

pub(crate) const ID_: &'static str = "Id";

pub(crate) const CONDITION: &'static str = "Condition";

pub(crate) const REINIT: &'static str = "reinit";

pub(crate) const ASSERT: &'static str = "assert";

pub(crate) const TERMINATE: &'static str = "terminate";

//This is the String attribute for the textual representation of the expressions.
pub(crate) const EXP_STRING: &'static str = "string";

//This constant is used when is necessary to bind equations, variables, whenequations,..
pub(crate) const INVOLVED: &'static str = "involved";

pub(crate) const ADDITIONAL_INFO: &'static str = "additionalInfo";

pub(crate) const SOLVING_INFO: &'static str = "solvingInfo";

//This is the name that identifies the Variables' block. It's also used to compose the other
//Variables' names, such as KnownVariables, OrderedVariables, and so on.
pub(crate) const VARIABLES: &'static str = "variables";

pub(crate) const VARIABLES_: &'static str = "Variables";

pub(crate) const ORDERED: &'static str = "ordered";

pub(crate) const KNOWN: &'static str = "known";

pub(crate) const EXTERNAL: &'static str = "external";

pub(crate) const ALIAS: &'static str = "alias";

pub(crate) const CLASSES: &'static str = "classes";

pub(crate) const CLASSES_: &'static str = "Classes";

pub(crate) const CLASS: &'static str = "class";

pub(crate) const CLASS_: &'static str = "Class";

pub(crate) const NAMES_: &'static str = "Names";

//This is used all the time a variable is referenced.
pub(crate) const VARIABLE: &'static str = "variable";

pub(crate) const VAR_ID: &'static str = ID;

pub(crate) const VAR_NAME: &'static str = "name";

pub(crate) const VAR_INDEX: &'static str = "differentiatedIndex";

pub(crate) const VAR_DERNAME: &'static str = "derivativeName";

pub(crate) const VAR_ORIGNAME: &'static str = "origName";

pub(crate) const STATE_SELECT_NEVER: &'static str = "Never";

pub(crate) const STATE_SELECT_AVOID: &'static str = "Avoid";

pub(crate) const STATE_SELECT_DEFAULT: &'static str = "Default";

pub(crate) const STATE_SELECT_PREFER: &'static str = "Prefer";

pub(crate) const STATE_SELECT_ALWAYS: &'static str = "Always";

pub(crate) const VAR_FLOW: &'static str = "flow";

pub(crate) const VAR_FLOW_FLOW: &'static str = "Flow";

pub(crate) const VAR_FLOW_NONFLOW: &'static str = "NonFlow";

pub(crate) const VAR_FLOW_NONCONNECTOR: &'static str = "NonConnector";

pub(crate) const VAR_STREAM: &'static str = "stream";

pub(crate) const VAR_STREAM_STREAM: &'static str = "Stream";

pub(crate) const VAR_STREAM_NONSTREAM: &'static str = "NonStream";

pub(crate) const VAR_STREAM_NONSTREAM_CONNECTOR: &'static str = "NonStreamConnector";

// /  TO CORRECT WITHIN THE OMC!!!  ///
// The variability is related to the
// possible values a variable can assume
// In this case also information for the
// variable are stored. For example it would be useful
// to print the information about state, dummyState, dummyDer separately.
//In addition to this there's a problem with the discrete states,
//since they aren't recognised as states.
pub(crate) const VAR_VARIABILITY: &'static str = "variability";

pub(crate) const VARIABILITY_CONTINUOUS: &'static str = "continuous";

pub(crate) const VARIABILITY_CONTINUOUS_STATE: &'static str = "continuousState";

pub(crate) const VARIABILITY_CONTINUOUS_DUMMYDER: &'static str = "continuousDummyDer";

pub(crate) const VARIABILITY_CONTINUOUS_DUMMYSTATE: &'static str = "continuousDummyState";

pub(crate) const VARIABILITY_DISCRETE: &'static str = "discrete";

pub(crate) const VARIABILITY_PARAMETER: &'static str = "parameter";

pub(crate) const VARIABILITY_CONSTANT: &'static str = "constant";

pub(crate) const VARIABILITY_EXTERNALOBJECT: &'static str = "externalObject";

pub(crate) const VAR_TYPE: &'static str = "type";

pub(crate) const VARTYPE_INTEGER: &'static str = "Integer";

pub(crate) const VARTYPE_REAL: &'static str = "Real";

pub(crate) const VARTYPE_STRING: &'static str = "String";

pub(crate) const VARTYPE_BOOLEAN: &'static str = "Boolean";

pub(crate) const VARTYPE_ENUM: &'static str = "Enum";

pub(crate) const VARTYPE_ENUMERATION: &'static str = "enumeration";

pub(crate) const VARTYPE_EXTERNALOBJECT: &'static str = "ExternalObject";

pub(crate) const VAR_DIRECTION: &'static str = "direction";

pub(crate) const VARDIR_INPUT: &'static str = "input";

pub(crate) const VARDIR_OUTPUT: &'static str = "output";

pub(crate) const VARDIR_NONE: &'static str = "none";

pub(crate) const VAR_FIXED: &'static str = "fixed";

pub(crate) const VAR_COMMENT: &'static str = "comment";

pub(crate) const VAR_ATTRIBUTES_VALUES: &'static str = "attributesValues";

pub(crate) const VAR_ATTR_QUANTITY: &'static str = "quantity";

pub(crate) const VAR_ATTR_UNIT: &'static str = "unit";

pub(crate) const VAR_ATTR_DISPLAY_UNIT: &'static str = "displayUnit";

pub(crate) const VAR_ATTR_STATESELECT: &'static str = "stateSelect";

pub(crate) const VAR_ATTR_MINVALUE: &'static str = "minValue";

pub(crate) const VAR_ATTR_MAXVALUE: &'static str = "maxValue";

pub(crate) const VAR_ATTR_NOMINAL: &'static str = "nominal";

pub(crate) const VAR_ATTR_INITIALVALUE: &'static str = "initialValue";

pub(crate) const VAR_ATTR_FIXED: &'static str = "fixed";

//Name of the element containing the binding information
//for the variables (bindExpression)
//For example consider:
//parameter Real a = 3*2+e; //With Real constant e = 3;
//BindExpression 3*2+e
pub(crate) const BIND_EXPRESSION: &'static str = "bindExpression";

//Name of the element representing the subscript, for example the array's index.
pub(crate) const SUBSCRIPT: &'static str = "subscript";

//Additional info for variables.
pub(crate) const HASH_TB_CREFS_LIST: &'static str = "hashTb";

pub(crate) const HASH_TB_STRING_LIST_OLDVARS: &'static str = "hashTbOldVars";

//All this constants below are used in the dumpBackendDAE method.
pub(crate) const EQUATIONS: &'static str = "equations";

pub(crate) const EQUATIONS_: &'static str = "Equations";

pub(crate) const SIMPLE: &'static str = "simple";

pub(crate) const INITIAL: &'static str = "initial";

pub(crate) const ZERO_CROSSING: &'static str = "zeroCrossing";

pub(crate) const SAMPLES: &'static str = "Samples";

pub(crate) const ARRAY_OF_EQUATIONS: &'static str = "arrayOfEquations";

//This is used also in the dumpEquation method.
pub(crate) const COMPLEX_EQUATION: &'static str = "complexequations";

pub(crate) const EQUATION: &'static str = "equation";

pub(crate) const EQUATION_: &'static str = "Equation";

pub(crate) const SOLVED: &'static str = "solved";

pub(crate) const SOLVED_: &'static str = "Solved";

pub(crate) const WHEN: &'static str = "when";

pub(crate) const WHEN_: &'static str = "When";

pub(crate) const WHEN_OPERATORS: &'static str = "WhenOperators";

pub(crate) const WHEN_OPERATOR: &'static str = "WhenOperator";

pub(crate) const RESIDUAL: &'static str = "residual";

pub(crate) const RESIDUAL_: &'static str = "Residual";

/*
This String constant is used in:
  1 - dumpAlgorithms to print out the list of Algorithms:
      <ALGORITHM LABEL=Algorithm_ID>
        ...
      </ALGORITHM>
  2 - dumpEquation if the equation element is an algorithm:
      <ALGORITHM ID=...>
        <AlgorithmID>...</AlgorithmID>
        <ANCHOR ALGORITHM_NAME=Algorithm_No></ANCHOR>
      </ALGORITHM>
*/
pub(crate) const ALGORITHM: &'static str = "algorithm";

/*
This String constant is used to print the reference to the
corresponding algorithm.
*/
pub(crate) const ALGORITHM_REF: &'static str = "algorithm_ref";

pub(crate) const CONSTRAINT: &'static str = "constraint";

pub(crate) const CONSTRAINT_REF: &'static str = "constraint_ref";

/*
This String constant represents the single equation of an array of
equations and it is used in:
  1 - dumpArrayEqns to print the list of equations
  2 - dumpEquation to print the list of equations corresponding to
      the array
*/
pub(crate) const ARRAY_EQUATION: &'static str = "arrayEquation";

pub(crate) const ALGORITHMS: &'static str = "algorithms";

pub(crate) const CONSTRAINTS: &'static str = "constraints";

pub(crate) const FUNCTIONS: &'static str = "functions";

pub(crate) const FUNCTION: &'static str = "function";

pub(crate) const FUNCTION_NAME: &'static str = "name";

pub(crate) const FUNCTION_ORIGNAME: &'static str = VAR_ORIGNAME;

pub(crate) const NAME_BINDINGS: &'static str = "nameBindings";

pub(crate) const C_NAME: &'static str = "cName";

pub(crate) const C_IMPLEMENTATIONS: &'static str = "cImplementations";

pub(crate) const MODELICA_IMPLEMENTATION: &'static str = "ModelicaImplementation";

/*This strings here below are used for printing additionalInfo
concerning the DAE system of equations, such as:
 - the original adjacency matrix (before performing matching and BLT
 - the matching algorithm output
 - the blocks obtained after running the BLT algorithm (Tarjan)
 */
pub(crate) const MATCHING_ALGORITHM: &'static str = "matchingAlgorithm";

pub(crate) const SOLVED_IN: &'static str = "solvedIn";

pub(crate) const BLT_REPRESENTATION: &'static str = "bltRepresentation";

pub(crate) const BLT_BLOCK: &'static str = "bltBlock";

pub(crate) const ORIGINAL_ADJACENCY_MATRIX: &'static str = "originalAdjacencyMatrix";

pub(crate) const MATH: &'static str = "math";

pub(crate) const MathML: &'static str = "MathML";

pub(crate) const MathMLApply: &'static str = "apply";

pub(crate) const MathMLWeb: &'static str = "http://www.w3.org/1998/Math/MathML";

pub(crate) const MathMLXmlns: &'static str = "xmlns";

pub(crate) const MathMLType: &'static str = "type";

pub(crate) const MathMLNumber: &'static str = "cn";

pub(crate) const MathMLVariable: &'static str = "ci";

pub(crate) const MathMLConstant: &'static str = "constant";

pub(crate) const MathMLInteger: &'static str = "integer";

pub(crate) const MathMLReal: &'static str = "real";

pub(crate) const MathMLVector: &'static str = "vector";

pub(crate) const MathMLMatrixrow: &'static str = "matrixrow";

pub(crate) const MathMLMatrix: &'static str = "matrix";

pub(crate) const MathMLTrue: &'static str = "true";

pub(crate) const MathMLFalse: &'static str = "false";

pub(crate) const MathMLAnd: &'static str = "and";

pub(crate) const MathMLOr: &'static str = "or";

pub(crate) const MathMLNot: &'static str = "not";

pub(crate) const MathMLEqual: &'static str = "eq";

pub(crate) const MathMLLessThan: &'static str = "lt";

pub(crate) const MathMLLessEqualThan: &'static str = "leq";

pub(crate) const MathMLGreaterThan: &'static str = "gt";

pub(crate) const MathMLGreaterEqualThan: &'static str = "geq";

pub(crate) const MathMLEquivalent: &'static str = "equivalent";

pub(crate) const MathMLNotEqual: &'static str = "neq";

pub(crate) const MathMLPlus: &'static str = "plus";

pub(crate) const MathMLMinus: &'static str = "minus";

pub(crate) const MathMLTimes: &'static str = "times";

pub(crate) const MathMLDivide: &'static str = "divide";

pub(crate) const MathMLPower: &'static str = "power";

pub(crate) const MathMLTranspose: &'static str = "transpose";

pub(crate) const MathMLScalarproduct: &'static str = "scalarproduct";

pub(crate) const MathMLVectorproduct: &'static str = "vectorproduct";

pub(crate) const MathMLInterval: &'static str = "interval";

pub(crate) const MathMLSelector: &'static str = "selector";

pub(crate) const MathMLIfClause: &'static str = "piecewise";

pub(crate) const MathMLIfBranch: &'static str = "piece";

pub(crate) const MathMLElseBranch: &'static str = "otherwise";

pub(crate) const MathMLOperator: &'static str = "mo";

pub(crate) const MathMLArccos: &'static str = "arccos";

pub(crate) const MathMLArcsin: &'static str = "arcsin";

pub(crate) const MathMLArctan: &'static str = "arctan";

pub(crate) const MathMLLn: &'static str = "ln";

pub(crate) const MathMLLog: &'static str = "log";

fn binopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator {
        mut op => {
            let mut s: ArcStr;
            s = binopSymbol2(&op)?;
            s
        }
    });
    Ok(outString)
}

fn binopSymbol2(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::ADD { .. } => {
            arcstr::literal!(MathMLPlus)
        }
        DAE::Operator::SUB { .. } => {
            arcstr::literal!(MathMLMinus)
        }
        DAE::Operator::MUL { .. } => {
            arcstr::literal!(MathMLTimes)
        }
        DAE::Operator::DIV { .. } => {
            arcstr::literal!(MathMLDivide)
        }
        DAE::Operator::POW { .. } => {
            arcstr::literal!(MathMLPower)
        }
        DAE::Operator::ADD_ARR { .. } => {
            arcstr::literal!(MathMLPlus)
        }
        DAE::Operator::SUB_ARR { .. } => {
            arcstr::literal!(MathMLMinus)
        }
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => {
            arcstr::literal!(MathMLTimes)
        }
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => {
            arcstr::literal!(MathMLScalarproduct)
        }
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => {
            arcstr::literal!(MathMLVectorproduct)
        }
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => {
            arcstr::literal!(MathMLDivide)
        }
        _ => {
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.binopSymbol2 - Unknown operator: ");
            error_msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*error_msg);
                __mm_s.push_str(&*ExpressionDump::debugBinopSymbol(inOperator)?);
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn dumpAbsynPathLst(
    mut absynPathLst: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut Content: ArcStr,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match absynPathLst {
        Deref @ metamodelica::ListNode::Nil => (),
        _ => {
            dumpStrOpenTag(Content.clone())?;
            dumpAbsynPathLst2(absynPathLst)?;
            dumpStrCloseTag(Content)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpAbsynPathLst2(mut absynPathLst: &metamodelica::List<metamodelica::Ref<Absyn::Path>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match absynPathLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: ap, tail: apLst } => {
            let mut r#str: ArcStr;
            r#str = AbsynUtil::pathStringNoQual(ap.clone(), literal!("."), false, false)?;
            dumpStrTagContent(arcstr::literal!(ELEMENT), r#str)?;
            dumpAbsynPathLst2(apLst)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpConstraints(mut constrs: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match constrs {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => {
            let mut len: i32;
            len = ((constrs).len() as i32);
            dumpStrOpenTagAttr(arcstr::literal!(CONSTRAINTS), arcstr::literal!(DIMENSION), intString(len))?;
            dumpConstraints2(constrs, 0)?;
            dumpStrCloseTag(arcstr::literal!(CONSTRAINTS))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpConstraints2(
    mut iConstrs: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut inConsNo: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match iConstrs {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps }, tail: constrs } => {
            let mut conNo = inConsNo;
            let mut conNo_1: i32;
            dumpStrOpenTagAttr(arcstr::literal!(CONSTRAINT), arcstr::literal!(LABEL), stringAppend(stringAppend(arcstr::literal!(CONSTRAINT_REF), literal!("_")), intString(conNo)))?;
            Print::printBuf(Util::xmlEscape(DAEDump::dumpConstraintsStr(&(list![metamodelica::Ref::new(DAE::Element::CONSTRAINT { constraints: metamodelica::Ref::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps.clone() }), source: DAE::emptyElementSource().clone() })]))?)?)?;
            dumpStrCloseTag(arcstr::literal!(CONSTRAINT))?;
            conNo_1 = conNo + 1;
            dumpConstraints2(constrs, conNo_1)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn dumpBltInvolvedEquations(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut offset: i32,
) -> Result<()> {
    let () = (match &**inComp {
        BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, .. } => {
            dumpStrTagAttrNoChild(
                stringAppend(arcstr::literal!(INVOLVED), arcstr::literal!(EQUATION_)),
                stringAppend(arcstr::literal!(EQUATION), arcstr::literal!(ID_)),
                intString(e.clone() + offset),
            )?;
            ()
        }
        _ => {
            let mut elst: metamodelica::List<i32>;
            (elst, _) = BackendDAETransform::getEquationAndSolvedVarIndxes(inComp)?;
            dumpBltInvolvedEquations1(&elst, offset)?;
            ()
        }
    });
    Ok(())
}

fn dumpBltInvolvedEquations1(mut inList: &metamodelica::List<i32>, mut offset: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: el, tail: remList } => {
            dumpStrTagAttrNoChild(stringAppend(arcstr::literal!(INVOLVED), arcstr::literal!(EQUATION_)), stringAppend(arcstr::literal!(EQUATION), arcstr::literal!(ID_)), intString(el.clone() + offset))?;
            dumpBltInvolvedEquations1(remList, offset)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpBindExpression(mut inOptExpExp: Option<metamodelica::Ref<DAE::Exp>>, mut addMathMLCode: bool) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inOptExpExp.clone()) {
        None => (),
        Some(_) => {
            dumpOptExp(inOptExpExp, arcstr::literal!(BIND_EXPRESSION), addMathMLCode)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpComment(mut inComment: ArcStr) -> Result<()> {
    Print::printBuf(literal!("<!--"))?;
    Print::printBuf(Util::xmlEscape(inComment)?)?;
    Print::printBuf(literal!("-->"))?;
    Ok(())
}

fn dumpComponents(mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<()> {
    dumpStrOpenTag(arcstr::literal!(BLT_REPRESENTATION))?;
    BackendDAEUtil::foldEqSystem(
        dae,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: (i32, i32)| dumpComponentsWork(&__a0, &__a1, __a2),
        (0, 0),
    )?;
    dumpStrCloseTag(arcstr::literal!(BLT_REPRESENTATION))?;
    Ok(())
}

fn dumpComponentsWork(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inOffset: (i32, i32),
) -> Result<(i32, i32)> {
    let mut outOffset: (i32, i32);
    let mut v1: metamodelica::Array<i32>;
    let mut v2: metamodelica::Array<i32>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut voffset: i32;
    let mut eoffset: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*syst)) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    v1 = metamodelica::Own::own(__pa0);
    v2 = metamodelica::Own::own(__pa1);
    comps = metamodelica::Own::own(__pa2);
    (voffset, eoffset) = inOffset;
    dumpStrOpenTag(arcstr::literal!(BLT_REPRESENTATION))?;
    dumpComponents1(&comps, voffset, eoffset)?;
    dumpStrCloseTag(arcstr::literal!(BLT_REPRESENTATION))?;
    outOffset = (
        voffset + metamodelica::arrayLength(v2.clone()),
        eoffset + metamodelica::arrayLength(v1.clone()),
    );
    Ok(outOffset)
}

fn dumpComponents1(
    mut l: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut voffset: i32,
    mut eoffset: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match l {
        Deref @ metamodelica::ListNode::Nil => (),
        _ => {
            dumpComponents2(l, 1 + voffset, eoffset)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpComponents2(
    mut inIntegerLstLst: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut i: i32,
    mut offset: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inIntegerLstLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: l, tail: lst } => {
            dumpStrOpenTagAttr(arcstr::literal!(BLT_BLOCK), arcstr::literal!(ID), intString(i))?;
            dumpBltInvolvedEquations(metamodelica::AsArg::as_arg(&l), offset)?;
            dumpStrCloseTag(arcstr::literal!(BLT_BLOCK))?;
            dumpComponents2(lst, i + 1, offset)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpCrefIdxLstArr(
    mut crefIdxLstArr: metamodelica::Array<metamodelica::List<BackendDAE::CrefIndex>>,
    mut Content: ArcStr,
    mut inInteger: i32,
) -> Result<()> {
    let __ab_crefIdxLstArr = crefIdxLstArr.borrow();
    let () = 'mc: {
        let __mc_input = inInteger;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if !((*metamodelica::index_checked(&__ab_crefIdxLstArr, inInteger)?).is_empty()) {
                return Err("guard");
            }
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            dumpCrefIdxLst(
                &(*metamodelica::index_checked(&__ab_crefIdxLstArr, inInteger)?),
                Content.clone(),
            )?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.dumpCrefIdxLstArr - failed for var number:");
            error_msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*error_msg);
                __mm_s.push_str(&*intString(inInteger));
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn dumpCrefIdxLst(mut crefIdxLst: &metamodelica::List<BackendDAE::CrefIndex>, mut Content: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match crefIdxLst {
        Deref @ metamodelica::ListNode::Nil => (),
        _ => {
            dumpStrOpenTag(Content.clone())?;
            dumpCrefIdxLst2(crefIdxLst)?;
            dumpStrCloseTag(Content)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpCrefIdxLst2(mut crefIdxLst: &metamodelica::List<BackendDAE::CrefIndex>) -> Result<()> {
    let () = (::match_deref::match_deref! { match crefIdxLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: BackendDAE::CrefIndex { cref: cref_c, index: index_c }, tail: crefIndexList } => {
            let mut cref: ArcStr;
            cref = ComponentReference::crefStr(metamodelica::AsArg::as_arg(&cref_c))?;
            dumpStrOpenTagAttr(arcstr::literal!(ELEMENT), arcstr::literal!(ID), intString(index_c.clone()))?;
            Print::printBuf(cref)?;
            dumpStrCloseTag(arcstr::literal!(ELEMENT))?;
            dumpCrefIdxLst2(crefIndexList)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpDAEInstDims(
    mut arry_Dim: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut Content: ArcStr,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match arry_Dim {
        Deref @ metamodelica::ListNode::Nil => (),
        _ => {
            dumpStrOpenTag(Content.clone())?;
            dumpDAEInstDims2(arry_Dim)?;
            dumpStrCloseTag(Content)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpDAEInstDims2(mut arry_Dim: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match arry_Dim {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: dim, tail: lDim } => {
            dumpStrOpenTag(arcstr::literal!(DIMENSION))?;
            dumpDimension(metamodelica::AsArg::as_arg(&dim))?;
            dumpStrCloseTag(arcstr::literal!(DIMENSION))?;
            dumpDAEInstDims2(lDim)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpDAEXML(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut fileNamePrefix: ArcStr;
    let __arc2 = inDAE.clone();
    let BackendDAE::DAE { shared: __t1, .. } = &*__arc2;
    let __arc3 = __t1.clone();
    let BackendDAE::SHARED {
        info: BackendDAE::EXTRA_INFO {
            fileNamePrefix: __pa0, ..
        },
        ..
    } = &*__arc3;
    fileNamePrefix = metamodelica::Own::own(__pa0);
    Print::clearBuf();
    dumpBackendDAE(&inDAE, false, false, false, false, false)?;
    Print::writeBuf({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fileNamePrefix);
        __mm_s.push_str(&*literal!(".xml"));
        ArcStr::from(__mm_s)
    })?;
    Print::clearBuf();
    Ok(outDAE)
}

pub fn dumpBackendDAE(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut addOriginalAdjacencyMatrix: bool,
    mut addSolvingInfo: bool,
    mut addMathMLCode: bool,
    mut dumpResiduals: bool,
    mut dumpSolvedEquations: bool,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (
            &**inBackendDAE,
            addOriginalAdjacencyMatrix,
            addSolvingInfo,
            addMathMLCode,
            dumpResiduals,
            dumpSolvedEquations,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::BackendDAE { eqs: systs, shared: Deref @ BackendDAE::Shared { globalKnownVars: vars_knownVars @ BackendDAE::Variables { crefIndices: crefIdxLstArr_knownVars, .. }, localKnownVars: _, externalObjects: vars_externalObject @ BackendDAE::Variables { crefIndices: crefIdxLstArr_externalObject, .. }, aliasVars: vars_aliasVars @ BackendDAE::Variables { crefIndices: crefIdxLstArr_aliasVars, .. }, initialEqs: ieqns, removedEqs: _, constraints: constrs, classAttrs: _, cache: _, graph: _, functionTree: funcs, eventInfo, extObjClasses: extObjCls, backendDAEType: _, symjacs: _, info: _, .. } }, addOrInMatrix, addSolInfo, addMML, dumpRes, false) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut knvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut extvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut aliasvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut eqnsl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnsl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut ieqnsl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut functionsElems: metamodelica::List<DAE::Function>;
                    knvars = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars_knownVars))?;
                    extvars = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars_externalObject))?;
                    aliasvars = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars_aliasVars))?;
                    reqns = BackendDAEUtil::collapseRemovedEqs(inBackendDAE)?;
                    Print::printBuf(arcstr::literal!(HEADER))?;
                    dumpStrOpenTag(arcstr::literal!(DAE_OPEN))?;
                    dumpStrOpenTagAttr(arcstr::literal!(VARIABLES), arcstr::literal!(DIMENSION), intString(List::fold(&(List::map(systs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| BackendDAEUtil::systemSize(&__a0))?), &fnptr!(intAdd, i32, i32), 0)? + ((knvars).len() as i32) + ((extvars).len() as i32) + ((aliasvars).len() as i32)))?;
                    vars = List::fold(metamodelica::AsArg::as_arg(&systs), &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>| getOrderedVars(&__a0, __a1), metamodelica::nil())?;
                    dumpVars(&vars, arrayCreate(1, metamodelica::nil()), stringAppend(arcstr::literal!(ORDERED), arcstr::literal!(VARIABLES_)), addMML.clone())?;
                    dumpVars(&knvars, crefIdxLstArr_knownVars.clone(), stringAppend(arcstr::literal!(KNOWN), arcstr::literal!(VARIABLES_)), addMML.clone())?;
                    dumpVars(&extvars, crefIdxLstArr_externalObject.clone(), stringAppend(arcstr::literal!(EXTERNAL), arcstr::literal!(VARIABLES_)), addMML.clone())?;
                    dumpVars(&aliasvars, crefIdxLstArr_aliasVars.clone(), stringAppend(arcstr::literal!(ALIAS), arcstr::literal!(VARIABLES_)), addMML.clone())?;
                    dumpExtObjCls(extObjCls.clone(), &(stringAppend(arcstr::literal!(EXTERNAL), arcstr::literal!(CLASSES_))))?;
                    dumpStrCloseTag(arcstr::literal!(VARIABLES))?;
                    eqnsl = List::fold(metamodelica::AsArg::as_arg(&systs), &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>| getEqsList(&__a0, __a1), metamodelica::nil())?;
                    dumpEqns(&eqnsl, arcstr::literal!(EQUATIONS), addMML.clone(), dumpRes.clone(), false)?;
                    reqnsl = BackendEquation::equationList(reqns.clone())?;
                    dumpEqns(&reqnsl, stringAppend(arcstr::literal!(SIMPLE), arcstr::literal!(EQUATIONS_)), addMML.clone(), dumpRes.clone(), false)?;
                    ieqnsl = BackendEquation::equationList(ieqns.clone())?;
                    dumpEqns(&ieqnsl, stringAppend(arcstr::literal!(INITIAL), arcstr::literal!(EQUATIONS_)), addMML.clone(), dumpRes.clone(), false)?;
                    dumpEventInfo(metamodelica::AsArg::as_arg(&eventInfo), addMML.clone())?;
                    dumpConstraints(metamodelica::AsArg::as_arg(&constrs))?;
                    functionsElems = DAEUtil::getFunctionList(metamodelica::AsArg::as_arg(&funcs), false)?;
                    dumpFunctions(&functionsElems)?;
                    dumpSolvingInfo(addOrInMatrix.clone(), addSolInfo.clone(), inBackendDAE)?;
                    dumpStrCloseTag(arcstr::literal!(DAE_CLOSE))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::BackendDAE { eqs: systs, shared: Deref @ BackendDAE::Shared { globalKnownVars: vars_knownVars @ BackendDAE::Variables { crefIndices: crefIdxLstArr_knownVars, .. }, localKnownVars: _, externalObjects: vars_externalObject @ BackendDAE::Variables { crefIndices: crefIdxLstArr_externalObject, .. }, aliasVars: vars_aliasVars @ BackendDAE::Variables { crefIndices: crefIdxLstArr_aliasVars, .. }, initialEqs: ieqns, removedEqs: _, constraints: constrs, classAttrs: _, cache: _, graph: _, functionTree: funcs, eventInfo, extObjClasses: extObjCls, backendDAEType: _, symjacs: _, info: _, partitionsInfo: _, .. } }, addOrInMatrix, addSolInfo, addMML, dumpRes, true) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut knvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut extvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut aliasvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut reqnsl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut ieqnsl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut functionsElems: metamodelica::List<DAE::Function>;
                    let mut eqnsVarsinOrderLst: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                    knvars = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars_knownVars))?;
                    extvars = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars_externalObject))?;
                    aliasvars = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars_aliasVars))?;
                    reqns = BackendDAEUtil::collapseRemovedEqs(inBackendDAE)?;
                    Print::printBuf(arcstr::literal!(HEADER))?;
                    dumpStrOpenTag(arcstr::literal!(DAE_OPEN))?;
                    dumpStrOpenTagAttr(arcstr::literal!(VARIABLES), arcstr::literal!(DIMENSION), intString(List::fold(&(List::map(systs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| BackendDAEUtil::systemSize(&__a0))?), &fnptr!(intAdd, i32, i32), 0)? + ((knvars).len() as i32) + ((extvars).len() as i32) + ((aliasvars).len() as i32)))?;
                    vars = List::fold(metamodelica::AsArg::as_arg(&systs), &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>| getOrderedVars(&__a0, __a1), metamodelica::nil())?;
                    dumpVars(&vars, arrayCreate(1, metamodelica::nil()), stringAppend(arcstr::literal!(ORDERED), arcstr::literal!(VARIABLES_)), addMML.clone())?;
                    dumpVars(&knvars, crefIdxLstArr_knownVars.clone(), stringAppend(arcstr::literal!(KNOWN), arcstr::literal!(VARIABLES_)), addMML.clone())?;
                    dumpVars(&extvars, crefIdxLstArr_externalObject.clone(), stringAppend(arcstr::literal!(EXTERNAL), arcstr::literal!(VARIABLES_)), addMML.clone())?;
                    dumpVars(&aliasvars, crefIdxLstArr_aliasVars.clone(), stringAppend(arcstr::literal!(ALIAS), arcstr::literal!(VARIABLES_)), addMML.clone())?;
                    dumpExtObjCls(extObjCls.clone(), &(stringAppend(arcstr::literal!(EXTERNAL), arcstr::literal!(CLASSES_))))?;
                    dumpStrCloseTag(arcstr::literal!(VARIABLES))?;
                    eqnsVarsinOrderLst = List::fold(metamodelica::AsArg::as_arg(&systs), &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>| getOrderedEqsandVars(&__a0, __a1), metamodelica::nil())?;
                    dumpStrOpenTagAttr(arcstr::literal!(EQUATIONS), arcstr::literal!(DIMENSION), intString(((eqnsVarsinOrderLst).len() as i32)))?;
                    dumpSolvedEqns(&eqnsVarsinOrderLst, 1, &(arcstr::literal!(EQUATIONS)), addMML.clone(), dumpRes.clone(), true)?;
                    dumpStrCloseTag(arcstr::literal!(EQUATIONS))?;
                    reqnsl = BackendEquation::equationList(reqns.clone())?;
                    dumpEqns(&reqnsl, stringAppend(arcstr::literal!(SIMPLE), arcstr::literal!(EQUATIONS_)), addMML.clone(), dumpRes.clone(), false)?;
                    ieqnsl = BackendEquation::equationList(ieqns.clone())?;
                    dumpEqns(&ieqnsl, stringAppend(arcstr::literal!(INITIAL), arcstr::literal!(EQUATIONS_)), addMML.clone(), dumpRes.clone(), false)?;
                    dumpEventInfo(metamodelica::AsArg::as_arg(&eventInfo), addMML.clone())?;
                    dumpConstraints(metamodelica::AsArg::as_arg(&constrs))?;
                    functionsElems = DAEUtil::getFunctionList(metamodelica::AsArg::as_arg(&funcs), false)?;
                    dumpFunctions(&functionsElems)?;
                    dumpSolvingInfo(addOrInMatrix.clone(), addSolInfo.clone(), inBackendDAE)?;
                    dumpStrCloseTag(arcstr::literal!(DAE_CLOSE))?;
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
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("XMLDump.dumpBackendDAE failed")])?;
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

fn dumpEventInfo(mut inEventInfo: &BackendDAE::EventInfo, mut addMML: bool) -> Result<()> {
    let () = (match inEventInfo.clone() {
        BackendDAE::EventInfo {
            timeEvents: mut timeEvents,
            zeroCrossings: mut zc,
            ..
        } => {
            dumpTimeEvents(
                metamodelica::AsArg::as_arg(&timeEvents),
                stringAppend(arcstr::literal!(SAMPLES), arcstr::literal!(LIST_)),
                addMML,
            )?;
            dumpZeroCrossing(
                &(ZeroCrossings::toList(metamodelica::AsArg::as_arg(&zc))),
                stringAppend(arcstr::literal!(ZERO_CROSSING), arcstr::literal!(LIST_)),
                addMML,
            )?;
            ()
        }
    });
    Ok(())
}

fn getOrderedVars(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    vars = BackendVariable::varList(&(BackendVariable::daeVars(syst)))?;
    outVars = listAppend(inVars, vars);
    Ok(outVars)
}

fn getEqsList(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqnsl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    eqnsl = BackendEquation::equationList(BackendEquation::getEqnsFromEqSystem(syst))?;
    outEqns = listAppend(inEqns, eqnsl);
    Ok(outEqns)
}

fn getOrderedEqsandVars(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inEqnsVars: metamodelica::List<(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    )>,
> {
    let mut outEqnsVars: metamodelica::List<(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    )>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*syst)) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    eqns = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    comps = metamodelica::Own::own(__pa2);
    outEqnsVars = getOrderedEqs2(&comps, eqns, &vars, inEqnsVars)?;
    Ok(outEqnsVars)
}

fn getOrderedEqs2<'__b>(
    mut inComps: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut vars: &'__b BackendDAE::Variables,
    mut inAccum: metamodelica::List<(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    )>,
> {
    '__tco: loop {
        ::match_deref::match_deref! { match inComps {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inAccum)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, var: v }, tail: rest } => {
                let mut var: metamodelica::Ref<BackendDAE::Var>;
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut result: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                var = BackendVariable::getVarAt(vars, v.clone())?;
                eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                result = listAppend(inAccum, list![(list![eqn], list![var])]);
                { (inComps, eqns, vars, inAccum) = (rest, eqns, vars, result); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: elst, vars: vlst, .. }, tail: rest } => {
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut result: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                eqnlst = BackendEquation::getList(elst.clone(), eqns.clone())?;
                result = listAppend(inAccum, list![(eqnlst, varlst)]);
                { (inComps, eqns, vars, inAccum) = (rest, eqns, vars, result); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: e, vars: vlst }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut result: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                result = listAppend(inAccum, list![(list![eqn], varlst)]);
                { (inComps, eqns, vars, inAccum) = (rest, eqns, vars, result); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut result: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                result = listAppend(inAccum, list![(list![eqn], varlst)]);
                { (inComps, eqns, vars, inAccum) = (rest, eqns, vars, result); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: e, vars: vlst }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut result: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                result = listAppend(inAccum, list![(list![eqn], varlst)]);
                { (inComps, eqns, vars, inAccum) = (rest, eqns, vars, result); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut result: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                result = listAppend(inAccum, list![(list![eqn], varlst)]);
                { (inComps, eqns, vars, inAccum) = (rest, eqns, vars, result); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut result: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                result = listAppend(inAccum, list![(list![eqn], varlst)]);
                { (inComps, eqns, vars, inAccum) = (rest, eqns, vars, result); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, residualequations: elst, innerEquations, .. }, .. }, tail: rest } => {
                let mut vlst1: metamodelica::List<i32>;
                let mut elst1: metamodelica::List<i32>;
                let mut vlst1Lst: metamodelica::List<metamodelica::List<i32>>;
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut varlst1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnlst1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut result: metamodelica::List<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)>;
                (elst1, vlst1Lst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                vlst1 = List::flatten(vlst1Lst)?;
                varlst1 = List::map1r(vlst1, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                varlst = listAppend(varlst1, varlst);
                eqnlst1 = BackendEquation::getList(elst1, eqns.clone())?;
                eqnlst = BackendEquation::getList(elst.clone(), eqns.clone())?;
                eqnlst = listAppend(eqnlst1, eqnlst);
                result = listAppend(inAccum, list![(eqnlst, varlst)]);
                { (inComps, eqns, vars, inAccum) = (rest, eqns, vars, result); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                Debug::traceln(literal!("XMLDump.getOrderedEqs2 failed!"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn dumpDAEVariableAttributes(
    mut dae_var_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut Content: ArcStr,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (dae_var_attr, addMathMLCode);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: _, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: None, min: None, max: None, start: None, fixed: None, uncertainOption: _, distributionOption: _, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: None, start: None, fixed: None, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: None, start: None, fixed: _, equationBound: _, isProtected: _, finalPrefix: _, .. }), _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: None, min: None, max: None, start: None, fixed: None, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quant, unit, displayUnit, min, max, start: Initial, fixed, nominal, stateSelectOption: stateSel, uncertainOption: _, distributionOption: _, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), addMMLCode) => {
                    dumpStrOpenTag(Content.clone())?;
                    dumpOptExp(quant.clone(), arcstr::literal!(VAR_ATTR_QUANTITY), addMMLCode.clone())?;
                    dumpOptExp(unit.clone(), arcstr::literal!(VAR_ATTR_UNIT), addMMLCode.clone())?;
                    dumpOptExp(displayUnit.clone(), arcstr::literal!(VAR_ATTR_DISPLAY_UNIT), addMMLCode.clone())?;
                    dumpOptionDAEStateSelect(stateSel.clone(), arcstr::literal!(VAR_ATTR_STATESELECT))?;
                    dumpOptExp(min.clone(), arcstr::literal!(VAR_ATTR_MINVALUE), addMMLCode.clone())?;
                    dumpOptExp(max.clone(), arcstr::literal!(VAR_ATTR_MAXVALUE), addMMLCode.clone())?;
                    dumpOptExp(nominal.clone(), arcstr::literal!(VAR_ATTR_NOMINAL), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), arcstr::literal!(VAR_ATTR_INITIALVALUE), addMMLCode.clone())?;
                    dumpOptExp(fixed.clone(), arcstr::literal!(VAR_ATTR_FIXED), addMMLCode.clone())?;
                    dumpStrCloseTag(Content.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: quant, min, max, start: Initial, fixed, uncertainOption: _, distributionOption: _, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), addMMLCode) => {
                    dumpStrOpenTag(Content.clone())?;
                    dumpOptExp(quant.clone(), arcstr::literal!(VAR_ATTR_QUANTITY), addMMLCode.clone())?;
                    dumpOptExp(min.clone(), arcstr::literal!(VAR_ATTR_MINVALUE), addMMLCode.clone())?;
                    dumpOptExp(max.clone(), arcstr::literal!(VAR_ATTR_MAXVALUE), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), arcstr::literal!(VAR_ATTR_INITIALVALUE), addMMLCode.clone())?;
                    dumpOptExp(fixed.clone(), arcstr::literal!(VAR_ATTR_FIXED), addMMLCode.clone())?;
                    dumpStrCloseTag(Content.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quant, start: Initial, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), addMMLCode) => {
                    dumpStrOpenTag(Content.clone())?;
                    dumpOptExp(quant.clone(), arcstr::literal!(VAR_ATTR_QUANTITY), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), arcstr::literal!(VAR_ATTR_INITIALVALUE), addMMLCode.clone())?;
                    dumpOptExp(fixed.clone(), arcstr::literal!(VAR_ATTR_FIXED), addMMLCode.clone())?;
                    dumpStrCloseTag(Content.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quant, start: Initial, fixed: _, equationBound: _, isProtected: _, finalPrefix: _, .. }), addMMLCode) => {
                    dumpStrOpenTag(Content.clone())?;
                    dumpOptExp(quant.clone(), arcstr::literal!(VAR_ATTR_QUANTITY), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), arcstr::literal!(VAR_ATTR_INITIALVALUE), addMMLCode.clone())?;
                    dumpStrCloseTag(Content.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quant, min, max, start: Initial, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), addMMLCode) => {
                    dumpStrOpenTag(Content.clone())?;
                    dumpOptExp(quant.clone(), arcstr::literal!(VAR_ATTR_QUANTITY), addMMLCode.clone())?;
                    dumpOptExp(min.clone(), arcstr::literal!(VAR_ATTR_MINVALUE), addMMLCode.clone())?;
                    dumpOptExp(max.clone(), arcstr::literal!(VAR_ATTR_MAXVALUE), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), arcstr::literal!(VAR_ATTR_INITIALVALUE), addMMLCode.clone())?;
                    dumpOptExp(fixed.clone(), arcstr::literal!(VAR_ATTR_FIXED), addMMLCode.clone())?;
                    dumpStrCloseTag(Content.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    dumpComment(literal!("unknown VariableAttributes"))?;
                    Ok(())
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

fn dumpDirectionStr(mut inVarDirection: DAE::VarDirection) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inVarDirection {
        DAE::VarDirection::INPUT { .. } => {
            arcstr::literal!(VARDIR_INPUT)
        }
        DAE::VarDirection::OUTPUT { .. } => {
            arcstr::literal!(VARDIR_OUTPUT)
        }
        DAE::VarDirection::BIDIR { .. } => {
            arcstr::literal!(VARDIR_NONE)
        }
        _ => {
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.dumpDirectionStr - Unknown var direction");
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn dumpSolvedEqns(
    mut eqns: &metamodelica::List<(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    )>,
    mut inCount: i32,
    mut inContent: &ArcStr,
    mut addMathMLCode: bool,
    mut dumpResiduals: bool,
    mut dumpSolved: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match eqns {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (Deref @ metamodelica::ListNode::Nil, _), tail: rest } => {
            dumpSolvedEqns(rest, inCount, inContent, addMathMLCode, dumpResiduals, dumpSolved)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (eqnsLst, varLst), tail: rest } => {
            let mut addMMLCode = addMathMLCode;
            dumpEqns2(metamodelica::AsArg::as_arg(&eqnsLst), metamodelica::AsArg::as_arg(&varLst), inCount, addMMLCode, dumpResiduals, dumpSolved)?;
            dumpSolvedEqns(rest, inCount + 1, inContent, addMathMLCode, dumpResiduals, dumpSolved)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpEqns(
    mut eqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inContent: ArcStr,
    mut addMathMLCode: bool,
    mut dumpResiduals: bool,
    mut dumpSolved: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match eqns {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => {
            let mut addMMLCode = addMathMLCode;
            let mut len: i32;
            len = ((eqns).len() as i32);
            dumpStrOpenTagAttr(inContent.clone(), arcstr::literal!(DIMENSION), intString(len))?;
            dumpEqns2(eqns, &(metamodelica::nil()), 1, addMMLCode, dumpResiduals, dumpSolved)?;
            dumpStrCloseTag(inContent)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpEqns2(
    mut inEquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inInteger: i32,
    mut addMathMLCode: bool,
    mut dumpResiduals: bool,
    mut dumpSolved: bool,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (
            &**inEquationLst,
            &**inVarLst,
            inInteger,
            addMathMLCode,
            dumpResiduals,
            dumpSolved,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _, _, _, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns }, _, index, addMMLCode, false, false) => {
                    dumpEquation(metamodelica::AsArg::as_arg(&eqn), intString(index.clone()), addMMLCode.clone())?;
                    dumpEqns2(metamodelica::AsArg::as_arg(&eqns), inVarLst, index.clone() + 1, addMMLCode.clone(), false, false)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns }, _, index, addMMLCode, true, false) => {
                    dumpResidual(metamodelica::AsArg::as_arg(&eqn), intString(index.clone()), addMMLCode.clone())?;
                    dumpEqns2(metamodelica::AsArg::as_arg(&eqns), inVarLst, index.clone() + 1, addMMLCode.clone(), true, false)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns }, Deref @ metamodelica::ListNode::Cons { head: var, tail: vars }, index, addMMLCode, false, true) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut varexp: metamodelica::Ref<DAE::Exp>;
                    let mut eqn = (*eqn).clone();
                    cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
                    varexp = Expression::crefExp(cref.clone())?;
                    varexp = if (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&var))) {Expression::expDer(varexp.clone())} else {varexp.clone()};
                    eqn = BackendEquation::solveEquation(eqn.clone(), varexp.clone(), None)?;
                    dumpEquation(metamodelica::AsArg::as_arg(&eqn), intString(index.clone()), addMMLCode.clone())?;
                    dumpEqns2(metamodelica::AsArg::as_arg(&eqns), metamodelica::AsArg::as_arg(&vars), index.clone() + 1, addMMLCode.clone(), false, true)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns }, Deref @ metamodelica::ListNode::Cons { head: _, tail: vars }, index, addMMLCode, false, true) => {
                    dumpEquation(metamodelica::AsArg::as_arg(&eqn), intString(index.clone()), addMMLCode.clone())?;
                    dumpEqns2(metamodelica::AsArg::as_arg(&eqns), metamodelica::AsArg::as_arg(&vars), index.clone() + 1, addMMLCode.clone(), false, true)?;
                    Ok(())
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

fn dumpEquation(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut inIndexNumber: ArcStr,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inEquation.clone(), addMathMLCode)) {
        (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, true) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            dumpStrOpenTagAttr(arcstr::literal!(EQUATION), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(arcstr::literal!(EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            dumpStrOpenTagAttr(arcstr::literal!(EQUATION), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(arcstr::literal!(EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }, true) => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            s = stringAppendList(list![s1, literal!(" = "), s2, literal!("\n")]);
            dumpStrOpenTagAttr(arcstr::literal!(ARRAY_EQUATION), arcstr::literal!(EXP_STRING), s)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(arcstr::literal!(ARRAY_EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            dumpStrOpenTagAttr(arcstr::literal!(ARRAY_OF_EQUATIONS), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(arcstr::literal!(ARRAY_OF_EQUATIONS))?;
            ()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. }, true) => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            s = stringAppendList(list![s1, literal!(" = "), s2, literal!("\n")]);
            dumpStrOpenTagAttr(arcstr::literal!(COMPLEX_EQUATION), arcstr::literal!(EXP_STRING), s)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(arcstr::literal!(COMPLEX_EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. }, _) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            dumpStrOpenTagAttr(arcstr::literal!(COMPLEX_EQUATION), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(arcstr::literal!(COMPLEX_EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. }, true) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1.clone(), literal!(" := "), s2]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(SOLVED), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpStrMathMLVariable(s1)?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(SOLVED), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" := "), s2]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(SOLVED), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(SOLVED), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition: e1, whenStmtLst, .. }, .. }, true) => {
            let mut indexS = inIndexNumber;
            let mut is: ArcStr;
            is = printExpStr(e1.clone())?;
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&whenStmtLst), addMathMLCode)?;
            dumpStrOpenTag(stringAppend(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(CONDITION)))?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(is)?;
            dumpExp(e1.clone(), true);
            dumpStrCloseTag(stringAppend(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(CONDITION)))?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition: e1, whenStmtLst, .. }, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut is: ArcStr;
            is = printExpStr(e1.clone())?;
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&whenStmtLst), addMathMLCode)?;
            dumpStrTagContent(stringAppend(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(CONDITION)), is)?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, true) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e.clone())?;
            res = stringAppendList(list![s1, literal!(" = 0")]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(RESIDUAL), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e))?;
            dumpStrMathMLNumber(literal!("0"))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(RESIDUAL), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e.clone())?;
            res = stringAppendList(list![s1, literal!(" = 0")]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(RESIDUAL), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(RESIDUAL), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: stmts }, source, .. }, _) => {
            let mut indexS = inIndexNumber;
            dumpStrOpenTagAttr(arcstr::literal!(ALGORITHM), arcstr::literal!(ID), indexS)?;
            Print::printBuf(Util::xmlEscape(DAEDump::dumpAlgorithmsStr(&(list![metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts.clone() }), source: source.clone() })]))?)?)?;
            dumpStrCloseTag(arcstr::literal!(ALGORITHM))?;
            ()
        },
        _ => {
            let mut res: ArcStr;
            res = literal!("in XMLDump.dumpEquation - Unknown equation");
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![res])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpExp(mut e: metamodelica::Ref<DAE::Exp>, mut addMathMLCode: bool) -> () {
    let () = 'mc: {
        let __mc_input = (e, addMathMLCode);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inExp, true) => {
                    dumpStrOpenTag(arcstr::literal!(MathML))?;
                    dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&inExp))?;
                    dumpStrCloseTag(arcstr::literal!(MATH))?;
                    dumpStrCloseTag(arcstr::literal!(MathML))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn dumpExp2(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { integer: x } => {
                    dumpStrMathMLNumberAttr(intString(x.clone()), arcstr::literal!(MathMLType), arcstr::literal!(MathMLInteger))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { real: rval } => {
                    dumpStrMathMLNumberAttr(realString(rval.clone()), arcstr::literal!(MathMLType), arcstr::literal!(MathMLReal))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    dumpStrMathMLNumberAttr(Util::xmlEscape(s.clone())?, arcstr::literal!(MathMLType), arcstr::literal!(MathMLConstant))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: false } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLFalse))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: true } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLTrue))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: c, .. } => {
                    let mut s: ArcStr;
                    s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    dumpStrMathMLVariable(s.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut sym: ArcStr;
                    sym = binopSymbol(op.clone())?;
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(sym.clone())?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
                    let mut sym: ArcStr;
                    sym = unaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(sym.clone())?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut sym: ArcStr;
                    sym = lbinopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(sym.clone())?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
                    let mut sym: ArcStr;
                    sym = lunaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(sym.clone())?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. } => {
                    let mut sym: ArcStr;
                    sym = relopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(sym.clone())?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: tb, expElse: fb } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLIfClause))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLIfBranch))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&tb))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&cond))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLIfBranch))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLElseBranch))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&fb))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLElseBranch))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLIfClause))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: args, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(literal!("diff"))?;
                    dumpList(metamodelica::AsArg::as_arg(&args), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, expLst: args, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLArccos))?;
                    dumpList(metamodelica::AsArg::as_arg(&args), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, expLst: args, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLArcsin))?;
                    dumpList(metamodelica::AsArg::as_arg(&args), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, expLst: args, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLArctan))?;
                    dumpList(metamodelica::AsArg::as_arg(&args), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, expLst: args, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!("atan2"))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!("("))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpList(metamodelica::AsArg::as_arg(&args), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpComment(literal!("atan2 is not a MathML element it could be possible to use arg in future"))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!(")"))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: args, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLLn))?;
                    dumpList(metamodelica::AsArg::as_arg(&args), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: args, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLLog))?;
                    dumpList(metamodelica::AsArg::as_arg(&args), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: fcn, expLst: args, .. } => {
                    let mut fs: ArcStr;
                    fs = AbsynUtil::pathStringNoQual(fcn.clone(), literal!("."), false, false)?;
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(fs.clone())?;
                    dumpList(metamodelica::AsArg::as_arg(&args), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: es, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLTranspose))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLVector))?;
                    dumpList(metamodelica::AsArg::as_arg(&es), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLVector))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: es } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLTranspose))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLVector))?;
                    dumpList(metamodelica::AsArg::as_arg(&es), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLVector))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: ebs, .. } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLMatrix))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLMatrixrow))?;
                    dumpListSeparator(metamodelica::AsArg::as_arg(&ebs), (std::sync::Arc::new(move |__a0: metamodelica::List<metamodelica::Ref<DAE::Exp>>| dumpRow(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<()> + 'static>), stringAppendList(list![literal!("\n</"), arcstr::literal!(MathMLMatrixrow), literal!(">\n<"), arcstr::literal!(MathMLMatrixrow), literal!(">")]))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLMatrixrow))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLMatrix))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start, step: None, stop } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLInterval))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&start))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&stop))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLInterval))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start, step: Some(step), stop } => {
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!("{"))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&start))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!(":"))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&step))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!(":"))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&stop))?;
                    dumpComment(literal!("Interval range specification is not supported by MathML standard"))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!("}"))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: Deref @ DAE::Exp::ICONST { integer: ival } } => {
                    let mut res: ArcStr;
                    let mut rval: metamodelica::Real;
                    let false = (Config::modelicaOutput()?) else { return Err("pattern mismatch") };
                    rval = intReal(ival.clone());
                    res = realString(rval);
                    dumpStrMathMLNumberAttr(res.clone(), arcstr::literal!(MathMLType), arcstr::literal!(MathMLReal))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::ICONST { integer: ival } } } => {
                    let mut res: ArcStr;
                    let mut rval: metamodelica::Real;
                    let false = (Config::modelicaOutput()?) else { return Err("pattern mismatch") };
                    rval = intReal(ival.clone());
                    res = realString(rval);
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLMinus))?;
                    dumpStrMathMLNumberAttr(res.clone(), arcstr::literal!(MathMLType), arcstr::literal!(MathMLReal))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: e } => {
                    let false = (Config::modelicaOutput()?) else { return Err("pattern mismatch") };
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrVoidTag(arcstr::literal!(MathMLReal))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: e } => {
                    let true = (Config::modelicaOutput()?) else { return Err("pattern mismatch") };
                    dumpExp2(metamodelica::AsArg::as_arg(&e))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: tp, exp: e } => {
                    let mut r#str: ArcStr;
                    r#str = TypesDump::unparseType(tp.clone())?;
                    dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!("("))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!("CAST as "))?;
                    Print::printBuf(r#str.clone())?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpExp2(metamodelica::AsArg::as_arg(&e))?;
                    dumpComment(literal!("CAST operator is not supported by MathML standard."))?;
                    dumpStrOpenTag(arcstr::literal!(MathMLOperator))?;
                    Print::printBuf(literal!(")"))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLOperator))?;
                    dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
                            let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
                            dumpStrVoidTag(arcstr::literal!(MathMLSelector))?;
                            dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
                            dumpList(&args, (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>))?;
                            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
                            Ok(())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ENUM_LITERAL { name: fcn, .. } => {
                    dumpStrMathMLVariable(AbsynUtil::pathStringNoQual(fcn.clone(), literal!("."), false, false)?)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { sz: Some(_), .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { sz: None, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LIST { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CONS { .. } => {
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
                    dumpComment({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNKNOWN EXPRESSION: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(())
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

fn dumpExtObjCls(mut cls: metamodelica::List<BackendDAE::ExternalObjectClass>, mut Content: &ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(cls) {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        xs => {
            let mut len: i32;
            len = ((xs).len() as i32);
            dumpStrOpenTagAttr(stringAppend(stringAppend(arcstr::literal!(EXTERNAL), arcstr::literal!(CLASSES_)), arcstr::literal!(LIST_)), arcstr::literal!(DIMENSION), intString(len))?;
            dumpExtObjCls2(metamodelica::AsArg::as_arg(&xs), stringAppend(arcstr::literal!(EXTERNAL), arcstr::literal!(CLASS_)))?;
            dumpStrCloseTag(stringAppend(stringAppend(arcstr::literal!(EXTERNAL), arcstr::literal!(CLASSES_)), arcstr::literal!(LIST_)))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpExtObjCls2(mut cls: &metamodelica::List<BackendDAE::ExternalObjectClass>, mut Content: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match cls {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: BackendDAE::ExternalObjectClass { path, source: _ }, tail: xs } => {
            let mut c = Content;
            dumpStrOpenTag(c.clone())?;
            Print::printBuf(literal!("class "))?;
            Print::printBuf(AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?)?;
            Print::printBuf(literal!("\n  extends ExternalObject"))?;
            Print::printBuf(literal!("end"))?;
            Print::printBuf(AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?)?;
            dumpStrCloseTag(c.clone())?;
            dumpExtObjCls2(xs, c)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpFlowStr(mut inVarFlow: &metamodelica::Ref<DAE::ConnectorType>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match &**inVarFlow {
        DAE::ConnectorType::FLOW { .. } => arcstr::literal!(VAR_FLOW_FLOW),
        DAE::ConnectorType::POTENTIAL { .. } => arcstr::literal!(VAR_FLOW_NONFLOW),
        DAE::ConnectorType::STREAM { .. } => arcstr::literal!(VAR_FLOW_NONFLOW),
        DAE::ConnectorType::NON_CONNECTOR { .. } => arcstr::literal!(VAR_FLOW_NONCONNECTOR),
    });
    outString
}

fn dumpFunctions(mut funcelems: &metamodelica::List<DAE::Function>) -> Result<()> {
    let () = (::match_deref::match_deref! { match funcelems {
        Deref @ metamodelica::ListNode::Nil => (),
        _ => {
            dumpStrOpenTag(arcstr::literal!(FUNCTIONS))?;
            dumpFunctions2(funcelems);
            dumpStrCloseTag(arcstr::literal!(FUNCTIONS))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpFunctions2(mut funcelems: &metamodelica::List<DAE::Function>) -> () {
    let () = (::match_deref::match_deref! { match funcelems {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: fun, tail: rem_fun } => {
            dumpFunctions3(metamodelica::AsArg::as_arg(&fun));
            dumpFunctions2(rem_fun);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn dumpFunctions3(mut fun: &DAE::Function) -> () {
    let () = 'mc: {
        let __mc_input = fun;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { type_: Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: _, .. }, .. }, .. }, .. } => {
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
                    dumpStrOpenTagAttr(arcstr::literal!(FUNCTION), arcstr::literal!(FUNCTION_NAME), Util::xmlEscape(AbsynUtil::pathStringNoQual(DAEUtil::functionName(fun), literal!("."), false, false)?)?)?;
                    dumpStrOpenTag(arcstr::literal!(MODELICA_IMPLEMENTATION))?;
                    Print::printBuf(Util::xmlEscape(DAEDump::dumpFunctionStr(fun))?)?;
                    dumpStrCloseTag(arcstr::literal!(MODELICA_IMPLEMENTATION))?;
                    dumpStrCloseTag(arcstr::literal!(FUNCTION))?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn dumpAdjacencyMatrix(mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<()> {
    dumpStrOpenTag(arcstr::literal!(MathML))?;
    dumpStrOpenTagAttr(
        arcstr::literal!(MATH),
        arcstr::literal!(MathMLXmlns),
        arcstr::literal!(MathMLWeb),
    )?;
    dumpStrOpenTag(arcstr::literal!(MathMLMatrix))?;
    BackendDAEUtil::foldEqSystem(
        dae,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: i32| dumpAdjacencyMatrixWork(__a0, &__a1, __a2),
        0,
    )?;
    dumpStrCloseTag(arcstr::literal!(MathMLMatrix))?;
    dumpStrCloseTag(arcstr::literal!(MATH))?;
    dumpStrCloseTag(arcstr::literal!(MathML))?;
    Ok(())
}

fn dumpAdjacencyMatrixWork(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inOffset: i32,
) -> Result<i32> {
    let mut outOffset: i32;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    funcs = BackendDAEUtil::getFunctions(shared);
    (_, m, _) = BackendDAEUtil::getAdjacencyMatrixfromOption(
        syst,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        Some(funcs),
        BackendDAEUtil::isInitializationDAE(shared),
    )?;
    Array::fold(
        m.clone(),
        &move |__a0: metamodelica::List<i32>, __a1: (i32, i32)| dumpAdjacencyMatrix2(&__a0, __a1),
        (inOffset, 1),
    )?;
    outOffset = inOffset + metamodelica::arrayLength(m.clone());
    Ok(outOffset)
}

fn dumpAdjacencyMatrix2(mut row: &metamodelica::List<i32>, mut inTpl: (i32, i32)) -> Result<(i32, i32)> {
    let mut outTpl: (i32, i32);
    let mut offset: i32;
    let mut c: i32;
    (offset, c) = inTpl;
    dumpStrOpenTagAttr(arcstr::literal!(MathMLMatrixrow), literal!("id"), intString(c))?;
    List::map1_0(row, &dumpMatrixIntegerRow, offset)?;
    dumpStrCloseTag(arcstr::literal!(MathMLMatrixrow))?;
    outTpl = (offset, c + 1);
    Ok(outTpl)
}

fn dumpMatrixIntegerRow(mut x: i32, mut offset: i32) -> Result<()> {
    let mut e: i32;
    let mut s: ArcStr;
    e = if (intGt(x, 0)) { x + offset } else { x - offset };
    s = intString(e);
    dumpStrOpenTag(arcstr::literal!(MathMLVariable))?;
    Print::printBuf(s)?;
    dumpStrCloseTag(arcstr::literal!(MathMLVariable))?;
    Ok(())
}

fn dumpKind(mut inVarKind: &BackendDAE::VarKind) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inVarKind.clone() {
        BackendDAE::VarKind::VARIABLE { .. } => {
            arcstr::literal!(VARIABILITY_CONTINUOUS)
        }
        BackendDAE::VarKind::STATE { .. } => {
            arcstr::literal!(VARIABILITY_CONTINUOUS_STATE)
        }
        BackendDAE::VarKind::DUMMY_DER { .. } => {
            arcstr::literal!(VARIABILITY_CONTINUOUS_DUMMYDER)
        }
        BackendDAE::VarKind::DUMMY_STATE { .. } => {
            arcstr::literal!(VARIABILITY_CONTINUOUS_DUMMYSTATE)
        }
        BackendDAE::VarKind::DISCRETE { .. } => {
            arcstr::literal!(VARIABILITY_DISCRETE)
        }
        BackendDAE::VarKind::PARAM { .. } => {
            arcstr::literal!(VARIABILITY_PARAMETER)
        }
        BackendDAE::VarKind::CONST { .. } => {
            arcstr::literal!(VARIABILITY_CONSTANT)
        }
        BackendDAE::VarKind::EXTOBJ {
            fullClassName: ref path,
        } => stringAppend(
            arcstr::literal!(VARIABILITY_EXTERNALOBJECT),
            stringAppend(
                literal!(":"),
                AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?,
            ),
        ),
        _ => {
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.dumpKind - Unknown kind");
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn dumpList<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeALst: &metamodelica::List<Type_a>,
    mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>,
) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>;

    let () = 'mc: {
        let __mc_input = (&**inTypeALst, inFuncTypeTypeATo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: Deref @ metamodelica::ListNode::Nil }, r) => {
                    r(h.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: t }, r) => {
                    r(h.clone())?;
                    dumpList(metamodelica::AsArg::as_arg(&t), r.clone())?;
                    Ok(())
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

fn dumpListSeparator<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeALst: &metamodelica::List<Type_a>,
    mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>,
    mut inString: ArcStr,
) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>;

    let () = 'mc: {
        let __mc_input = (&**inTypeALst, inFuncTypeTypeATo.clone(), inString);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: Deref @ metamodelica::ListNode::Nil }, r, _) => {
                    r(h.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: t }, r, sep) => {
                    r(h.clone())?;
                    Print::printBuf(sep.clone())?;
                    dumpListSeparator(metamodelica::AsArg::as_arg(&t), r.clone(), sep.clone())?;
                    Ok(())
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

fn printExpStr(mut e: metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = Util::xmlEscape(ExpressionBasics::printExpStr(e)?)?;
    Ok(s)
}

fn dumpLstInt(mut inLstStr: &metamodelica::List<i32>, mut inElementName: &ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**inLstStr, inElementName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ "") => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
                    dumpStrTagContent(inElementName.clone(), intString(h.clone()))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: t }, _) => {
                    dumpStrTagContent(inElementName.clone(), intString(h.clone()))?;
                    dumpLstInt(metamodelica::AsArg::as_arg(&t), inElementName)?;
                    Ok(())
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

fn dumpLstIntAttr(mut lst: metamodelica::List<i32>, mut inContent: ArcStr, mut inElementContent: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(lst) {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        l => {
            let mut inLst = inContent;
            let mut inEl = inElementContent;
            dumpStrOpenTag(inLst.clone())?;
            dumpLstInt(metamodelica::AsArg::as_arg(&l), &inEl)?;
            dumpStrCloseTag(inLst)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpMatching(mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<()> {
    dumpStrOpenTag(arcstr::literal!(MATCHING_ALGORITHM))?;
    BackendDAEUtil::foldEqSystem(
        dae,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: (i32, i32)| dumpMatchingWork(&__a0, &__a1, __a2),
        (0, 0),
    )?;
    dumpStrCloseTag(arcstr::literal!(MATCHING_ALGORITHM))?;
    Ok(())
}

fn dumpMatchingWork(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inOffset: (i32, i32),
) -> Result<(i32, i32)> {
    let mut outOffset: (i32, i32);
    let mut v1: metamodelica::Array<i32>;
    let mut v2: metamodelica::Array<i32>;
    let mut voffset: i32;
    let mut eoffset: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*syst)) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: _ }, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    v1 = metamodelica::Own::own(__pa0);
    v2 = metamodelica::Own::own(__pa1);
    (voffset, eoffset) = inOffset;
    dumpMatching1(v1.clone(), voffset, eoffset)?;
    outOffset = (
        voffset + metamodelica::arrayLength(v1.clone()),
        eoffset + metamodelica::arrayLength(v2.clone()),
    );
    Ok(outOffset)
}

fn dumpMatching1(mut v: metamodelica::Array<i32>, mut voffset: i32, mut eoffset: i32) -> Result<()> {
    if intGt(metamodelica::arrayLength(v.clone()), 0) {
        Array::fold(v.clone(), &dumpMatching2, (1, voffset, eoffset))?;
    }
    Ok(())
}

fn dumpMatching2(mut eqn: i32, mut inTpl: (i32, i32, i32)) -> Result<(i32, i32, i32)> {
    let mut outTpl: (i32, i32, i32);
    let mut v: i32;
    let mut voffset: i32;
    let mut eoffset: i32;
    let mut s: ArcStr;
    let mut s2: ArcStr;
    (v, voffset, eoffset) = inTpl;
    s = intString(v + voffset);
    s2 = intString(eqn + eoffset);
    Print::printBuf(stringAppendList(list![
        literal!("\n<"),
        arcstr::literal!(SOLVED_IN),
        literal!(" "),
        arcstr::literal!(VARIABLE),
        arcstr::literal!(ID_),
        literal!("=\""),
        s,
        literal!("\" "),
        arcstr::literal!(EQUATION),
        arcstr::literal!(ID_),
        literal!("=\""),
        s2,
        literal!("\" "),
        literal!("/>")
    ]))?;
    outTpl = (v + 1, voffset, eoffset);
    Ok(outTpl)
}

fn dumpOptExp(
    mut inExpExpOption: Option<metamodelica::Ref<DAE::Exp>>,
    mut Content: ArcStr,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inExpExpOption) {
        None => {
            ()
        },
        Some(e) => {
            dumpStrOpenTagAttr(Content.clone(), arcstr::literal!(EXP_STRING), printExpStr(e.clone())?)?;
            dumpExp(e.clone(), addMathMLCode);
            dumpStrCloseTag(Content)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpOptInteger(mut inOption: Option<i32>, mut Content: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let () = (match inOption {
        None => (),
        Some(mut i) => {
            dumpStrOpenTagAttr(Content.clone(), arcstr::literal!(INDEX), intString(i))?;
            dumpStrCloseTag(Content)?;
            ()
        }
    });
    Ok(())
}

fn dumpOptionDAEStateSelect(mut ss: Option<DAE::StateSelect>, mut Content: ArcStr) -> Result<()> {
    let () = (match ss {
        None => {
            Print::printBuf(literal!(""))?;
            ()
        }
        Some(DAE::StateSelect::NEVER { .. }) => {
            dumpStrTagContent(Content, arcstr::literal!(STATE_SELECT_NEVER))?;
            ()
        }
        Some(DAE::StateSelect::AVOID { .. }) => {
            dumpStrTagContent(Content, arcstr::literal!(STATE_SELECT_AVOID))?;
            ()
        }
        Some(DAE::StateSelect::DEFAULT { .. }) => {
            dumpStrTagContent(Content, arcstr::literal!(STATE_SELECT_DEFAULT))?;
            ()
        }
        Some(DAE::StateSelect::PREFER { .. }) => {
            dumpStrTagContent(Content, arcstr::literal!(STATE_SELECT_PREFER))?;
            ()
        }
        Some(DAE::StateSelect::ALWAYS { .. }) => {
            dumpStrTagContent(Content, arcstr::literal!(STATE_SELECT_ALWAYS))?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn dumpRow(mut es_1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<()> {
    dumpList(
        es_1,
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| dumpExp2(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>),
    )?;
    Ok(())
}

fn dumpSolvingInfo(
    mut addOriginalAdjacencyMatrix: bool,
    mut addSolvingInfo: bool,
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<()> {
    let () = (match (addOriginalAdjacencyMatrix, addSolvingInfo) {
        (false, false) => (),
        (true, true) => {
            let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
            dlow = BackendDAEUtil::transformBackendDAE(inBackendDAE, None, None, None)?;
            dumpStrOpenTag(arcstr::literal!(ADDITIONAL_INFO))?;
            dumpStrOpenTag(arcstr::literal!(ORIGINAL_ADJACENCY_MATRIX))?;
            dumpAdjacencyMatrix(&dlow)?;
            dumpStrCloseTag(arcstr::literal!(ORIGINAL_ADJACENCY_MATRIX))?;
            dumpStrOpenTag(arcstr::literal!(SOLVING_INFO))?;
            dumpMatching(&dlow)?;
            dumpComponents(&dlow)?;
            dumpStrCloseTag(arcstr::literal!(SOLVING_INFO))?;
            dumpStrCloseTag(arcstr::literal!(ADDITIONAL_INFO))?;
            ()
        }
        (true, false) => {
            dumpStrOpenTag(arcstr::literal!(ADDITIONAL_INFO))?;
            dumpStrOpenTag(arcstr::literal!(ORIGINAL_ADJACENCY_MATRIX))?;
            dumpAdjacencyMatrix(inBackendDAE)?;
            dumpStrCloseTag(arcstr::literal!(ORIGINAL_ADJACENCY_MATRIX))?;
            dumpStrCloseTag(arcstr::literal!(ADDITIONAL_INFO))?;
            ()
        }
        (false, true) => {
            let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
            dlow = BackendDAEUtil::transformBackendDAE(inBackendDAE, None, None, None)?;
            dumpStrOpenTag(arcstr::literal!(ADDITIONAL_INFO))?;
            dumpStrOpenTag(arcstr::literal!(SOLVING_INFO))?;
            dumpMatching(&dlow)?;
            dumpComponents(&dlow)?;
            dumpStrCloseTag(arcstr::literal!(SOLVING_INFO))?;
            dumpStrCloseTag(arcstr::literal!(ADDITIONAL_INFO))?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn transformModelicaIdentifierToXMLElementTag(mut modelicaIdentifier: ArcStr) -> Result<ArcStr> {
    let mut xmlElementTag: ArcStr;
    xmlElementTag = System::stringReplace(modelicaIdentifier, literal!("$"), literal!("_dollar_"))?;
    Ok(xmlElementTag)
}

fn dumpStrCloseTag(mut inContent: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inContent) {
        Deref @ "" => {
            ()
        },
        inString => {
            Print::printBuf(literal!("\n</"))?;
            Print::printBuf(transformModelicaIdentifierToXMLElementTag(inString.clone())?)?;
            Print::printBuf(literal!(">"))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpStreamStr(mut inVarStream: &metamodelica::Ref<DAE::ConnectorType>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match &**inVarStream {
        DAE::ConnectorType::STREAM { .. } => arcstr::literal!(VAR_STREAM_STREAM),
        DAE::ConnectorType::POTENTIAL { .. } => arcstr::literal!(VAR_STREAM_NONSTREAM),
        DAE::ConnectorType::FLOW { .. } => arcstr::literal!(VAR_STREAM_NONSTREAM),
        DAE::ConnectorType::NON_CONNECTOR { .. } => arcstr::literal!(VAR_STREAM_NONSTREAM_CONNECTOR),
    });
    outString
}

fn dumpStrMathMLNumber(mut inNumber: ArcStr) -> Result<()> {
    dumpStrOpenTag(arcstr::literal!(MathMLNumber))?;
    Print::printBuf(inNumber)?;
    dumpStrCloseTag(arcstr::literal!(MathMLNumber))?;
    Ok(())
}

fn dumpStrMathMLNumberAttr(
    mut inNumber: ArcStr,
    mut inAttribute: ArcStr,
    mut inAttributeContent: ArcStr,
) -> Result<()> {
    dumpStrOpenTagAttr(arcstr::literal!(MathMLNumber), inAttribute, inAttributeContent)?;
    Print::printBuf(inNumber)?;
    dumpStrCloseTag(arcstr::literal!(MathMLNumber))?;
    Ok(())
}

fn dumpStrMathMLVariable(mut inVariable: ArcStr) -> Result<()> {
    dumpStrOpenTag(arcstr::literal!(MathMLVariable))?;
    Print::printBuf(inVariable)?;
    dumpStrCloseTag(arcstr::literal!(MathMLVariable))?;
    Ok(())
}

fn dumpStrOpenTag(mut inContent: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inContent;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    Print::printBuf(literal!(""))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                inString => {
                    Print::printBuf(literal!("\n<"))?;
                    Print::printBuf(transformModelicaIdentifierToXMLElementTag(inString.clone())?)?;
                    Print::printBuf(literal!(">"))?;
                    Ok(())
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

fn dumpStrOpenTagAttr(mut inContent: ArcStr, mut Attribute: ArcStr, mut AttributeContent: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inContent, Attribute.clone(), AttributeContent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", _, _) => {
                    Print::printBuf(literal!(""))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ "", _) => {
                    Print::printBuf(literal!(""))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ "") => {
                    Print::printBuf(literal!(""))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, Deref @ "", _) => {
                    dumpStrOpenTag(transformModelicaIdentifierToXMLElementTag(inString.clone())?)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, _, Deref @ "") => {
                    dumpStrOpenTag(transformModelicaIdentifierToXMLElementTag(inString.clone())?)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, _, inAttributeContent) => {
                    Print::printBuf(literal!("\n<"))?;
                    Print::printBuf(transformModelicaIdentifierToXMLElementTag(inString.clone())?)?;
                    Print::printBuf(literal!(" "))?;
                    Print::printBuf(Attribute.clone())?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(inAttributeContent.clone())?;
                    Print::printBuf(literal!("\">"))?;
                    Ok(())
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

fn dumpStrTagAttrNoChild(mut inContent: ArcStr, mut Attribute: ArcStr, mut AttributeContent: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inContent, Attribute.clone(), AttributeContent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", _, _) => {
                    Print::printBuf(literal!(""))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ "", _) => {
                    Print::printBuf(literal!(""))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ "") => {
                    Print::printBuf(literal!(""))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, Deref @ "", _) => {
                    dumpStrOpenTag(transformModelicaIdentifierToXMLElementTag(inString.clone())?)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, _, Deref @ "") => {
                    dumpStrOpenTag(transformModelicaIdentifierToXMLElementTag(inString.clone())?)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, _, inAttributeContent) => {
                    Print::printBuf(literal!("\n<"))?;
                    Print::printBuf(transformModelicaIdentifierToXMLElementTag(inString.clone())?)?;
                    Print::printBuf(literal!(" "))?;
                    Print::printBuf(Attribute.clone())?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(inAttributeContent.clone())?;
                    Print::printBuf(literal!("\" />"))?;
                    Ok(())
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

fn dumpStrTagContent(mut inElementName: ArcStr, mut inContent: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inElementName, inContent)) {
        (Deref @ "", _) => {
            ()
        },
        (_, Deref @ "") => {
            ()
        },
        (inTagString, inTagContent) => {
            dumpStrOpenTag(inTagString.clone())?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(inTagContent.clone())?;
            dumpStrCloseTag(inTagString.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpStrVoidTag(mut inElementName: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inElementName) {
        Deref @ "" => {
            ()
        },
        ElementName => {
            Print::printBuf(literal!("\n<"))?;
            Print::printBuf(transformModelicaIdentifierToXMLElementTag(ElementName.clone())?)?;
            Print::printBuf(literal!("/>"))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpDimension(mut inDimension: &metamodelica::Ref<DAE::Dimension>) -> Result<()> {
    let () = (match &**inDimension {
        DAE::Dimension::DIM_INTEGER { integer: i } => {
            Print::printBuf(intString(i.clone()))?;
            ()
        }
        DAE::Dimension::DIM_ENUM {
            enumTypeName: _,
            literals: _,
            size: _,
        } => {
            Print::printBuf(literal!("Dim Enum"))?;
            ()
        }
        DAE::Dimension::DIM_EXP { exp: e1 } => {
            Print::printBuf(printExpStr(e1.clone())?)?;
            ()
        }
        DAE::Dimension::DIM_UNKNOWN { .. } => {
            Print::printBuf(literal!(":"))?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn dumpTypeStr(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inType {
        DAE::Type::T_INTEGER { .. } => {
            arcstr::literal!(VARTYPE_INTEGER)
        }
        DAE::Type::T_REAL { .. } => {
            arcstr::literal!(VARTYPE_REAL)
        }
        DAE::Type::T_BOOL { .. } => {
            arcstr::literal!(VARTYPE_BOOLEAN)
        }
        DAE::Type::T_STRING { .. } => {
            arcstr::literal!(VARTYPE_STRING)
        }
        DAE::Type::T_ENUMERATION { names: l, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut r#str: ArcStr;
            s1 = stringDelimitList(l.clone(), literal!(", "));
            s2 = stringAppend(arcstr::literal!(VARTYPE_ENUMERATION), stringAppend(literal!("("), s1));
            r#str = stringAppend(s2, literal!(")"));
            r#str
        }
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ },
            ..
        } => {
            arcstr::literal!(VARTYPE_EXTERNALOBJECT)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn dumpVariable(
    mut varno: ArcStr,
    mut cr: ArcStr,
    mut kind: ArcStr,
    mut dir: ArcStr,
    mut var_type: ArcStr,
    mut indx: ArcStr,
    mut derName: ArcStr,
    mut varFixed: ArcStr,
    mut flowPrefix: ArcStr,
    mut streamPrefix: ArcStr,
    mut comment: ArcStr,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = comment.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    Print::printBuf(literal!("\n<"))?;
                    Print::printBuf(arcstr::literal!(VARIABLE))?;
                    Print::printBuf(literal!(" "))?;
                    Print::printBuf(arcstr::literal!(VAR_ID))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(varno.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_NAME))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(cr.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_VARIABILITY))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(kind.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_DIRECTION))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(dir.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_TYPE))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(var_type.clone())?;
                    printIndexAndDerName(indx.clone(), derName.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_FIXED))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(varFixed.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_FLOW))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(flowPrefix.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_STREAM))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(streamPrefix.clone())?;
                    Print::printBuf(literal!("\">"))?;
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
                    Print::printBuf(literal!("\n<"))?;
                    Print::printBuf(arcstr::literal!(VARIABLE))?;
                    Print::printBuf(literal!(" "))?;
                    Print::printBuf(arcstr::literal!(VAR_ID))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(varno.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_NAME))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(cr.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_VARIABILITY))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(kind.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_DIRECTION))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(dir.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_TYPE))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(var_type.clone())?;
                    printIndexAndDerName(indx.clone(), derName.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_FIXED))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(varFixed.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_FLOW))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(flowPrefix.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_STREAM))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(streamPrefix.clone())?;
                    Print::printBuf(literal!("\" "))?;
                    Print::printBuf(arcstr::literal!(VAR_COMMENT))?;
                    Print::printBuf(literal!("=\""))?;
                    Print::printBuf(Util::xmlEscape(comment.clone())?)?;
                    Print::printBuf(literal!("\">"))?;
                    Ok(())
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

fn printIndexAndDerName(mut indx: ArcStr, mut derName: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((indx.clone(), derName.clone())) {
        (Deref @ "", Deref @ "") => (),
        (_, Deref @ "") => {
            Print::printBuf(literal!("\" "))?;
            Print::printBuf(arcstr::literal!(VAR_INDEX))?;
            Print::printBuf(literal!("=\""))?;
            Print::printBuf(indx)?;
            ()
        },
        (Deref @ "", _) => {
            Print::printBuf(literal!("\" "))?;
            Print::printBuf(arcstr::literal!(VAR_DERNAME))?;
            Print::printBuf(literal!("=\""))?;
            Print::printBuf(derName)?;
            ()
        },
        (_, _) => {
            Print::printBuf(literal!("\" "))?;
            Print::printBuf(arcstr::literal!(VAR_INDEX))?;
            Print::printBuf(literal!("=\""))?;
            Print::printBuf(indx)?;
            Print::printBuf(literal!("\" "))?;
            Print::printBuf(arcstr::literal!(VAR_DERNAME))?;
            Print::printBuf(literal!("=\""))?;
            Print::printBuf(derName)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpVarsAdditionalInfo(
    mut crefIdxLstArr: metamodelica::Array<metamodelica::List<BackendDAE::CrefIndex>>,
    mut i: i32,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = i;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if !(({
                let __elt = (*metamodelica::index_checked(&crefIdxLstArr.borrow(), 1)?).clone();
                __elt
            })
            .is_empty())
            {
                return Err("guard");
            }
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            dumpStrOpenTag(arcstr::literal!(ADDITIONAL_INFO))?;
            dumpCrefIdxLstArr(crefIdxLstArr.clone(), arcstr::literal!(HASH_TB_CREFS_LIST), i)?;
            dumpStrCloseTag(arcstr::literal!(ADDITIONAL_INFO))?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.dumpVarsAdditionalInfo - Unknown info");
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg.clone()])?;
            Ok(())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn dumpVars(
    mut vars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut crefIdxLstArr: metamodelica::Array<metamodelica::List<BackendDAE::CrefIndex>>,
    mut Content: ArcStr,
    mut addMathMLCode: bool,
) -> Result<()> {
    let __ab_crefIdxLstArr = crefIdxLstArr.borrow();
    let () = 'mc: {
        let __mc_input = (&**vars, addMathMLCode);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, addMMLCode) => {
                    if !((!(((*metamodelica::index_checked(&__ab_crefIdxLstArr, 1)?)).is_empty()))) { return Err("guard") }
                    let mut len: i32;
                    len = ((vars).len() as i32);
                    dumpStrOpenTagAttr(Content.clone(), arcstr::literal!(DIMENSION), intString(len))?;
                    dumpStrOpenTag(stringAppend(arcstr::literal!(VARIABLES), arcstr::literal!(LIST_)))?;
                    dumpVars2(vars, 1, addMMLCode.clone())?;
                    dumpStrCloseTag(stringAppend(arcstr::literal!(VARIABLES), arcstr::literal!(LIST_)))?;
                    dumpStrCloseTag(Content.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, addMMLCode) => {
                    let mut len: i32;
                    len = ((vars).len() as i32);
                    dumpStrOpenTagAttr(Content.clone(), arcstr::literal!(DIMENSION), intString(len))?;
                    dumpStrOpenTag(stringAppend(arcstr::literal!(VARIABLES), arcstr::literal!(LIST_)))?;
                    dumpVars2(vars, 1, addMMLCode.clone())?;
                    dumpStrCloseTag(stringAppend(arcstr::literal!(VARIABLES), arcstr::literal!(LIST_)))?;
                    dumpStrCloseTag(Content.clone())?;
                    Ok(())
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

fn getIndex(mut kind: &BackendDAE::VarKind) -> ArcStr {
    let mut diffIndex: ArcStr;
    diffIndex = (match kind.clone() {
        BackendDAE::VarKind::STATE { index: mut di, .. } => intString(di.clone()),
        _ => {
            literal!("")
        }
    });
    diffIndex
}

fn getDerName(mut kind: &BackendDAE::VarKind) -> Result<ArcStr> {
    let mut derName: ArcStr;
    derName = (::match_deref::match_deref! { match &(kind) {
        BackendDAE::VarKind::STATE { derName: Some(cr), .. } => {
            let mut dn: ArcStr;
            dn = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            dn
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(derName)
}

fn dumpVars2(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inInteger: i32,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inVarLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ BackendDAE::Var { .. }, tail: xs } if (BackendVariable::isParam(metamodelica::AsArg::as_arg(&v)) && Types::isArray(&v.varType)) => {
            let mut varno = inInteger;
            let mut addMMLCode = addMathMLCode;
            let mut scalarVar: metamodelica::Ref<BackendDAE::Var>;
            let mut scalar_crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            scalar_crefs = ComponentReference::expandCref(&v.varName, false)?;
            for mut cref in &*scalar_crefs {
                scalarVar = BackendVariable::copyVarNewName(cref.clone(), v.clone());
                assign_field!(scalarVar.varType = ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cref))?);
                dumpVars2(&(list![scalarVar]), varno, addMMLCode)?;
                varno = varno + 1;
            }
            dumpVars2(xs, varno, addMMLCode)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ BackendDAE::Var { varName: cr, varKind: kind, varDirection: dir, varType: var_type, bindExp: e, source, values: dae_var_attr, comment, connectorType: ct, .. }, tail: xs } => {
            let mut varno = inInteger;
            let mut addMMLCode = addMathMLCode;
            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut var_1: i32;
            dumpVariable(intString(varno), ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?, dumpKind(metamodelica::AsArg::as_arg(&kind))?, dumpDirectionStr(dir.clone())?, dumpTypeStr(metamodelica::AsArg::as_arg(&var_type))?, getIndex(metamodelica::AsArg::as_arg(&kind)), getDerName(metamodelica::AsArg::as_arg(&kind))?, boolString(BackendVariable::varFixed(metamodelica::AsArg::as_arg(&v))), dumpFlowStr(metamodelica::AsArg::as_arg(&ct)), dumpStreamStr(metamodelica::AsArg::as_arg(&ct)), unparseCommentOptionNoAnnotation(comment.clone()))?;
            dumpBindExpression(e.clone(), addMMLCode)?;
            paths = ElementSource::getElementSourceTypes(metamodelica::AsArg::as_arg(&source));
            dumpAbsynPathLst(&paths, stringAppend(arcstr::literal!(CLASSES), arcstr::literal!(NAMES_)))?;
            dumpDAEVariableAttributes(dae_var_attr.clone(), arcstr::literal!(VAR_ATTRIBUTES_VALUES), addMMLCode)?;
            dumpStrCloseTag(arcstr::literal!(VARIABLE))?;
            var_1 = varno + 1;
            dumpVars2(xs, var_1, addMMLCode)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpVarsAdds2(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut crefIdxLstArr: metamodelica::Array<metamodelica::List<BackendDAE::CrefIndex>>,
    mut inInteger: i32,
    mut addMMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inVarLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ BackendDAE::Var { varName: cr, varKind: kind, varDirection: dir, varType: var_type, bindExp: e, source, values: dae_var_attr, comment, connectorType: ct, .. }, tail: xs } => {
            let mut varno = inInteger;
            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut var_1: i32;
            dumpVariable(intString(varno), ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?, dumpKind(metamodelica::AsArg::as_arg(&kind))?, dumpDirectionStr(dir.clone())?, dumpTypeStr(metamodelica::AsArg::as_arg(&var_type))?, getIndex(metamodelica::AsArg::as_arg(&kind)), getDerName(metamodelica::AsArg::as_arg(&kind))?, boolString(BackendVariable::varFixed(metamodelica::AsArg::as_arg(&v))), dumpFlowStr(metamodelica::AsArg::as_arg(&ct)), dumpStreamStr(metamodelica::AsArg::as_arg(&ct)), DAEDumpTypes::dumpCommentAnnotationStr(comment.clone()))?;
            dumpBindExpression(e.clone(), addMMLCode)?;
            paths = ElementSource::getElementSourceTypes(metamodelica::AsArg::as_arg(&source));
            dumpAbsynPathLst(&paths, stringAppend(arcstr::literal!(CLASSES), arcstr::literal!(NAMES_)))?;
            dumpDAEVariableAttributes(dae_var_attr.clone(), arcstr::literal!(VAR_ATTRIBUTES_VALUES), addMMLCode)?;
            dumpVarsAdditionalInfo(crefIdxLstArr.clone(), varno)?;
            dumpStrCloseTag(arcstr::literal!(VARIABLE))?;
            var_1 = varno + 1;
            dumpVarsAdds2(xs, crefIdxLstArr.clone(), var_1, addMMLCode)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
            let mut varno = inInteger;
            let mut var_1: i32;
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.dumpVarsAdds2 - Unknown var: ");
            error_msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg); __mm_s.push_str(&*intString(varno)); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            var_1 = varno + 1;
            dumpVarsAdds2(xs, crefIdxLstArr.clone(), var_1, addMMLCode)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpWhenOperators(
    mut inWhenOperators: metamodelica::List<BackendDAE::WhenOperator>,
    mut inContent: ArcStr,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inWhenOperators) {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        lst => {
            let mut len: i32;
            len = ((lst).len() as i32);
            dumpStrOpenTagAttr(inContent.clone(), arcstr::literal!(DIMENSION), intString(len))?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&lst), addMathMLCode)?;
            dumpStrCloseTag(inContent)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpWhenOperatorLst(
    mut inWhenOperators: &metamodelica::List<BackendDAE::WhenOperator>,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inWhenOperators.clone(), addMathMLCode)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left, right: value, source: _ }, tail: lst }, true) => {
            let mut r#str: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpStr(left.clone())?;
            s2 = printExpStr(value.clone())?;
            r#str = stringAppendList(list![s1.clone(), literal!(" := "), s2]);
            dumpStrOpenTag(arcstr::literal!(WHEN_OPERATOR))?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(r#str)?;
            dumpStrCloseTag(arcstr::literal!(WHEN_OPERATOR))?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpStrMathMLVariable(s1)?;
            dumpExp2(metamodelica::AsArg::as_arg(&value))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&lst), addMathMLCode)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left, right: value, source: _ }, tail: lst }, false) => {
            let mut r#str: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpStr(left.clone())?;
            s2 = printExpStr(value.clone())?;
            r#str = stringAppendList(list![s1, literal!(" := "), s2]);
            dumpStrOpenTag(arcstr::literal!(WHEN_OPERATOR))?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(r#str)?;
            dumpStrCloseTag(arcstr::literal!(WHEN_OPERATOR))?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&lst), addMathMLCode)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::REINIT { stateVar, value, source: _ }, tail: lst }, _) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut r#str: ArcStr;
            e = Expression::makeCrefExp(stateVar.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            call = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: arcstr::literal!(REINIT) }), expLst: list![e, value.clone()], attr: DAE::callAttrBuiltinOther().clone() });
            r#str = printExpStr(call.clone())?;
            dumpStrOpenTag(arcstr::literal!(WHEN_OPERATOR))?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(r#str)?;
            dumpExp(call, addMathMLCode);
            dumpStrCloseTag(arcstr::literal!(WHEN_OPERATOR))?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&lst), addMathMLCode)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSERT { condition: cond, message: msg, level, source: _ }, tail: lst }, _) => {
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut r#str: ArcStr;
            call = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: arcstr::literal!(ASSERT) }), expLst: list![cond.clone(), msg.clone(), level.clone()], attr: DAE::callAttrBuiltinOther().clone() });
            r#str = printExpStr(call.clone())?;
            dumpStrOpenTag(arcstr::literal!(WHEN_OPERATOR))?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(r#str)?;
            dumpExp(call, addMathMLCode);
            dumpStrCloseTag(arcstr::literal!(WHEN_OPERATOR))?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&lst), addMathMLCode)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::TERMINATE { message: msg, source: _ }, tail: lst }, _) => {
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut r#str: ArcStr;
            call = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: arcstr::literal!(TERMINATE) }), expLst: list![msg.clone()], attr: DAE::callAttrBuiltinOther().clone() });
            r#str = printExpStr(call.clone())?;
            dumpStrOpenTag(arcstr::literal!(WHEN_OPERATOR))?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(r#str)?;
            dumpExp(call, addMathMLCode);
            dumpStrCloseTag(arcstr::literal!(WHEN_OPERATOR))?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&lst), addMathMLCode)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: call, .. }, tail: lst }, _) => {
            let mut r#str: ArcStr;
            r#str = printExpStr(call.clone())?;
            dumpStrOpenTag(arcstr::literal!(WHEN_OPERATOR))?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(r#str)?;
            dumpExp(call.clone(), addMathMLCode);
            dumpStrCloseTag(arcstr::literal!(WHEN_OPERATOR))?;
            dumpWhenOperatorLst(metamodelica::AsArg::as_arg(&lst), addMathMLCode)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn dumpTimeEvents(
    mut inTimeEvents: &metamodelica::List<BackendDAE::TimeEvent>,
    mut inContent: ArcStr,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inTimeEvents {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => {
            let mut len: i32;
            len = ((inTimeEvents).len() as i32);
            dumpStrOpenTagAttr(inContent.clone(), arcstr::literal!(DIMENSION), intString(len))?;
            dumpSampleLst(inTimeEvents, addMathMLCode)?;
            dumpStrCloseTag(inContent)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpSampleLst(mut inSamples: &metamodelica::List<BackendDAE::TimeEvent>, mut addMathMLCode: bool) -> Result<()> {
    let () = (::match_deref::match_deref! { match inSamples {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: BackendDAE::TimeEvent::SIMPLE_TIME_EVENT { .. }, tail: lst } => {
            let mut addMMLCode = addMathMLCode;
            dumpSampleLst(lst, addMMLCode)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { index: i, startExp: e1, intervalExp: e2, .. }, tail: lst } => {
            let mut addMMLCode = addMathMLCode;
            dumpStrOpenTag(stringAppend(arcstr::literal!(SAMPLES), arcstr::literal!(ELEMENT_)))?;
            dumpStrOpenTagAttr(arcstr::literal!(INDEX), arcstr::literal!(VALUE), intString(i.clone()))?;
            dumpExp(e1.clone(), addMMLCode);
            dumpStrCloseTag(arcstr::literal!(INDEX))?;
            dumpStrOpenTagAttr(arcstr::literal!(START), arcstr::literal!(EXP_STRING), printExpStr(e1.clone())?)?;
            dumpExp(e1.clone(), addMMLCode);
            dumpStrCloseTag(arcstr::literal!(START))?;
            dumpStrOpenTagAttr(arcstr::literal!(INTERVAL), arcstr::literal!(EXP_STRING), printExpStr(e2.clone())?)?;
            dumpExp(e2.clone(), addMMLCode);
            dumpStrCloseTag(arcstr::literal!(INTERVAL))?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(SAMPLES), arcstr::literal!(ELEMENT_)))?;
            dumpSampleLst(lst, addMMLCode)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn dumpZeroCrossing(
    mut zeroCross: &metamodelica::List<BackendDAE::ZeroCrossing>,
    mut inContent: ArcStr,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match zeroCross {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => {
            let mut len: i32;
            len = ((zeroCross).len() as i32);
            dumpStrOpenTagAttr(inContent.clone(), arcstr::literal!(DIMENSION), intString(len))?;
            dumpZcLst(zeroCross, addMathMLCode)?;
            dumpStrCloseTag(inContent)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpZcLst(
    mut inZeroCrossingLst: &metamodelica::List<BackendDAE::ZeroCrossing>,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inZeroCrossingLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: BackendDAE::ZeroCrossing { relation_: e, occurEquLst: eq, .. }, tail: zcLst } => {
            let mut addMMLCode = addMathMLCode;
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(ZERO_CROSSING), arcstr::literal!(ELEMENT_)), arcstr::literal!(EXP_STRING), printExpStr(e.clone())?)?;
            dumpExp(e.clone(), addMMLCode);
            dumpLstIntAttr(eq.clone(), stringAppend(arcstr::literal!(INVOLVED), arcstr::literal!(EQUATIONS_)), stringAppend(arcstr::literal!(EQUATION), arcstr::literal!(ID_)))?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(ZERO_CROSSING), arcstr::literal!(ELEMENT_)))?;
            dumpZcLst(zcLst, addMMLCode)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn lbinopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::AND { .. } => {
            arcstr::literal!(MathMLAnd)
        }
        DAE::Operator::OR { .. } => {
            arcstr::literal!(MathMLOr)
        }
        _ => {
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.lbinopSymbol - Unknown operator");
            error_msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*error_msg);
                __mm_s.push_str(&*ExpressionDump::debugBinopSymbol(inOperator)?);
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn lunaryopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::NOT { .. } => {
            arcstr::literal!(MathMLNot)
        }
        _ => {
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.lunaryopSymbol - Unknown operator");
            error_msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*error_msg);
                __mm_s.push_str(&*ExpressionDump::debugBinopSymbol(inOperator)?);
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn relopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::LESS { .. } => {
            arcstr::literal!(MathMLLessThan)
        }
        DAE::Operator::LESSEQ { .. } => {
            arcstr::literal!(MathMLLessEqualThan)
        }
        DAE::Operator::GREATER { .. } => {
            arcstr::literal!(MathMLGreaterThan)
        }
        DAE::Operator::GREATEREQ { .. } => {
            arcstr::literal!(MathMLGreaterEqualThan)
        }
        DAE::Operator::EQUAL { .. } => {
            arcstr::literal!(MathMLEquivalent)
        }
        DAE::Operator::NEQUAL { .. } => {
            arcstr::literal!(MathMLNotEqual)
        }
        _ => {
            let mut error_msg: ArcStr;
            error_msg = literal!("in XMLDump.relopSymbol - Unknown operator");
            error_msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*error_msg);
                __mm_s.push_str(&*ExpressionDump::debugBinopSymbol(inOperator)?);
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn dumpResidual(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut inIndexNumber: ArcStr,
    mut addMathMLCode: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((&**inEquation, addMathMLCode)) {
        (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, true) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" ( "), s2, literal!(") = 0")]);
            dumpStrOpenTagAttr(arcstr::literal!(EQUATION), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrOpenTag(arcstr::literal!(MathMLMinus))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpExp2(&(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(arcstr::literal!(EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" - ( "), s2, literal!(" ) = 0")]);
            dumpStrOpenTagAttr(arcstr::literal!(EQUATION), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(arcstr::literal!(EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }, true) => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            s = stringAppendList(list![s1, literal!(" - ("), s2, literal!(") = 0\n")]);
            dumpStrOpenTagAttr(arcstr::literal!(ARRAY_EQUATION), arcstr::literal!(EXP_STRING), s)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrOpenTag(arcstr::literal!(MathMLMinus))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpExp2(&(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(arcstr::literal!(ARRAY_EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" - ( "), s2, literal!(" ) = 0")]);
            dumpStrOpenTagAttr(arcstr::literal!(ARRAY_OF_EQUATIONS), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(arcstr::literal!(ARRAY_OF_EQUATIONS))?;
            ()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. }, true) => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            s = stringAppendList(list![s1, literal!(" - ("), s2, literal!(") = 0\n")]);
            dumpStrOpenTagAttr(arcstr::literal!(COMPLEX_EQUATION), arcstr::literal!(EXP_STRING), s)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrOpenTag(arcstr::literal!(MathMLMinus))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e1))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpExp2(&(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(arcstr::literal!(COMPLEX_EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e1.clone())?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" - ( "), s2, literal!(" ) = 0")]);
            dumpStrOpenTagAttr(arcstr::literal!(COMPLEX_EQUATION), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(arcstr::literal!(COMPLEX_EQUATION))?;
            ()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. }, true) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1.clone(), literal!(" - ( "), s2, literal!(" ) := 0")]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(SOLVED), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrOpenTag(arcstr::literal!(MathMLMinus))?;
            Print::printBuf(s1)?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpExp2(&(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(SOLVED), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            s2 = printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" - ("), s2, literal!(") := 0")]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(SOLVED), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(SOLVED), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition: e1, whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e, right: e2, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, true) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            let mut is: ArcStr;
            s1 = printExpStr(e.clone())?;
            s2 = printExpStr(e2.clone())?;
            is = printExpStr(e1.clone())?;
            res = stringAppendList(list![s1.clone(), literal!(" - ("), s2, literal!(") := 0")]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrOpenTag(arcstr::literal!(MathMLMinus))?;
            Print::printBuf(s1)?;
            dumpExp2(metamodelica::AsArg::as_arg(&e2))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpExp2(&(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrOpenTag(stringAppend(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(CONDITION)))?;
            Print::printBuf(literal!("\n"))?;
            Print::printBuf(is)?;
            dumpExp(e1.clone(), true);
            dumpStrCloseTag(stringAppend(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(CONDITION)))?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition: e1, whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e, right: e2, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            let mut is: ArcStr;
            s1 = printExpStr(e.clone())?;
            s2 = printExpStr(e2.clone())?;
            is = printExpStr(e1.clone())?;
            res = stringAppendList(list![s1, literal!(" - ("), s2, literal!(") := 0")]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrTagContent(stringAppend(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)), arcstr::literal!(CONDITION)), is)?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(WHEN), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, true) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e.clone())?;
            res = stringAppendList(list![s1, literal!(" = 0")]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(RESIDUAL), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrOpenTag(arcstr::literal!(MathML))?;
            dumpStrOpenTagAttr(arcstr::literal!(MATH), arcstr::literal!(MathMLXmlns), arcstr::literal!(MathMLWeb))?;
            dumpStrOpenTag(arcstr::literal!(MathMLApply))?;
            dumpStrVoidTag(arcstr::literal!(MathMLEquivalent))?;
            dumpExp2(metamodelica::AsArg::as_arg(&e))?;
            dumpStrMathMLNumber(literal!("0"))?;
            dumpStrCloseTag(arcstr::literal!(MathMLApply))?;
            dumpStrCloseTag(arcstr::literal!(MATH))?;
            dumpStrCloseTag(arcstr::literal!(MathML))?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(RESIDUAL), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, false) => {
            let mut indexS = inIndexNumber;
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = printExpStr(e.clone())?;
            res = stringAppendList(list![s1, literal!(" = 0")]);
            dumpStrOpenTagAttr(stringAppend(arcstr::literal!(RESIDUAL), arcstr::literal!(EQUATION_)), arcstr::literal!(ID), indexS)?;
            Print::printBuf(res)?;
            dumpStrCloseTag(stringAppend(arcstr::literal!(RESIDUAL), arcstr::literal!(EQUATION_)))?;
            ()
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: stmts }, source, .. }, _) => {
            let mut indexS = inIndexNumber;
            dumpStrOpenTagAttr(arcstr::literal!(ALGORITHM), arcstr::literal!(ID), indexS)?;
            Print::printBuf(Util::xmlEscape(DAEDump::dumpAlgorithmsStr(&(list![metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts.clone() }), source: source.clone() })]))?)?)?;
            dumpStrCloseTag(arcstr::literal!(ALGORITHM))?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn unaryopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::UMINUS { .. } => arcstr::literal!(MathMLMinus),
        DAE::Operator::UMINUS_ARR { .. } => arcstr::literal!(MathMLMinus),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn unparseCommentOptionNoAnnotation(mut inAbsynCommentOption: Option<metamodelica::Ref<SCode::Comment>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inAbsynCommentOption) {
        Some(Deref @ SCode::Comment { annotation_: _, comment: Some(cmt) }) => {
            let mut r#str: ArcStr;
            r#str = cmt.clone();
            r#str
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}
