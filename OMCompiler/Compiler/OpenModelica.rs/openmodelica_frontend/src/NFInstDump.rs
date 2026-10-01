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

use crate::NFInstDumpTpl;
use crate::NFInstTypes;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_tpl::Tpl;
use openmodelica_util_datatypes_basic::Array;

//public import NFConnect2;
pub(crate) fn modelStr(mut inName: ArcStr, mut inClass: metamodelica::Ref<NFInstTypes::Class>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: ArcStr, __a2: metamodelica::Ref<NFInstTypes::Class>| {
                NFInstDumpTpl::dumpModel(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, ArcStr, metamodelica::Ref<NFInstTypes::Class>) -> Result<Tpl::Text>
                    + 'static,
            >),
        inName,
        inClass,
    )?;
    Ok(outString)
}

pub(crate) fn elementStr(mut inElement: metamodelica::Ref<NFInstTypes::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<NFInstTypes::Element>| {
            NFInstDumpTpl::dumpElement(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<NFInstTypes::Element>) -> Result<Tpl::Text> + 'static,
            >),
        inElement,
    )?;
    Ok(outString)
}

pub(crate) fn componentStr(mut inComponent: metamodelica::Ref<NFInstTypes::Component>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<NFInstTypes::Component>| {
                NFInstDumpTpl::dumpComponent(__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<NFInstTypes::Component>) -> Result<Tpl::Text> + 'static,
            >),
        inComponent,
    )?;
    Ok(outString)
}

pub(crate) fn bindingStr(mut inBinding: NFInstTypes::Binding) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: NFInstTypes::Binding| NFInstDumpTpl::dumpBinding(__a0, &__a1))
            as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, NFInstTypes::Binding) -> Result<Tpl::Text> + 'static>),
        inBinding,
    )?;
    Ok(outString)
}

pub(crate) fn prefixStr(mut inPrefix: metamodelica::Ref<NFInstPrefix::Prefix>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<NFInstPrefix::Prefix>| {
            NFInstDumpTpl::dumpPrefix(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<NFInstPrefix::Prefix>) -> Result<Tpl::Text> + 'static,
            >),
        inPrefix,
    )?;
    Ok(outString)
}

pub(crate) fn equationStr(mut inEquation: metamodelica::Ref<NFInstTypes::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<NFInstTypes::Equation>| {
            NFInstDumpTpl::dumpEquation(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<NFInstTypes::Equation>) -> Result<Tpl::Text> + 'static,
            >),
        inEquation,
    )?;
    Ok(outString)
}

//public function connectionsStr
//  input NFConnect2.Connections inConnections;
//  output String outString;
//algorithm
//  outString := Tpl.tplString(NFInstDumpTpl.dumpConnections, inConnections);
//end connectionsStr;
pub(crate) fn dimensionStr(mut inDimension: NFInstTypes::Dimension) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: NFInstTypes::Dimension| {
            NFInstDumpTpl::dumpDimension(__a0, &__a1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, NFInstTypes::Dimension) -> Result<Tpl::Text> + 'static>),
        inDimension,
    )?;
    Ok(outString)
}

pub(crate) fn dumpUntypedComponentDims(mut inComponent: &metamodelica::Ref<NFInstTypes::Component>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inComponent {
        NFInstTypes::Component::UNTYPED_COMPONENT { dimensions: dims, .. } => {
            let mut dims_str: ArcStr;
            dims_str = Array::toString(
                dims.clone(),
                &dimensionStr,
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                false,
                0,
            )?;
            dims_str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}
