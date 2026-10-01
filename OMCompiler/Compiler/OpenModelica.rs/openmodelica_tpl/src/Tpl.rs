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

use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

#[path = "Tpl.handwritten.rs"]
mod handwritten;
pub use handwritten::{
    Text, closeFile, emptyTxt, getIteri_i0, isEmpty, newLine, nextIter, popBlock, popIter, pushBlock, pushIter,
    redirectToFile, softNewLine, strTokString, strTokText, stringText, textStrTok, textStringBuf, writeStr, writeText,
    writeTok,
};

// indentation will be implemented through spaces
// where tabs will be converted where 1 tab = 4 spaces ??
pub type Tokens = metamodelica::List<metamodelica::Ref<StringToken>>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct BlockTypeFileText {
    /// The block type
    pub bt: metamodelica::Ref<BlockType>,
    pub nchars: i32,
    pub aind: i32,
    pub isstart: bool,
    /// Usage depends on bt; stores the last file position to know if it is empty or not.
    pub tell: Mutable::Mutable<i32>,
    pub septok: Mutable::Mutable<Option<metamodelica::Ref<StringToken>>>,
}

impl metamodelica::gc::MMTrace for BlockTypeFileText {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.bt, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nchars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.aind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isstart, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tell, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.septok, __mmv)?;
        Ok(())
    }
}
pub type BT_FILE_TEXT = BlockTypeFileText;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum StringToken {
    /// Always outputs the new-line char.
    ST_NEW_LINE,
    /// A string without new-lines in it.
    ST_STRING { value: ArcStr },
    /// A (non-empty) string with new-line at the end.
    ST_LINE { line: ArcStr },
    /// Every string in the list can have a new-line at its end (but does not have to).
    ST_STRING_LIST {
        strList: metamodelica::List<ArcStr>,
        /// True when the last string in the list has new-line at the end.
        lastHasNewLine: bool,
    },
    ST_BLOCK {
        tokens: Tokens,
        blockType: metamodelica::Ref<BlockType>,
    },
}
impl metamodelica::gc::MMTrace for StringToken {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            StringToken::ST_NEW_LINE => Ok(()),
            StringToken::ST_STRING { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            StringToken::ST_LINE { line } => {
                metamodelica::gc::MMTrace::mm_accept(line, __mmv)?;
                Ok(())
            }
            StringToken::ST_STRING_LIST {
                strList,
                lastHasNewLine,
            } => {
                metamodelica::gc::MMTrace::mm_accept(strList, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lastHasNewLine, __mmv)?;
                Ok(())
            }
            StringToken::ST_BLOCK { tokens, blockType } => {
                metamodelica::gc::MMTrace::mm_accept(tokens, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(blockType, __mmv)?;
                Ok(())
            }
        }
    }
}
impl StringToken {
    pub fn interned_ST_NEW_LINE() -> metamodelica::Ref<StringToken> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<StringToken>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(StringToken::ST_NEW_LINE));
        (*INTERNED).clone()
    }
}
pub fn interned_ST_NEW_LINE() -> metamodelica::Ref<StringToken> {
    StringToken::interned_ST_NEW_LINE()
}
impl Default for StringToken {
    fn default() -> Self {
        Self::ST_NEW_LINE
    }
}
pub use self::StringToken::{ST_BLOCK, ST_LINE, ST_NEW_LINE, ST_STRING, ST_STRING_LIST};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum BlockType {
    BT_TEXT,
    BT_INDENT {
        width: i32,
    },
    BT_ABS_INDENT {
        width: i32,
    },
    BT_REL_INDENT {
        offset: i32,
    },
    BT_ANCHOR {
        offset: i32,
    },
    /// Iteration items block, every token in the block is an item.
    ///                index0 is the active index during the build phase, then it is the last one + 1.
    BT_ITER {
        options: metamodelica::Ref<IterOptions>,
        index0: Mutable::Mutable<i32>,
    },
}
impl metamodelica::gc::MMTrace for BlockType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            BlockType::BT_TEXT => Ok(()),
            BlockType::BT_INDENT { width } => {
                metamodelica::gc::MMTrace::mm_accept(width, __mmv)?;
                Ok(())
            }
            BlockType::BT_ABS_INDENT { width } => {
                metamodelica::gc::MMTrace::mm_accept(width, __mmv)?;
                Ok(())
            }
            BlockType::BT_REL_INDENT { offset } => {
                metamodelica::gc::MMTrace::mm_accept(offset, __mmv)?;
                Ok(())
            }
            BlockType::BT_ANCHOR { offset } => {
                metamodelica::gc::MMTrace::mm_accept(offset, __mmv)?;
                Ok(())
            }
            BlockType::BT_ITER { options, index0 } => {
                metamodelica::gc::MMTrace::mm_accept(options, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index0, __mmv)?;
                Ok(())
            }
        }
    }
}
impl BlockType {
    pub fn interned_BT_TEXT() -> metamodelica::Ref<BlockType> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<BlockType>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(BlockType::BT_TEXT));
        (*INTERNED).clone()
    }
}
pub fn interned_BT_TEXT() -> metamodelica::Ref<BlockType> {
    BlockType::interned_BT_TEXT()
}
impl Default for BlockType {
    fn default() -> Self {
        Self::BT_TEXT
    }
}
pub use self::BlockType::{BT_ABS_INDENT, BT_ANCHOR, BT_INDENT, BT_ITER, BT_REL_INDENT, BT_TEXT};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct IterOptions {
    pub startIndex0: i32,
    pub empty: Option<metamodelica::Ref<StringToken>>,
    pub separator: Option<metamodelica::Ref<StringToken>>,
    /// Number of items to be aligned by. When 0, no alignment.
    pub alignNum: i32,
    pub alignOfset: i32,
    pub alignSeparator: metamodelica::Ref<StringToken>,
    /// Number of chars on a line, after that the wrapping can occur. When 0, no wrapping.
    pub wrapWidth: i32,
    pub wrapSeparator: metamodelica::Ref<StringToken>,
}

impl metamodelica::gc::MMTrace for IterOptions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.startIndex0, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.empty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.separator, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.alignNum, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.alignOfset, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.alignSeparator, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.wrapWidth, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.wrapSeparator, __mmv)?;
        Ok(())
    }
}
pub type ITER_OPTIONS = IterOptions;

//by default, we will parse new lines in every non-token string
pub fn textString(mut inText: Text) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inText {
        mut txt => {
            let mut r#str: ArcStr;
            let mut handle: i32;
            handle = Print::saveAndClearBuf()?;
            textStringBuf(txt)?;
            r#str = Print::getString()?;
            Print::restoreBuf(handle)?;
            r#str
        }
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("-!!!Tpl.textString failed.\n"))?;
            return Err("fail");
        }
    });
    Ok(outString)
}

pub fn failIfTrue(mut istrue: bool) -> Result<()> {
    if istrue {
        return Err("fail");
    }
    Ok(())
}

fn tplCallHandleErrors(
    mut inFun: Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>,
    mut txt: Text,
) -> Result<Text> {
    pub type Tpl_Fun = std::sync::Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
        if let Ok(__iflet1) = inFun(txt.clone()) {
            txt = __iflet1;
        } else {
            addTemplateErrorFunc(inFun.clone())?;
            return Err("fail");
        }
        Ok(false)
    });
    match __cp0 {
        Ok(__returned) => {
            if __returned? {
                return Ok(txt.clone());
            }
        }
        Err(_) => {
            let _rearm = metamodelica::heap_limit::RearmOnDrop;
            if StackOverflow::hasStacktraceMessages() {
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Stack overflow when evaluating function:\n"));
                        __mm_s.push_str(&*stringDelimitList(
                            StackOverflow::readableStacktraceMessages()?,
                            literal!("\n"),
                        ));
                        ArcStr::from(__mm_s)
                    },
                    metamodelica::sourceInfo!("Template/Tpl.mo"),
                )?;
            }
            addTemplateErrorFunc(inFun.clone())?;
            return Err("fail");
        }
    }
    Ok(txt)
}

pub fn tplCallWithFailErrorNoArg(
    mut inFun: Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>,
    mut txt: Text,
) -> Result<Text> {
    pub type Tpl_Fun = std::sync::Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    txt = tplCallHandleErrors(inFun.clone(), txt)?;
    Ok(txt)
}

pub fn tplCallWithFailError<ArgType1: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>,
    mut inArg: ArgType1,
    mut txt: Text,
) -> Result<Text> {
    pub type Tpl_Fun<ArgType1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    txt = tplCallHandleErrors(
        (std::sync::Arc::new({
            let __pe_b1 = inArg;
            move |__pe_a0| inFun(__pe_a0, __pe_b1.clone())
        }) as std::sync::Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>),
        txt,
    )?;
    Ok(txt)
}

pub(crate) fn tplCallWithFailError2<
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>,
    mut inArgA: ArgType1,
    mut inArgB: ArgType2,
    mut txt: Text,
) -> Result<Text> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    txt = tplCallHandleErrors(
        (std::sync::Arc::new({
            let __pe_b1 = inArgA;
            let __pe_b2 = inArgB;
            move |__pe_a0| inFun(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        }) as std::sync::Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>),
        txt,
    )?;
    Ok(txt)
}

pub fn tplCallWithFailError3<
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType3: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>,
    mut inArgA: ArgType1,
    mut inArgB: ArgType2,
    mut inArgC: ArgType3,
    mut txt: Text,
) -> Result<Text> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    txt = tplCallHandleErrors(
        (std::sync::Arc::new({
            let __pe_b1 = inArgA;
            let __pe_b2 = inArgB;
            let __pe_b3 = inArgC;
            move |__pe_a0| inFun(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
        }) as std::sync::Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>),
        txt,
    )?;
    Ok(txt)
}

pub fn tplString<ArgType1: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>,
    mut inArg: ArgType1,
) -> Result<ArcStr> {
    pub type Tpl_Fun<ArgType1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>;

    let mut outString: ArcStr;
    let mut txt: Text;
    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError(inFun.clone(), inArg, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    outString = textString(txt)?;
    Ok(outString)
}

pub fn tplString2<
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>,
    mut inArgA: ArgType1,
    mut inArgB: ArgType2,
) -> Result<ArcStr> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>;

    let mut outString: ArcStr;
    let mut txt: Text;
    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError2(inFun.clone(), inArgA, inArgB, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    outString = textString(txt)?;
    Ok(outString)
}

pub fn tplString3<
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType3: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>,
    mut inArgA: ArgType1,
    mut inArgB: ArgType2,
    mut inArgC: ArgType3,
) -> Result<ArcStr> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>;

    let mut outString: ArcStr;
    let mut txt: Text;
    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError3(inFun.clone(), inArgA, inArgB, inArgC, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    outString = textString(txt)?;
    Ok(outString)
}

pub(crate) fn tplPrint<ArgType1: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>,
    mut inArg: ArgType1,
) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>;

    let mut txt: Text;
    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError(inFun.clone(), inArg, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    textStringBuf(txt)?;
    Ok(())
}

pub fn tplPrint2<
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>,
    mut inArgA: ArgType1,
    mut inArgB: ArgType2,
) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>;

    let mut txt: Text;
    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError2(inFun.clone(), inArgA, inArgB, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    textStringBuf(txt)?;
    Ok(())
}

pub(crate) fn tplPrint3<
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType3: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>,
    mut inArgA: ArgType1,
    mut inArgB: ArgType2,
    mut inArgC: ArgType3,
) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>;

    let mut txt: Text;
    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError3(inFun.clone(), inArgA, inArgB, inArgC, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    textStringBuf(txt)?;
    Ok(())
}

pub fn tplNoret3<
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType2: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType3: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>,
    mut inArg: ArgType1,
    mut inArg2: ArgType2,
    mut inArg3: ArgType3,
) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>;

    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    tplCallWithFailError3(inFun.clone(), inArg, inArg2, inArg3, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    Ok(())
}

pub fn tplNoret2<
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>,
    mut inArg: ArgType1,
    mut inArg2: ArgType2,
) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>;

    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    tplCallWithFailError2(inFun.clone(), inArg, inArg2, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    Ok(())
}

pub fn tplNoret<ArgType1: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>,
    mut inArg: ArgType1,
) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>;

    let mut nErr: i32;
    nErr = Error::getNumErrorMessages();
    tplCallWithFailError(inFun.clone(), inArg, emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr)?;
    Ok(())
}

pub fn textFile(mut inText: Text, mut inFileName: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inText, inFileName);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, mut file) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rtTickTxt: metamodelica::Real;
            let mut rtTickW: metamodelica::Real;
            rtTickTxt = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
            Print::clearBuf();
            textStringBuf(txt.clone())?;
            rtTickW = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
            Print::writeBuf(file.clone())?;
            if Testsuite::isRunning()? {
                System::appendFile(Testsuite::getTempFilesFile()?, {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
            Print::clearBuf();
            if Flags::isSet(Flags::TPL_PERF_TIMES.clone())? {
                Debug::trace({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("textFile "));
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!("\n    text:"));
                    __mm_s.push_str(&*realString((rtTickW) - (rtTickTxt)));
                    __mm_s.push_str(&*literal!("\n   write:"));
                    __mm_s.push_str(&*realString(
                        (System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?) - (rtTickW),
                    ));
                    ArcStr::from(__mm_s)
                })?;
            }
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("-!!!Tpl.textFile failed - a system error ?\n"))?;
            }
            Ok(())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub fn textFileConvertLines(mut inText: Text, mut inFileName: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inText, inFileName);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, mut file) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rtTickTxt: metamodelica::Real;
            let mut rtTickW: metamodelica::Real;
            rtTickTxt = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
            Print::clearBuf();
            textStringBuf(txt.clone())?;
            rtTickW = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
            System::writeFile(file.clone(), literal!(""))?;
            if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
                Print::writeBufConvertLines(System::realpath(file.clone())?)?;
            } else {
                Print::writeBuf(file.clone())?;
            }
            if Testsuite::isRunning()? {
                System::appendFile(Testsuite::getTempFilesFile()?, {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
            Print::clearBuf();
            if Flags::isSet(Flags::TPL_PERF_TIMES.clone())? {
                Debug::traceln({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("textFile "));
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!("\n    text:"));
                    __mm_s.push_str(&*realString((rtTickW) - (rtTickTxt)));
                    __mm_s.push_str(&*literal!("\n   write:"));
                    __mm_s.push_str(&*realString(
                        (System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?) - (rtTickW),
                    ));
                    ArcStr::from(__mm_s)
                })?;
            }
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("-!!!Tpl.textFile failed - a system error ?\n"))?;
            Ok(())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub fn sourceInfo(mut inFileName: ArcStr, mut inLineNum: i32, mut inColumnNum: i32) -> SourceInfo {
    let mut outSourceInfo: SourceInfo;
    outSourceInfo = SourceInfo {
        fileName: inFileName,
        isReadOnly: false,
        lineNumberStart: inLineNum,
        columnNumberStart: inColumnNum,
        lineNumberEnd: inLineNum,
        columnNumberEnd: inColumnNum,
        lastModification: metamodelica::OrderedFloat(0.0_f64),
    };
    outSourceInfo
}

//we do not import Error.addSourceMessage() directly
//because of list creation in Susan is not possible (yet by design)
pub fn addSourceTemplateError(mut inErrMsg: ArcStr, mut inInfo: &SourceInfo) -> Result<()> {
    Error::addSourceMessage(&(Error::TEMPLATE_ERROR.clone()), list![inErrMsg], inInfo)?;
    Ok(())
}

//for completeness
fn addTemplateErrorFunc<T: Clone + 'static + metamodelica::gc::MMTrace>(mut func: T) -> Result<()> {
    Error::addMessage(Error::TEMPLATE_ERROR_FUNC.clone(), list![(System::dladdr(&func)).0])?;
    Ok(())
}

pub fn addTemplateError(mut msg: ArcStr) -> Result<()> {
    Error::addMessage(Error::TEMPLATE_ERROR.clone(), list![msg])?;
    Ok(())
}

pub fn booleanString(mut b: bool) -> ArcStr {
    let mut s: ArcStr;
    s = ArcStr::from(::std::format!("{}", b));
    s
}

pub fn debugSusan() -> Result<bool> {
    let mut b: bool;
    b = Flags::isSet(Flags::SUSAN_MATCHCONTINUE_DEBUG.clone())?;
    Ok(b)
}

pub(crate) fn fakeStackOverflow() -> Result<()> {
    Error::addInternalError(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Stack overflow:\n"));
            __mm_s.push_str(&*StackOverflow::generateReadableMessage(1000, 4, literal!("\n"))?);
            ArcStr::from(__mm_s)
        },
        metamodelica::sourceInfo!("Template/Tpl.mo"),
    )?;
    StackOverflow::triggerStackOverflow()?;
    Ok(())
}
