// Auto-generated from MetaModelica source
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

use crate::Error;
use crate::System;

/*
 Template for Lexer Code
 replace keywords:
 %LexerCode
 %time
 %Token
 %Lexer
 %LexTable
 %constant
 %nameSpan
 %functions
 %caseAction
*/
pub(crate) const debug: bool = false;

pub mod LexTable {
    use super::*;
    pub(crate) const yy_limit: i32 = 46;

    pub(crate) const yy_finish: i32 = 82;

    pub(crate) static yy_acclist: std::sync::LazyLock<metamodelica::StaticArray<i32>> =
        std::sync::LazyLock::new(|| {
            metamodelica::StaticArray::new(
                list![
                    17, 16, 15, 16, 16, 13, 16, 5, 16, 14, 16, 11, 16, 12, 16, 16, 16, 16, 9, 16, 10, 16, 15, 1, 5, 2,
                    3, 4, 8, 6, 3, 7
                ]
                .into_iter()
                .cloned()
                .collect(),
            )
        });

    pub(crate) static yy_accept: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                1, 1, 1, 2, 3, 5, 6, 8, 10, 12, 14, 16, 17, 18, 19, 21, 23, 24, 24, 25, 25, 25, 26, 26, 26, 26, 26, 27,
                27, 28, 28, 29, 29, 29, 29, 29, 29, 29, 30, 31, 31, 31, 32, 33, 33, 33
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_ec: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 3,
                1, 1, 1, 1, 1, 1, 1, 1, 4, 5, 6, 7, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 10, 1, 1, 1, 1, 1, 1, 11, 11, 11,
                11, 12, 11, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 13, 14, 15, 1, 1, 1, 16, 17,
                11, 11, 18, 19, 1, 1, 1, 1, 1, 20, 1, 21, 1, 1, 1, 22, 23, 24, 25, 1, 1, 1, 1, 1, 26, 1, 27, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_meta: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                1, 1, 2, 1, 1, 1, 1, 2, 3, 1, 3, 3, 1, 2, 1, 3, 4, 3, 4, 1, 2, 2, 1, 2, 2, 1, 1
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_base: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                0, 0, 81, 82, 78, 25, 82, 22, 82, 82, 82, 63, 53, 55, 82, 82, 74, 27, 82, 50, 65, 26, 39, 53, 52, 37,
                82, 0, 37, 45, 43, 27, 27, 24, 0, 47, 19, 82, 82, 0, 27, 23, 82, 0, 82, 56, 59, 61, 63, 65, 67
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_def: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                45, 1, 45, 45, 45, 46, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 46, 45, 47, 45, 45, 45, 45, 45, 45,
                45, 48, 45, 45, 45, 45, 45, 45, 49, 45, 45, 45, 45, 50, 45, 45, 45, 51, 0, 45, 45, 45, 45, 45, 45
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_nxt: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                4, 5, 6, 4, 7, 4, 4, 4, 8, 9, 4, 4, 10, 4, 11, 4, 4, 4, 12, 4, 13, 4, 4, 14, 4, 15, 16, 19, 21, 27, 22,
                42, 21, 23, 22, 42, 43, 23, 20, 23, 20, 39, 30, 23, 30, 29, 38, 31, 36, 37, 41, 31, 41, 31, 36, 42, 18,
                18, 18, 18, 18, 34, 18, 35, 35, 40, 40, 44, 44, 18, 18, 33, 32, 29, 28, 17, 26, 25, 24, 17, 45, 3, 45,
                45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_chk: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 6, 8, 18, 8, 42, 22,
                8, 22, 41, 37, 22, 6, 8, 18, 34, 23, 22, 23, 29, 33, 23, 29, 32, 36, 31, 36, 30, 29, 36, 46, 46, 46,
                46, 47, 26, 47, 48, 48, 49, 49, 50, 50, 51, 51, 25, 24, 21, 20, 17, 14, 13, 12, 5, 3, 45, 45, 45, 45,
                45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });
}

pub(crate) fn scan(mut fileName: ArcStr) -> Result<(metamodelica::List<Token>, metamodelica::List<Token>)> {
    let mut tokens: metamodelica::List<Token>;
    let mut errorTokens: metamodelica::List<Token>;
    let mut contents: ArcStr;
    contents = System::readFile(fileName.clone())?;
    (tokens, errorTokens) = lex(fileName, contents)?;
    Ok((tokens, errorTokens))
}

pub(crate) fn scanString(
    mut fileSource: ArcStr,
    mut fileName: ArcStr,
) -> Result<(metamodelica::List<Token>, metamodelica::List<Token>)> {
    let mut tokens: metamodelica::List<Token>;
    let mut errorTokens: metamodelica::List<Token>;
    (tokens, errorTokens) = lex(fileName, fileSource)?;
    Ok((tokens, errorTokens))
}

/* grammar according to json.org */
pub(crate) fn action(
    mut act: i32,
    mut startSt: i32,
    mut mm_currSt: i32,
    mut mm_pos: i32,
    mut mm_sPos: i32,
    mut mm_ePos: i32,
    mut mm_linenr: i32,
    mut lineNrStart: i32,
    mut buffer: i32,
    mut fileNm: ArcStr,
    mut fileContents: ArcStr,
    mut inErrorTokens: metamodelica::List<Token>,
) -> Result<(Token, i32, i32, metamodelica::List<Token>)> {
    let mut token: Token;
    let mut mm_startSt: i32;
    let mut bufferRet: i32;
    let mut errorTokens: metamodelica::List<Token> = inErrorTokens;
    mm_startSt = startSt;
    bufferRet = 0;
    token = (match act {
        1 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::STRING.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        2 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::STRING.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        3 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::NUMBER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        4 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::NUMBER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        5 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::INTEGER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        6 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::TRUE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        7 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::FALSE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        8 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::NULL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        9 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::OBJECTBEGIN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        10 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::OBJECTEND.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        11 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ARRAYBEGIN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        12 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ARRAYEND.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        13 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::COMMA.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        14 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::COLON.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        15 => noToken.clone(),
        16 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::_NO_TOKEN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            errorTokens = metamodelica::cons(tok, errorTokens);
            noToken.clone()
        }
        _ => {
            let mut tok: Token;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nLexer unknown rule, action="));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", act)));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            tok = Token {
                fileName: fileNm,
                id: TokenId::_NO_TOKEN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            metamodelica::print(printToken(tok)?);
            return Err("fail");
        }
    });
    Ok((token, mm_startSt, bufferRet, errorTokens))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum TokenId {
    _NO_TOKEN = 1,
    ARRAYBEGIN = 2,
    ARRAYEND = 3,
    COLON = 4,
    COMMA = 5,
    FALSE = 6,
    INTEGER = 7,
    NULL = 8,
    NUMBER = 9,
    OBJECTBEGIN = 10,
    OBJECTEND = 11,
    STRING = 12,
    TRUE = 13,
}
impl PartialOrd for TokenId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TokenId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for TokenId {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for TokenId {
    fn default() -> Self {
        Self::_NO_TOKEN
    }
}

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Token {
    pub fileName: ArcStr,
    pub id: TokenId,
    pub fileContents: ArcStr,
    pub byteOffset: i32,
    pub length: i32,
    pub lineNumberStart: i32,
    pub columnNumberStart: i32,
    pub lineNumberEnd: i32,
    pub columnNumberEnd: i32,
}

impl metamodelica::gc::MMTrace for Token {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fileName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.id, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fileContents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.byteOffset, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.length, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineNumberStart, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.columnNumberStart, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineNumberEnd, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.columnNumberEnd, __mmv)?;
        Ok(())
    }
}
impl Default for Token {
    fn default() -> Self {
        Self {
            fileName: Default::default(),
            id: Default::default(),
            fileContents: Default::default(),
            byteOffset: Default::default(),
            length: Default::default(),
            lineNumberStart: Default::default(),
            columnNumberStart: Default::default(),
            lineNumberEnd: Default::default(),
            columnNumberEnd: Default::default(),
        }
    }
}

pub type TOKEN = Token;

pub(crate) static noToken: std::sync::LazyLock<Token> = std::sync::LazyLock::new(|| Token {
    fileName: literal!("<NoFile>"),
    id: TokenId::_NO_TOKEN.clone(),
    fileContents: literal!(""),
    byteOffset: 0,
    length: 0,
    lineNumberStart: 0,
    columnNumberStart: 0,
    lineNumberEnd: 0,
    columnNumberEnd: 0,
});

pub(crate) fn printToken(mut token: Token) -> Result<ArcStr> {
    let mut strTk: ArcStr;
    let mut id: TokenId;
    let mut contents: ArcStr;
    let mut byteOffset: i32;
    let mut length: i32;
    let Token {
        id: __pa0,
        fileContents: __pa1,
        byteOffset: __pa2,
        length: __pa3,
        ..
    } = &token;
    id = metamodelica::Own::own(__pa0);
    contents = metamodelica::Own::own(__pa1);
    byteOffset = metamodelica::Own::own(__pa2);
    length = metamodelica::Own::own(__pa3);
    contents = if (length > 0) {
        substring(contents, byteOffset, byteOffset + length - 1)?
    } else {
        literal!("")
    };
    strTk = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("[TOKEN:"));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", id)));
        __mm_s.push_str(&*literal!(" '"));
        __mm_s.push_str(&*contents);
        __mm_s.push_str(&*literal!("' ("));
        __mm_s.push_str(&*intString(token.lineNumberStart.clone()));
        __mm_s.push_str(&*literal!(":"));
        __mm_s.push_str(&*intString(token.columnNumberStart.clone()));
        __mm_s.push_str(&*literal!("-"));
        __mm_s.push_str(&*intString(token.lineNumberEnd.clone()));
        __mm_s.push_str(&*literal!(":"));
        __mm_s.push_str(&*intString(token.columnNumberEnd.clone()));
        __mm_s.push_str(&*literal!(")]"));
        ArcStr::from(__mm_s)
    };
    Ok(strTk)
}

pub(crate) fn tokenContent(mut token: Token) -> Result<ArcStr> {
    let mut contents: ArcStr;
    let mut byteOffset: i32;
    let mut length: i32;
    let Token {
        fileContents: __pa0,
        byteOffset: __pa1,
        length: __pa2,
        ..
    } = token;
    contents = metamodelica::Own::own(__pa0);
    byteOffset = metamodelica::Own::own(__pa1);
    length = metamodelica::Own::own(__pa2);
    contents = if (length > 0) {
        substring(contents, byteOffset, byteOffset + length - 1)?
    } else {
        literal!("")
    };
    Ok(contents)
}

pub(crate) fn tokenContentEq(mut token1: Token, mut token2: Token) -> bool {
    let mut b: bool;
    let mut contents1: ArcStr;
    let mut contents2: ArcStr;
    let mut offset1: i32;
    let mut length1: i32;
    let mut offset2: i32;
    let mut length2: i32;
    let Token {
        fileContents: __pa0,
        byteOffset: __pa1,
        length: __pa2,
        ..
    } = token1;
    contents1 = metamodelica::Own::own(__pa0);
    offset1 = metamodelica::Own::own(__pa1);
    length1 = metamodelica::Own::own(__pa2);
    let Token {
        fileContents: __pa3,
        byteOffset: __pa4,
        length: __pa5,
        ..
    } = token2;
    contents2 = metamodelica::Own::own(__pa3);
    offset2 = metamodelica::Own::own(__pa4);
    length2 = metamodelica::Own::own(__pa5);
    b = if (length1 != length2) {
        false
    } else {
        0 == System::strcmp_offset(contents1, offset1, length1, contents2, offset2, length2)
    };
    b
}

pub(crate) fn tokenSourceInfo(mut token: Token) -> SourceInfo {
    let mut info: SourceInfo;
    info = {
        let mut t = token;
        (match t.clone() {
            Token { .. } => SourceInfo {
                fileName: t.fileName.clone(),
                isReadOnly: false,
                lineNumberStart: t.lineNumberStart.clone(),
                columnNumberStart: t.columnNumberStart.clone(),
                lineNumberEnd: t.lineNumberEnd.clone(),
                columnNumberEnd: t.columnNumberEnd.clone(),
                lastModification: metamodelica::OrderedFloat(0.0_f64),
            },
        })
    };
    info
}

fn lex(mut fileName: ArcStr, mut contents: ArcStr) -> Result<(metamodelica::List<Token>, metamodelica::List<Token>)> {
    let mut tokens: metamodelica::List<Token>;
    let mut errorTokens: metamodelica::List<Token> = metamodelica::nil();
    let mut startSt: i32;
    let mut i: i32;
    let mut cTok: i32;
    let mut currSt: i32;
    let mut pos: i32;
    let mut sPos: i32;
    let mut ePos: i32;
    let mut linenr: i32;
    let mut contentLen: i32;
    let mut numBacktrack: i32;
    let mut buffer: i32;
    let mut lineNrStart: i32;
    let mut states: metamodelica::List<i32>;
    startSt = 1;
    currSt = 1;
    pos = 1;
    sPos = 0;
    ePos = 0;
    linenr = 1;
    lineNrStart = 1;
    buffer = 0;
    states = metamodelica::nil();
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nLexer analyzer LexerCode..."));
            __mm_s.push_str(&*fileName);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    tokens = metamodelica::nil();
    if debug.clone() {
        metamodelica::print(literal!("\n TOTAL Chars:"));
        metamodelica::print(intString(((contents).len() as i32)));
    }
    contentLen = ((contents).len() as i32);
    i = 1;
    while i <= contentLen {
        cTok = stringGet(&contents, i)?;
        (
            tokens,
            numBacktrack,
            startSt,
            currSt,
            pos,
            sPos,
            ePos,
            linenr,
            lineNrStart,
            buffer,
            states,
            errorTokens,
        ) = consume(
            cTok,
            tokens,
            contents.clone(),
            startSt,
            currSt,
            pos,
            sPos,
            ePos,
            linenr,
            lineNrStart,
            buffer,
            states,
            fileName.clone(),
            errorTokens,
        )?;
        i = i - numBacktrack + 1;
    }
    tokens = metamodelica::Dangerous::listReverseInPlace(tokens);
    errorTokens = metamodelica::Dangerous::listReverseInPlace(errorTokens);
    Ok((tokens, errorTokens))
}

fn consume(
    mut cp: i32,
    mut tokens: metamodelica::List<Token>,
    mut fileContents: ArcStr,
    mut startSt: i32,
    mut currSt: i32,
    mut pos: i32,
    mut sPos: i32,
    mut ePos: i32,
    mut linenr: i32,
    mut inLineNrStart: i32,
    mut inBuffer: i32,
    mut inStates: metamodelica::List<i32>,
    mut fileName: ArcStr,
    mut inErrorTokens: metamodelica::List<Token>,
) -> Result<(
    metamodelica::List<Token>,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    metamodelica::List<i32>,
    metamodelica::List<Token>,
)> {
    let mut resToken: metamodelica::List<Token>;
    let mut bkBuffer: i32 = 0;
    let mut mm_startSt: i32;
    let mut mm_currSt: i32;
    let mut mm_pos: i32;
    let mut mm_sPos: i32;
    let mut mm_ePos: i32;
    let mut mm_linenr: i32;
    let mut lineNrStart: i32;
    let mut buffer: i32;
    let mut states: metamodelica::List<i32>;
    let mut errorTokens: metamodelica::List<Token> = inErrorTokens;
    let mut tok: Token;
    let mut act: i32;
    let mut buffer2: i32;
    let mut c: i32;
    let mut baseCond: i32;
    mm_startSt = startSt;
    mm_currSt = currSt;
    mm_pos = pos;
    mm_sPos = sPos;
    mm_ePos = ePos;
    mm_linenr = linenr;
    lineNrStart = inLineNrStart;
    buffer = inBuffer;
    states = inStates;
    baseCond = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), mm_currSt)?).clone();
        __elt
    });
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nPROGRAM:{"));
            __mm_s.push_str(&*intString(cp));
            __mm_s.push_str(&*literal!("} "));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nBUFFER:{"));
            __mm_s.push_str(&*intString(buffer));
            __mm_s.push_str(&*literal!("} "));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("base:"));
            __mm_s.push_str(&*intString(baseCond));
            __mm_s.push_str(&*literal!(" st:"));
            __mm_s.push_str(&*intString(mm_currSt));
            __mm_s.push_str(&*literal!(" "));
            ArcStr::from(__mm_s)
        });
    }
    buffer = buffer + 1;
    mm_pos = mm_pos + 1;
    if cp == 10 {
        mm_linenr = mm_linenr + 1;
        mm_sPos = 0;
    } else {
        mm_sPos = mm_sPos + 1;
    }
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n[Reading:'"));
            __mm_s.push_str(&*intStringChar(cp));
            __mm_s.push_str(&*literal!("' at p:"));
            __mm_s.push_str(&*intString(mm_pos - 1));
            __mm_s.push_str(&*literal!(" line:"));
            __mm_s.push_str(&*intString(mm_linenr));
            __mm_s.push_str(&*literal!(" rPos:"));
            __mm_s.push_str(&*intString(mm_sPos));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        });
    }
    c = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_ec.borrow(), cp)?).clone();
        __elt
    });
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" evalState Before[c"));
            __mm_s.push_str(&*intString(c));
            __mm_s.push_str(&*literal!(",s"));
            __mm_s.push_str(&*intString(mm_currSt));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        });
    }
    (mm_currSt, c) = evalState(mm_currSt, c)?;
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" After[c"));
            __mm_s.push_str(&*intString(c));
            __mm_s.push_str(&*literal!(",s"));
            __mm_s.push_str(&*intString(mm_currSt));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        });
    }
    if mm_currSt > 0 {
        mm_currSt = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), mm_currSt)?).clone();
            __elt
        });
        mm_currSt = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_nxt.borrow(), mm_currSt + c)?).clone();
            __elt
        });
    } else {
        mm_currSt = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_nxt.borrow(), c)?).clone();
            __elt
        });
    }
    states = metamodelica::cons(mm_currSt, states);
    baseCond = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), mm_currSt)?).clone();
        __elt
    });
    if baseCond == LexTable::yy_finish.clone() {
        if debug.clone() == true {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n[RESTORE="));
                __mm_s.push_str(&*intString(
                    ({
                        let __elt = (*metamodelica::index_checked(&LexTable::yy_accept.borrow(), mm_currSt)?).clone();
                        __elt
                    }),
                ));
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            });
        }
        (act, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states) = findRule(
            &fileContents,
            mm_currSt,
            mm_pos,
            mm_sPos,
            mm_ePos,
            mm_linenr,
            buffer,
            bkBuffer,
            states,
        )?;
        if debug.clone() == true {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nFound rule: "));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", act)));
                ArcStr::from(__mm_s)
            });
        }
        (tok, mm_startSt, buffer2, errorTokens) = action(
            act,
            mm_startSt,
            mm_currSt,
            mm_pos,
            mm_sPos,
            mm_ePos,
            mm_linenr,
            lineNrStart,
            buffer,
            fileName,
            fileContents,
            errorTokens,
        )?;
        if debug.clone() == true {
            metamodelica::print(literal!("\nDid action"));
        }
        mm_currSt = mm_startSt;
        states = metamodelica::nil();
        if buffer != buffer2 {
            mm_ePos = mm_sPos;
            lineNrStart = linenr;
        }
        buffer = buffer2;
        resToken = (match tok.clone() {
            Token {
                id: TokenId::_NO_TOKEN, ..
            } => tokens,
            _ => metamodelica::cons(tok, tokens),
        });
        if debug.clone() {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n CountTokens:"));
                __mm_s.push_str(&*intString(((resToken).len() as i32)));
                ArcStr::from(__mm_s)
            });
        }
    } else {
        bkBuffer = 0;
        resToken = tokens;
    }
    Ok((
        resToken,
        bkBuffer,
        mm_startSt,
        mm_currSt,
        mm_pos,
        mm_sPos,
        mm_ePos,
        mm_linenr,
        lineNrStart,
        buffer,
        states,
        errorTokens,
    ))
}

fn findRule<'__b>(
    mut fileContents: &'__b ArcStr,
    mut currSt: i32,
    mut pos: i32,
    mut sPos: i32,
    mut mm_ePos: i32,
    mut linenr: i32,
    mut inBuffer: i32,
    mut inBkBuffer: i32,
    mut inStates: metamodelica::List<i32>,
) -> Result<(i32, i32, i32, i32, i32, i32, i32, metamodelica::List<i32>)> {
    let mut action: i32;
    let mut mm_currSt: i32;
    let mut mm_pos: i32;
    let mut mm_sPos: i32;
    let mut mm_linenr: i32;
    let mut buffer: i32;
    let mut bkBuffer: i32;
    let mut states: metamodelica::List<i32>;
    let mut lp: i32;
    let mut lp1: i32;
    let mut stCmp: i32;
    let mut cp: i32;
    let mut st: bool;
    mm_currSt = currSt;
    mm_pos = pos;
    mm_sPos = sPos;
    mm_linenr = linenr;
    buffer = inBuffer;
    bkBuffer = inBkBuffer;
    states = inStates;
    stCmp = (states).get(1)?;
    lp = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_accept.borrow(), stCmp)?).clone();
        __elt
    });
    lp1 = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_accept.borrow(), stCmp + 1)?).clone();
        __elt
    });
    st = intGt(lp, 0) && intLt(lp, lp1);
    if st {
        if debug.clone() {
            checkArrayModelica(
                LexTable::yy_accept.clone(),
                stCmp,
                metamodelica::sourceInfo!("Lexers/LexerJSON.mo"),
            )?;
            checkArrayModelica(
                LexTable::yy_acclist.clone(),
                lp,
                metamodelica::sourceInfo!("Lexers/LexerJSON.mo"),
            )?;
        }
        lp = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_accept.borrow(), stCmp)?).clone();
            __elt
        });
        action = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_acclist.borrow(), lp)?).clone();
            __elt
        });
    } else {
        cp = stringGet(&fileContents, mm_pos - 1)?;
        buffer = buffer - 1;
        bkBuffer = bkBuffer + 1;
        mm_pos = mm_pos - 1;
        mm_sPos = mm_sPos - 1;
        if cp == 10 {
            mm_sPos = mm_ePos;
            mm_linenr = mm_linenr - 1;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(states) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        mm_currSt = metamodelica::Own::own(__pa0);
        states = metamodelica::Own::own(__pa1);
        (action, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states) = findRule(
            fileContents,
            mm_currSt,
            mm_pos,
            mm_sPos,
            mm_ePos,
            mm_linenr,
            buffer,
            bkBuffer,
            states,
        )?;
    }
    Ok((action, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states))
}

fn evalState(mut cState: i32, mut c: i32) -> Result<(i32, i32)> {
    let mut new_state: i32;
    let mut new_c: i32;
    let mut cState1: i32 = cState;
    let mut c1: i32 = c;
    let mut val: i32;
    let mut val2: i32;
    let mut chk: i32;
    chk = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), cState1)?).clone();
        __elt
    });
    chk = chk + c1;
    val = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_chk.borrow(), chk)?).clone();
        __elt
    });
    val2 = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), cState1)?).clone();
        __elt
    }) + c1;
    if cState1 != val {
        cState1 = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_def.borrow(), cState1)?).clone();
            __elt
        });
        if cState1 >= LexTable::yy_limit.clone() {
            c1 = ({
                let __elt = (*metamodelica::index_checked(&LexTable::yy_meta.borrow(), c1)?).clone();
                __elt
            });
        }
        if cState1 > 0 {
            (cState1, c1) = evalState(cState1, c1)?;
        }
    }
    new_state = cState1;
    new_c = c1;
    Ok((new_state, new_c))
}

fn checkArray<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut index: i32,
    mut info: SourceInfo,
) -> Result<()> {
    let mut filename: ArcStr;
    let mut lineStart: i32;
    if index < 1 || index > metamodelica::arrayLength(arr.clone()) {
        let SourceInfo {
            fileName: __pa0,
            lineNumberStart: __pa1,
            ..
        } = (info)
        else {
            return Err("pattern mismatch");
        };
        filename = metamodelica::Own::own(__pa0);
        lineStart = metamodelica::Own::own(__pa1);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*filename);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", lineStart)));
            __mm_s.push_str(&*literal!("]: checkArray failed: arrayLength="));
            __mm_s.push_str(&*ArcStr::from(::std::format!(
                "{}",
                metamodelica::arrayLength(arr.clone())
            )));
            __mm_s.push_str(&*literal!(" index="));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        return Err("fail");
    }
    Ok(())
}

fn checkArrayModelica(mut arr: metamodelica::Array<i32>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let mut filename: ArcStr;
    let mut lineStart: i32;
    if index < 1 || index > metamodelica::arrayLength(arr.clone()) {
        let SourceInfo {
            fileName: __pa0,
            lineNumberStart: __pa1,
            ..
        } = (info)
        else {
            return Err("pattern mismatch");
        };
        filename = metamodelica::Own::own(__pa0);
        lineStart = metamodelica::Own::own(__pa1);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*filename);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", lineStart)));
            __mm_s.push_str(&*literal!("]: checkArray failed: arrayLength="));
            __mm_s.push_str(&*ArcStr::from(::std::format!(
                "{}",
                metamodelica::arrayLength(arr.clone())
            )));
            __mm_s.push_str(&*literal!(" index="));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        return Err("fail");
    }
    Ok(())
}
