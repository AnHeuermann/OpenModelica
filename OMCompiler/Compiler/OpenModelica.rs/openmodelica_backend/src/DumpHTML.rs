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

use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_backend_types::BackendDAE;
use openmodelica_util::ExpandableArray;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// types
//
// =============================================================================
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Style {
    pub name: ArcStr,
    pub value: ArcStr,
}

impl metamodelica::gc::MMTrace for Style {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        Ok(())
    }
}
pub type STYLE = Style;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Tag {
    HEADING {
        stage: i32,
        text: ArcStr,
    },
    HYPERLINK {
        /// #anker or javascript:toggle
        href: ArcStr,
        title: ArcStr,
        text: ArcStr,
    },
    ANKER {
        name: ArcStr,
    },
    LINE {
        text: ArcStr,
    },
    DIVISION {
        id: ArcStr,
        style: metamodelica::List<Style>,
        tags: metamodelica::List<metamodelica::Ref<Tag>>,
    },
    SCRIPT {
        type_: ArcStr,
        text: ArcStr,
    },
    SCRIPT_BODY {
        type_: ArcStr,
        text: ArcStr,
    },
    CANVAS {
        attr: metamodelica::List<ArcStr>,
    },
}
impl metamodelica::gc::MMTrace for Tag {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Tag::HEADING { stage, text } => {
                metamodelica::gc::MMTrace::mm_accept(stage, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(text, __mmv)?;
                Ok(())
            }
            Tag::HYPERLINK { href, title, text } => {
                metamodelica::gc::MMTrace::mm_accept(href, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(title, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(text, __mmv)?;
                Ok(())
            }
            Tag::ANKER { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            Tag::LINE { text } => {
                metamodelica::gc::MMTrace::mm_accept(text, __mmv)?;
                Ok(())
            }
            Tag::DIVISION { id, style, tags } => {
                metamodelica::gc::MMTrace::mm_accept(id, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(style, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(tags, __mmv)?;
                Ok(())
            }
            Tag::SCRIPT { type_, text } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(text, __mmv)?;
                Ok(())
            }
            Tag::SCRIPT_BODY { type_, text } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(text, __mmv)?;
                Ok(())
            }
            Tag::CANVAS { attr } => {
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Tag {
    fn default() -> Self {
        Self::ANKER {
            name: Default::default(),
        }
    }
}
pub(crate) use self::Tag::{ANKER, CANVAS, DIVISION, HEADING, HYPERLINK, LINE, SCRIPT, SCRIPT_BODY};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Document {
    pub docType: ArcStr,
    /// because of performance issues tags in reverse order
    pub head: metamodelica::List<metamodelica::Ref<Tag>>,
    /// because of performance issues tags in reverse order
    pub body: metamodelica::List<metamodelica::Ref<Tag>>,
}

impl metamodelica::gc::MMTrace for Document {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.docType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.head, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.body, __mmv)?;
        Ok(())
    }
}
impl Default for Document {
    fn default() -> Self {
        Self {
            docType: Default::default(),
            head: Default::default(),
            body: Default::default(),
        }
    }
}

pub type DOCUMENT = Document;

pub(crate) static emptyDocument: std::sync::LazyLock<Document> = std::sync::LazyLock::new(|| Document {
    docType: literal!(""),
    head: metamodelica::nil(),
    body: metamodelica::nil(),
});

fn emptyDocumentWithToggleFunktion() -> Document {
    let mut outDoc: Document;
    outDoc = addScript(
        literal!("text/Javascript"),
        literal!(
            "function toggle(name) {\n   var element = document.getElementById(name);\n   if (element.style.display == \"none\") {\n      // show the div\n      element.style.display = \"block\";   \n   } else {\n      // hide the div\n      element.style.display = \"none\";\n      // reset element\n      element.reset();\n   }\n}\n\nfunction show(name) {\n   var element = document.getElementById(name);\n   if (element.style.display == \"none\") {\n      // show the div\n      element.style.display = \"block\";   \n   }\n   return true;\n}\n\n    "
        ),
        emptyDocument.clone(),
    );
    outDoc
}

fn addScript(mut type_: ArcStr, mut script: ArcStr, mut inDoc: Document) -> Document {
    let mut outDoc: Document;
    outDoc = addHeadTag(
        metamodelica::Ref::new(Tag::SCRIPT {
            type_: type_,
            text: script,
        }),
        inDoc,
    );
    outDoc
}

fn addScriptBody(mut type_: ArcStr, mut script: ArcStr, mut inDoc: Document) -> Document {
    let mut outDoc: Document;
    outDoc = addBodyTag(
        metamodelica::Ref::new(Tag::SCRIPT_BODY {
            type_: type_,
            text: script,
        }),
        inDoc,
    );
    outDoc
}

fn addHeading(mut stage: i32, mut text: ArcStr, mut inDoc: Document) -> Document {
    let mut outDoc: Document;
    outDoc = addBodyTag(
        metamodelica::Ref::new(Tag::HEADING {
            stage: stage,
            text: text,
        }),
        inDoc,
    );
    outDoc
}

fn addHeadingTag(
    mut stage: i32,
    mut text: ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> metamodelica::List<metamodelica::Ref<Tag>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    outTags = metamodelica::cons(
        metamodelica::Ref::new(Tag::HEADING {
            stage: stage,
            text: text,
        }),
        inTags,
    );
    outTags
}

fn addHyperLink(mut href: ArcStr, mut title: ArcStr, mut text: ArcStr, mut inDoc: Document) -> Document {
    let mut outDoc: Document;
    outDoc = addBodyTag(
        metamodelica::Ref::new(Tag::HYPERLINK {
            href: href,
            title: title,
            text: text,
        }),
        inDoc,
    );
    outDoc
}

fn addHyperLinkTag(
    mut href: ArcStr,
    mut title: ArcStr,
    mut text: ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> metamodelica::List<metamodelica::Ref<Tag>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    outTags = metamodelica::cons(
        metamodelica::Ref::new(Tag::HYPERLINK {
            href: href,
            title: title,
            text: text,
        }),
        inTags,
    );
    outTags
}

fn addAnkerTag(
    mut name: ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> metamodelica::List<metamodelica::Ref<Tag>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    outTags = metamodelica::cons(metamodelica::Ref::new(Tag::ANKER { name: name }), inTags);
    outTags
}

fn addLine(mut text: ArcStr, mut inDoc: Document) -> Document {
    let mut outDoc: Document;
    outDoc = addBodyTag(metamodelica::Ref::new(Tag::LINE { text: text }), inDoc);
    outDoc
}

fn addLineTag(
    mut text: ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> metamodelica::List<metamodelica::Ref<Tag>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    outTags = metamodelica::cons(metamodelica::Ref::new(Tag::LINE { text: text }), inTags);
    outTags
}

fn addDivision(
    mut id: ArcStr,
    mut style: metamodelica::List<Style>,
    mut tags: metamodelica::List<metamodelica::Ref<Tag>>,
    mut inDoc: Document,
) -> Document {
    let mut outDoc: Document;
    let mut t: metamodelica::List<metamodelica::Ref<Tag>>;
    t = tags.reverse();
    outDoc = addBodyTag(
        metamodelica::Ref::new(Tag::DIVISION {
            id: id,
            style: style,
            tags: t,
        }),
        inDoc,
    );
    outDoc
}

fn addDivisionTag(
    mut id: ArcStr,
    mut style: metamodelica::List<Style>,
    mut tags: metamodelica::List<metamodelica::Ref<Tag>>,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> metamodelica::List<metamodelica::Ref<Tag>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut t: metamodelica::List<metamodelica::Ref<Tag>>;
    t = tags.reverse();
    outTags = metamodelica::cons(
        metamodelica::Ref::new(Tag::DIVISION {
            id: id,
            style: style,
            tags: t,
        }),
        inTags,
    );
    outTags
}

fn addBodyTags(mut tags: metamodelica::List<metamodelica::Ref<Tag>>, mut inDoc: Document) -> Document {
    let mut outDoc: Document;
    let mut docType: ArcStr;
    let mut head: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut body: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut t: metamodelica::List<metamodelica::Ref<Tag>>;
    t = tags.reverse();
    let Document {
        docType: __pa0,
        head: __pa1,
        body: __pa2,
    } = inDoc;
    docType = metamodelica::Own::own(__pa0);
    head = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    outDoc = Document {
        docType: docType,
        head: head,
        body: listAppend(body, t),
    };
    outDoc
}

fn dumpDocument(mut inDoc: Document, mut name: ArcStr) -> Result<()> {
    let mut r#str: ArcStr;
    let mut head: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut body: metamodelica::List<metamodelica::Ref<Tag>>;
    let Document {
        docType: __pa0,
        head: __pa1,
        body: __pa2,
    } = inDoc;
    r#str = metamodelica::Own::own(__pa0);
    head = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("\n<html>\n<head>"));
        ArcStr::from(__mm_s)
    };
    r#str = List::fold(
        &(head.reverse()),
        &move |__a0: metamodelica::Ref<Tag>, __a1: ArcStr| dumpTag(&__a0, &__a1),
        r#str,
    )?;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("\n</head>"));
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("\n<body>"));
        ArcStr::from(__mm_s)
    };
    r#str = List::fold(
        &(body.reverse()),
        &move |__a0: metamodelica::Ref<Tag>, __a1: ArcStr| dumpTag(&__a0, &__a1),
        r#str,
    )?;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("\n</body>\n</html>"));
        ArcStr::from(__mm_s)
    };
    System::writeFile(name, r#str)?;
    Ok(())
}

fn addHeadTag(mut tag: metamodelica::Ref<Tag>, mut inDoc: Document) -> Document {
    let mut outDoc: Document;
    let mut docType: ArcStr;
    let mut head: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut body: metamodelica::List<metamodelica::Ref<Tag>>;
    let Document {
        docType: __pa0,
        head: __pa1,
        body: __pa2,
    } = inDoc;
    docType = metamodelica::Own::own(__pa0);
    head = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    outDoc = Document {
        docType: docType,
        head: metamodelica::cons(tag, head),
        body: body,
    };
    outDoc
}

fn addBodyTag(mut tag: metamodelica::Ref<Tag>, mut inDoc: Document) -> Document {
    let mut outDoc: Document;
    let mut docType: ArcStr;
    let mut head: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut body: metamodelica::List<metamodelica::Ref<Tag>>;
    let Document {
        docType: __pa0,
        head: __pa1,
        body: __pa2,
    } = inDoc;
    docType = metamodelica::Own::own(__pa0);
    head = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    outDoc = Document {
        docType: docType,
        head: head,
        body: metamodelica::cons(tag, body),
    };
    outDoc
}

fn dumpTag(mut tag: &metamodelica::Ref<Tag>, mut iBuffer: &ArcStr) -> Result<ArcStr> {
    let mut oBuffer: ArcStr;
    oBuffer = (match &**tag {
        Tag::HEADING { stage: i, text: t } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iBuffer);
                __mm_s.push_str(&*literal!("\n<h"));
                __mm_s.push_str(&*intString(i.clone()));
                __mm_s.push_str(&*literal!(">"));
                __mm_s.push_str(&*t);
                __mm_s.push_str(&*literal!("</h"));
                __mm_s.push_str(&*intString(i.clone()));
                __mm_s.push_str(&*literal!(">"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Tag::HYPERLINK {
            href: t,
            title: t1,
            text: t2,
        } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iBuffer);
                __mm_s.push_str(&*literal!("\n<a href=\""));
                __mm_s.push_str(&*t);
                __mm_s.push_str(&*literal!("\" title=\""));
                __mm_s.push_str(&*t1);
                __mm_s.push_str(&*literal!("\">"));
                __mm_s.push_str(&*t2);
                __mm_s.push_str(&*literal!("</a>"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Tag::ANKER { name: t } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iBuffer);
                __mm_s.push_str(&*literal!("\n<a name=\""));
                __mm_s.push_str(&*t);
                __mm_s.push_str(&*literal!("\"/>"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Tag::LINE { text: t } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iBuffer);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*t);
                __mm_s.push_str(&*literal!("<br>"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Tag::DIVISION { id: t, style, tags } => {
            let mut t1: ArcStr;
            let mut t2: ArcStr;
            let mut r#str: ArcStr;
            t1 = stringDelimitList(List::map(style.clone(), &fnptr!(dumpStyle, Style))?, literal!("; "));
            t2 = List::fold(
                tags,
                &move |__a0: metamodelica::Ref<Tag>, __a1: ArcStr| dumpTag(&__a0, &__a1),
                literal!(""),
            )?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iBuffer);
                __mm_s.push_str(&*literal!("\n<div id=\""));
                __mm_s.push_str(&*t);
                __mm_s.push_str(&*literal!("\" style=\""));
                __mm_s.push_str(&*t1);
                __mm_s.push_str(&*literal!("\">\n"));
                __mm_s.push_str(&*t2);
                __mm_s.push_str(&*literal!("\n</div>"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Tag::SCRIPT { type_: t1, text: t2 } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iBuffer);
                __mm_s.push_str(&*literal!("\n<script type=\""));
                __mm_s.push_str(&*t1);
                __mm_s.push_str(&*literal!("\">\n"));
                __mm_s.push_str(&*t2);
                __mm_s.push_str(&*literal!("\n</script>"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Tag::SCRIPT_BODY { type_: t1, text: t2 } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iBuffer);
                __mm_s.push_str(&*literal!("\n<SCRIPT \""));
                __mm_s.push_str(&*t1);
                __mm_s.push_str(&*literal!("\">\n"));
                __mm_s.push_str(&*t2);
                __mm_s.push_str(&*literal!("\n</SCRIPT>"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Tag::CANVAS { attr } => {
            let mut t1: ArcStr;
            let mut r#str: ArcStr;
            t1 = stringDelimitList(attr.clone(), literal!(" "));
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iBuffer);
                __mm_s.push_str(&*literal!("\n<canvas "));
                __mm_s.push_str(&*t1);
                __mm_s.push_str(&*literal!("\">\n"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
    });
    Ok(oBuffer)
}

fn dumpStyle(mut inStyle: Style) -> ArcStr {
    let mut outBuffer: ArcStr;
    let mut name: ArcStr;
    let mut value: ArcStr;
    let Style {
        name: __pa0,
        value: __pa1,
    } = inStyle;
    name = metamodelica::Own::own(__pa0);
    value = metamodelica::Own::own(__pa1);
    outBuffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*value);
        ArcStr::from(__mm_s)
    };
    outBuffer
}

pub(crate) fn dumpDAE(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inHeader: ArcStr,
    mut inFilename: &ArcStr,
) -> Result<()> {
    let mut doc: Document;
    let mut r#str: ArcStr;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*inDAE);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    eqs = metamodelica::Own::own(__pa0);
    doc = emptyDocumentWithToggleFunktion();
    doc = addHeading(1, inHeader, doc);
    r#str = intString(((System::time()).0.floor() as i32));
    (doc, _) = List::fold1(
        &eqs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: ArcStr, __a2: (Document, i32)| {
            dumpEqSystem(&__a0, &__a1, &__a2)
        },
        r#str.clone(),
        (doc, 1),
    )?;
    dumpDocument(doc, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*inFilename);
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

fn dumpEqSystem(
    mut inEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inPrefixIdstr: &ArcStr,
    mut inTpl: &(Document, i32),
) -> Result<(Document, i32)> {
    let mut outTpl: (Document, i32);
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut i: i32;
    let mut varlen_str: ArcStr;
    let mut eqnlen_str: ArcStr;
    let mut prefixId: ArcStr;
    let mut eqnsl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut vars1: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: Option<metamodelica::Array<metamodelica::List<i32>>>;
    let mut mT: Option<metamodelica::Array<metamodelica::List<i32>>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut doc: Document;
    let mut tags: metamodelica::List<metamodelica::Ref<Tag>>;
    let __arc5 = &(*inEqSystem);
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        m: __pa2,
        mT: __pa3,
        matching: __pa4,
        ..
    } = &**__arc5;
    vars1 = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    m = metamodelica::Own::own(__pa2);
    mT = metamodelica::Own::own(__pa3);
    matching = metamodelica::Own::own(__pa4);
    (doc, i) = inTpl.clone();
    prefixId = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*inPrefixIdstr);
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*intString(i));
        ArcStr::from(__mm_s)
    };
    vars = BackendVariable::varList(&vars1)?;
    varlen_str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Variables ("));
        __mm_s.push_str(&*intString(((vars).len() as i32)));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    tags = addHeadingTag(2, varlen_str, metamodelica::nil());
    tags = printVarList(&vars, prefixId.clone(), tags)?;
    eqnsl = BackendEquation::equationList(eqns.clone())?;
    eqnlen_str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Equations ("));
        __mm_s.push_str(&*intString(((eqnsl).len() as i32)));
        __mm_s.push_str(&*literal!(", "));
        __mm_s.push_str(&*intString(BackendEquation::equationArraySize(eqns)?));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    tags = addHeadingTag(2, eqnlen_str, tags);
    tags = dumpEqns(&eqnsl, prefixId.clone(), tags)?;
    tags = dumpFullMatching(&matching, &prefixId, tags);
    doc = addLine(literal!("<hr>"), doc);
    doc = addHyperLink(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("javascript:toggle('"));
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("system')"));
            ArcStr::from(__mm_s)
        },
        literal!("show system"),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("show/hide system "));
            __mm_s.push_str(&*intString(i));
            ArcStr::from(__mm_s)
        },
        doc,
    );
    doc = addDivision(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("system"));
            ArcStr::from(__mm_s)
        },
        list![Style {
            name: literal!("display"),
            value: literal!("none")
        }],
        tags,
        doc,
    );
    outTpl = (doc, i + 1);
    Ok(outTpl)
}

fn printVarList(
    mut vars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut prefixId: ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> Result<metamodelica::List<metamodelica::Ref<Tag>>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut tags: metamodelica::List<metamodelica::Ref<Tag>>;
    (tags, _) = List::fold1(
        vars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>,
               __a1: ArcStr,
               __a2: (metamodelica::List<metamodelica::Ref<Tag>>, i32)| dumpVar(&__a0, &__a1, &__a2),
        prefixId.clone(),
        (metamodelica::nil(), 1),
    )?;
    outTags = addHyperLinkTag(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("javascript:toggle('"));
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("variables')"));
            ArcStr::from(__mm_s)
        },
        literal!("show variables"),
        literal!("show/hide variables"),
        inTags,
    );
    outTags = addDivisionTag(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("variables"));
            ArcStr::from(__mm_s)
        },
        list![
            Style {
                name: literal!("background"),
                value: literal!("#FFFFCC")
            },
            Style {
                name: literal!("display"),
                value: literal!("none")
            }
        ],
        tags,
        outTags,
    );
    Ok(outTags)
}

fn dumpVar(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut prefixId: &ArcStr,
    mut inTpl: &(metamodelica::List<metamodelica::Ref<Tag>>, i32),
) -> Result<(metamodelica::List<metamodelica::Ref<Tag>>, i32)> {
    let mut oTpl: (metamodelica::List<metamodelica::Ref<Tag>>, i32);
    let mut tags: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut i: i32;
    let mut ln: ArcStr;
    let mut istr: ArcStr;
    (tags, i) = inTpl.clone();
    istr = intString(i);
    ln = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*prefixId);
        __mm_s.push_str(&*literal!("varanker"));
        __mm_s.push_str(&*istr);
        ArcStr::from(__mm_s)
    };
    tags = addAnkerTag(ln, tags);
    ln = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*istr);
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*BackendDump::varString(inVar)?);
        ArcStr::from(__mm_s)
    };
    tags = addLineTag(ln, tags);
    oTpl = (tags, i + 1);
    Ok(oTpl)
}

fn dumpEqns(
    mut eqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut prefixId: ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> Result<metamodelica::List<metamodelica::Ref<Tag>>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut tags: metamodelica::List<metamodelica::Ref<Tag>>;
    (tags, _) = List::fold1(
        eqns,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: ArcStr,
               __a2: (metamodelica::List<metamodelica::Ref<Tag>>, i32)| dumpEqn(&__a0, &__a1, &__a2),
        prefixId.clone(),
        (metamodelica::nil(), 1),
    )?;
    outTags = addHyperLinkTag(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("javascript:toggle('"));
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("equations')"));
            ArcStr::from(__mm_s)
        },
        literal!("show equations"),
        literal!("show/hide equations"),
        inTags,
    );
    outTags = addDivisionTag(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("equations"));
            ArcStr::from(__mm_s)
        },
        list![
            Style {
                name: literal!("background"),
                value: literal!("#C0C0C0")
            },
            Style {
                name: literal!("display"),
                value: literal!("none")
            }
        ],
        tags,
        outTags,
    );
    Ok(outTags)
}

fn dumpEqn(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut prefixId: &ArcStr,
    mut inTpl: &(metamodelica::List<metamodelica::Ref<Tag>>, i32),
) -> Result<(metamodelica::List<metamodelica::Ref<Tag>>, i32)> {
    let mut oTpl: (metamodelica::List<metamodelica::Ref<Tag>>, i32);
    let mut tags: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut i: i32;
    let mut ln: ArcStr;
    let mut istr: ArcStr;
    (tags, i) = inTpl.clone();
    istr = intString(i);
    ln = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*prefixId);
        __mm_s.push_str(&*literal!("eqanker"));
        __mm_s.push_str(&*istr);
        ArcStr::from(__mm_s)
    };
    tags = addAnkerTag(ln, tags);
    ln = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*istr);
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(BackendEquation::equationSize(inEquation)?));
        __mm_s.push_str(&*literal!("): "));
        __mm_s.push_str(&*BackendDump::equationString(inEquation)?);
        ArcStr::from(__mm_s)
    };
    tags = addLineTag(ln, tags);
    oTpl = (tags, i + 1);
    Ok(oTpl)
}

fn dumpFullMatching(
    mut inMatch: &metamodelica::Ref<BackendDAE::Matching>,
    mut prefixId: &ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> metamodelica::List<metamodelica::Ref<Tag>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    outTags = (match &**inMatch {
        BackendDAE::Matching::NO_MATCHING { .. } => inTags,
        BackendDAE::Matching::MATCHING {
            ass1,
            ass2: _,
            comps: _,
        } => {
            let mut tags: metamodelica::List<metamodelica::Ref<Tag>>;
            tags = dumpMatching(ass1.clone(), prefixId, inTags);
            tags
        }
    });
    outTags
}

fn dumpMatching(
    mut v: metamodelica::Array<i32>,
    mut prefixId: &ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> metamodelica::List<metamodelica::Ref<Tag>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut len: i32;
    let mut len_str: ArcStr;
    let mut tags: metamodelica::List<metamodelica::Ref<Tag>>;
    outTags = addHeadingTag(2, literal!("Matching"), inTags);
    len = metamodelica::arrayLength(v.clone());
    len_str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(len));
        __mm_s.push_str(&*literal!(" variables and equations\n"));
        ArcStr::from(__mm_s)
    };
    outTags = addLineTag(len_str, outTags);
    tags = dumpMatching2(v.clone(), 1, len, prefixId, metamodelica::nil());
    outTags = addHyperLinkTag(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("javascript:toggle('"));
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("matching')"));
            ArcStr::from(__mm_s)
        },
        literal!("show matching"),
        literal!("show/hide matching"),
        outTags,
    );
    outTags = addDivisionTag(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("matching"));
            ArcStr::from(__mm_s)
        },
        list![
            Style {
                name: literal!("background"),
                value: literal!("#339966")
            },
            Style {
                name: literal!("display"),
                value: literal!("none")
            }
        ],
        tags,
        outTags,
    );
    outTags
}

fn dumpMatching2(
    mut v: metamodelica::Array<i32>,
    mut i: i32,
    mut len: i32,
    mut prefixId: &ArcStr,
    mut inTags: metamodelica::List<metamodelica::Ref<Tag>>,
) -> metamodelica::List<metamodelica::Ref<Tag>> {
    let mut outTags: metamodelica::List<metamodelica::Ref<Tag>>;
    let mut eqn: i32;
    let mut s: ArcStr;
    let mut s2: ArcStr;
    match '__try0: {
        let true = (intLe(i, len)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        s = intString(i);
        eqn = ({
            let __elt = (*unwrap_break_err!(metamodelica::index_checked(&v.borrow(), i), '__try0)).clone();
            __elt
        });
        s2 = intString(eqn);
        s = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Variable <a href=\"#"));
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("varanker"));
            __mm_s.push_str(&*s);
            __mm_s.push_str(&*literal!("\" onclick=\"return show('"));
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("variables');\">"));
            __mm_s.push_str(&*s);
            __mm_s.push_str(&*literal!("</a> is solved in equation  <a href=\"#"));
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("eqanker"));
            __mm_s.push_str(&*s2);
            __mm_s.push_str(&*literal!("\" onclick=\"return show('"));
            __mm_s.push_str(&*prefixId);
            __mm_s.push_str(&*literal!("equations');\">"));
            __mm_s.push_str(&*s2);
            __mm_s.push_str(&*literal!("</a>"));
            ArcStr::from(__mm_s)
        };
        outTags = dumpMatching2(
            v.clone(),
            i + 1,
            len,
            prefixId,
            metamodelica::cons(metamodelica::Ref::new(Tag::LINE { text: s.clone() }), inTags.clone()),
        );
        Ok::<_, &'static str>((outTags.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outTags = __try0_o0;
        }
        Err(_) => {
            outTags = inTags.clone();
        }
    }
    outTags
}

pub(crate) fn dumpMatrixHTML(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut rowNames: &metamodelica::List<ArcStr>,
    mut columNames: &metamodelica::List<ArcStr>,
    mut fileName: &ArcStr,
) -> Result<()> {
    let mut size: i32;
    let mut rowIdx: i32 = 0;
    let mut colIdx: i32 = 0;
    let mut matrixMargin: i32;
    let mut blockSize: i32;
    let mut row: metamodelica::List<i32>;
    let mut blockDraw: ArcStr;
    let mut rowLabelDraw: ArcStr;
    let mut colLabelDraw: ArcStr;
    let mut scripts: metamodelica::List<ArcStr>;
    let mut rowLabelScripts: metamodelica::List<ArcStr>;
    let mut colLabelScripts: metamodelica::List<ArcStr>;
    let mut doc: Document;
    let mut canvas: metamodelica::Ref<Tag>;
    matrixMargin = 100;
    blockSize = 20;
    scripts = metamodelica::nil();
    rowLabelScripts = metamodelica::nil();
    colLabelScripts = metamodelica::nil();
    scripts = metamodelica::cons(
        literal!("var ctx = document.querySelector('canvas').getContext('2d');\n"),
        scripts,
    );
    scripts = metamodelica::cons(literal!("ctx.fillStyle = '#001D4B';\n"), scripts);
    scripts = metamodelica::cons(literal!("ctx.font=\"18px Arial\";\n\n"), scripts);
    scripts = metamodelica::cons(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("var blockSize = "));
            __mm_s.push_str(&*intString(blockSize));
            __mm_s.push_str(&*literal!(";\n"));
            ArcStr::from(__mm_s)
        },
        scripts,
    );
    scripts = metamodelica::cons(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("var matrixMargin = "));
            __mm_s.push_str(&*intString(matrixMargin));
            __mm_s.push_str(&*literal!(";\n\n"));
            ArcStr::from(__mm_s)
        },
        scripts,
    );
    scripts = metamodelica::cons(
        literal!(
            "\nfunction drawRectangle(px, py, blockSize, margin, ctx) {\n   ctx.fillRect(((py-1)*blockSize) + matrixMargin,((px-1)*blockSize) + matrixMargin, blockSize, blockSize);\n   return ctx;\n     }\n\nfunction rowName(name, rowIdx, blockSize, margin, ctx) {\n   ctx.strokeText(name, 0, 18+margin+(rowIdx-1)*blockSize, margin);\n   return ctx;\n     }\n\nfunction colName(name, colIdx, blockSize, margin, ctx) {\n   ctx.strokeText(name, 0, 18+margin+(colIdx-1)*blockSize, margin);\n   return ctx;\n     }\n\nfunction makeLines(blockSize, margin,  n,  ctx) {\n     for (var x = 0; x < n+1; ++x) {\n     ctx.beginPath();\n     ctx.moveTo( x*blockSize + margin, margin);\n     ctx.lineTo( x*blockSize + margin, margin + (n)*blockSize);\n     ctx.stroke();\n     }\n\n\n    for (var x = 0; x < n+1; ++x) {\n     ctx.beginPath();\n     ctx.moveTo(margin, x*blockSize + margin);\n     ctx.lineTo(margin + (n)*blockSize, x*blockSize + margin);\n     ctx.stroke();\n    }\n\n  return ctx;\n  }\n  "
        ),
        scripts,
    );
    size = metamodelica::arrayLength(m.clone());
    for mut rowIdx in 1..=size {
        row = metamodelica::arrayGet(m.clone(), rowIdx)?;
        rowLabelDraw = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("ctx = rowName(\"eq_"));
            __mm_s.push_str(&*(rowNames).get(rowIdx)?);
            __mm_s.push_str(&*literal!("\", "));
            __mm_s.push_str(&*intString(rowIdx));
            __mm_s.push_str(&*literal!(", blockSize, matrixMargin, ctx);\n"));
            ArcStr::from(__mm_s)
        };
        rowLabelScripts = metamodelica::cons(rowLabelDraw, rowLabelScripts);
        colLabelDraw = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("ctx = colName(\"var_"));
            __mm_s.push_str(&*(columNames).get(rowIdx)?);
            __mm_s.push_str(&*literal!("\", "));
            __mm_s.push_str(&*intString(rowIdx));
            __mm_s.push_str(&*literal!(", blockSize, matrixMargin, ctx);\n"));
            ArcStr::from(__mm_s)
        };
        colLabelScripts = metamodelica::cons(colLabelDraw, colLabelScripts);
        for mut colIdx in &*row {
            let mut colIdx = colIdx.clone();
            if colIdx > 0 {
                blockDraw = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("ctx = drawRectangle("));
                    __mm_s.push_str(&*intString(rowIdx));
                    __mm_s.push_str(&*literal!(", "));
                    __mm_s.push_str(&*intString(colIdx));
                    __mm_s.push_str(&*literal!(",blockSize, matrixMargin,  ctx);\n"));
                    ArcStr::from(__mm_s)
                };
                scripts = metamodelica::cons(blockDraw, scripts);
            }
        }
    }
    scripts = listAppend(rowLabelScripts, scripts);
    scripts = metamodelica::cons(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "\n  ctx.textAlign = 'right';\n\n  ctx = makeLines(blockSize, matrixMargin, "
            ));
            __mm_s.push_str(&*intString(size));
            __mm_s.push_str(&*literal!(", ctx);\n"));
            ArcStr::from(__mm_s)
        },
        scripts,
    );
    scripts = metamodelica::cons(literal!("ctx.rotate(-Math.PI / 2);\n"), scripts);
    scripts = listAppend(colLabelScripts, scripts);
    doc = emptyDocumentWithToggleFunktion();
    canvas = metamodelica::Ref::new(Tag::CANVAS {
        attr: list![
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("width = \""));
                __mm_s.push_str(&*intString(size * blockSize + matrixMargin));
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            },
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" height = \""));
                __mm_s.push_str(&*intString(size * blockSize + matrixMargin));
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            }
        ],
    });
    doc = addScriptBody(
        literal!("LANGUAGE=\"JavaScript"),
        List::fold(&scripts, &fnptr!(stringAppend, ArcStr, ArcStr), literal!(""))?,
        doc,
    );
    doc = addHeadTag(canvas, doc);
    dumpDocument(doc, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fileName);
        __mm_s.push_str(&*literal!(".html"));
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

fn intAbsGt(mut i1: i32, mut i2: i32) -> bool {
    let mut out: bool;
    out = intGt(intAbs(i1), intAbs(i2));
    out
}
