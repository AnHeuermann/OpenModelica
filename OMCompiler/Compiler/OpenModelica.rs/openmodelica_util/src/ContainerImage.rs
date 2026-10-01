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

use crate::Error;
use crate::JSON;
use crate::StringUtil;
use crate::System;
use crate::Util;

/// file:        ContainerImage.mo
/// package:     ContainerImage
/// description: This file contains util functions for working with
///              OCI (Open Container Initiative) containers like Docker images or
///              Podman pods.
/// OCI container representing Docker image or Podman pod.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ContainerImage {
    /// Registry location where the image resides.
    pub host: Option<ArcStr>,
    /// Port number for the registry.
    pub port: Option<ArcStr>,
    /// Represents a user or organization.
    pub namespace: Option<ArcStr>,
    /// Image name, identifies specific image.
    pub repository: ArcStr,
    /// Identifier to specify a particular version or variant of the image.
    pub tag: Option<ArcStr>,
    /// Digest sha256
    pub digest: Option<ArcStr>,
}

impl metamodelica::gc::MMTrace for ContainerImage {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.host, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.port, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.namespace, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.repository, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tag, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.digest, __mmv)?;
        Ok(())
    }
}
impl Default for ContainerImage {
    fn default() -> Self {
        Self {
            host: Default::default(),
            port: Default::default(),
            namespace: Default::default(),
            repository: Default::default(),
            tag: Default::default(),
            digest: Default::default(),
        }
    }
}

pub type CONTAINER_IMAGE = ContainerImage;

pub(crate) const containerTool: &'static str = "docker";

pub fn parseWithArgs(
    mut containerReferenceWithArgs: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::Ref<ContainerImage>, metamodelica::List<ArcStr>)> {
    let mut image: metamodelica::Ref<ContainerImage>;
    let mut arguments: metamodelica::List<ArcStr>;
    if (containerReferenceWithArgs).is_empty() {
        Error::addCompilerError({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "Failed to parse container image reference with arguments \""
            ));
            __mm_s.push_str(&*stringDelimitList(containerReferenceWithArgs.clone(), literal!(" ")));
            __mm_s.push_str(&*literal!("\"."));
            ArcStr::from(__mm_s)
        })?;
        return Err("fail");
    }
    image = parseContainerReference((containerReferenceWithArgs).head().cloned()?)?;
    arguments = (containerReferenceWithArgs).rest()?;
    Ok((image, arguments))
}

pub(crate) fn parseContainerReference(mut containerReference: ArcStr) -> Result<metamodelica::Ref<ContainerImage>> {
    let mut image: metamodelica::Ref<ContainerImage>;
    image = (::match_deref::match_deref! { match &(Util::stringSplitAtChar(containerReference.clone(), literal!("/"))?) {
        Deref @ metamodelica::ListNode::Cons { head: host_and_port, tail: Deref @ metamodelica::ListNode::Cons { head: namespace, tail: Deref @ metamodelica::ListNode::Cons { head: repository_and_tag, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            let mut host: ArcStr;
            let mut port: Option<ArcStr>;
            let mut repository: ArcStr;
            let mut tag: Option<ArcStr>;
            (host, port) = parseContainerHostPort(host_and_port.clone())?;
            (repository, tag) = parseContainerRepository(repository_and_tag.clone())?;
            metamodelica::Ref::new(ContainerImage { host: Some(host), port: port, namespace: Some(namespace.clone()), repository: repository, tag: tag, digest: None })
        },
        Deref @ metamodelica::ListNode::Cons { head: namespace, tail: Deref @ metamodelica::ListNode::Cons { head: repository_and_tag, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut repository: ArcStr;
            let mut tag: Option<ArcStr>;
            (repository, tag) = parseContainerRepository(repository_and_tag.clone())?;
            metamodelica::Ref::new(ContainerImage { host: None, port: None, namespace: Some(namespace.clone()), repository: repository, tag: tag, digest: None })
        },
        Deref @ metamodelica::ListNode::Cons { head: repository_and_tag, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut repository: ArcStr;
            let mut tag: Option<ArcStr>;
            (repository, tag) = parseContainerRepository(repository_and_tag.clone())?;
            metamodelica::Ref::new(ContainerImage { host: None, port: None, namespace: None, repository: repository, tag: tag, digest: None })
        },
        _ => {
            Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to parse container image '")); __mm_s.push_str(&*containerReference); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(image)
}

pub fn getDigestSha(mut image: metamodelica::Ref<ContainerImage>) -> Result<metamodelica::Ref<ContainerImage>> {
    let mut image: metamodelica::Ref<ContainerImage> = image;
    let mut imageName: ArcStr = toString(&image, false)?;
    let mut cmd: ArcStr;
    let mut manifestFile: ArcStr;
    let mut manifest: metamodelica::Ref<JSON::JSON>;
    let mut descriptor: metamodelica::Ref<JSON::JSON>;
    let mut digest: metamodelica::Ref<JSON::JSON>;
    let mut digest_sha256_str: ArcStr;
    manifestFile = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*image.repository);
        __mm_s.push_str(&*literal!("_manifest.json"));
        ArcStr::from(__mm_s)
    };
    if System::regularFileExists(manifestFile.clone()) {
        System::removeFile(manifestFile.clone());
    }
    cmd = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(containerTool));
        __mm_s.push_str(&*literal!(" manifest inspect "));
        __mm_s.push_str(&*quoteForShell(imageName.clone())?);
        __mm_s.push_str(&*literal!(" -v"));
        ArcStr::from(__mm_s)
    };
    if System::systemCall(cmd, manifestFile.clone()) != 0 {
        Error::addCompilerError({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Failed to retrieve manifest of container image '"));
            __mm_s.push_str(&*imageName);
            __mm_s.push_str(&*literal!("'."));
            ArcStr::from(__mm_s)
        })?;
        Error::addCompilerNotification({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*System::readFile(manifestFile.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        })?;
        System::removeFile(manifestFile.clone());
        return Err("fail");
    }
    manifest = JSON::parseFile(manifestFile.clone())?;
    descriptor = (match &*manifest {
        JSON::OBJECT { .. } => JSON::getOrDefault(&manifest, literal!("Descriptor"), crate::JSON::interned_NULL())?,
        JSON::LIST_OBJECT { .. } => {
            JSON::getOrDefault(&manifest, literal!("Descriptor"), crate::JSON::interned_NULL())?
        }
        JSON::ARRAY { .. } => {
            Error::addCompilerError({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Container image '"));
                __mm_s.push_str(&*imageName);
                __mm_s.push_str(&*literal!(
                    "' is a multi-platform image, which isn't supported for cross compilation."
                ));
                ArcStr::from(__mm_s)
            })?;
            Error::addCompilerNotification({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "Use a reference to a single platform image, e.g. one of the images listed by `"
                ));
                __mm_s.push_str(&*arcstr::literal!(containerTool));
                __mm_s.push_str(&*literal!(" manifest inspect "));
                __mm_s.push_str(&*imageName);
                __mm_s.push_str(&*literal!("`."));
                ArcStr::from(__mm_s)
            })?;
            System::removeFile(manifestFile.clone());
            return Err("fail");
        }
        _ => {
            Error::addCompilerError({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "Failed to retrieve manifest descriptor of container image '"
                ));
                __mm_s.push_str(&*imageName);
                __mm_s.push_str(&*literal!("'."));
                ArcStr::from(__mm_s)
            })?;
            System::removeFile(manifestFile.clone());
            return Err("fail");
        }
    });
    digest = (match &*descriptor {
        JSON::OBJECT { .. } => JSON::getOrDefault(&descriptor, literal!("digest"), crate::JSON::interned_NULL())?,
        JSON::LIST_OBJECT { .. } => JSON::getOrDefault(&descriptor, literal!("digest"), crate::JSON::interned_NULL())?,
        _ => crate::JSON::interned_NULL(),
    });
    digest_sha256_str = (match &*digest {
        JSON::STRING { r#str: __digest_str } => __digest_str.clone(),
        _ => {
            Error::addCompilerError({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "Failed to retrieve digest SHA from manifest of container image '"
                ));
                __mm_s.push_str(&*imageName);
                __mm_s.push_str(&*literal!("'."));
                ArcStr::from(__mm_s)
            })?;
            System::removeFile(manifestFile.clone());
            return Err("fail");
        }
    });
    if !(StringUtil::startsWith(digest_sha256_str.clone(), literal!("sha256:"))) {
        Error::addCompilerError({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Retrieve digest 256-SHA has unexpected format: '"));
            __mm_s.push_str(&*digest_sha256_str);
            __mm_s.push_str(&*literal!("'."));
            ArcStr::from(__mm_s)
        })?;
        System::removeFile(manifestFile.clone());
        return Err("fail");
    }
    assign_field!(image.digest = Some(digest_sha256_str));
    System::removeFile(manifestFile);
    Ok(image)
}

pub fn isTrustedOpenModelicaImage(mut image: &metamodelica::Ref<ContainerImage>) -> Result<(bool, bool)> {
    let mut isOpenModelicaImage: bool = false;
    let mut hasKnownDigest: bool = false;
    let mut isKnownHost: bool;
    let mut isKnownNamespace: bool;
    let mut isKnownTag: bool;
    let mut host: ArcStr;
    isKnownHost = (::match_deref::match_deref! { match &(image.host.clone()) {
        None => false,
        Some(Deref @ "docker.io") => false,
        Some(Deref @ "ghcr.io") => true,
        Some(__esc_host) => {
            host = (*__esc_host).clone();
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using container registry \"")); __mm_s.push_str(&*host); __mm_s.push_str(&*literal!("\". Make sure you trust the registry.")); ArcStr::from(__mm_s) })?;
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isKnownNamespace = (::match_deref::match_deref! { match &((image.namespace.clone(), isKnownHost)) {
        (Some(Deref @ "openmodelica"), true) => true,
        (_, true) => {
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Container image \"")); __mm_s.push_str(&*toString(image, false)?); __mm_s.push_str(&*literal!("\" is an external image. Make sure you trust the image.")); ArcStr::from(__mm_s) })?;
            false
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOpenModelicaImage = (::match_deref::match_deref! { match &((image.repository.clone(), isKnownNamespace)) {
        (Deref @ "crossbuild", true) => true,
        (_, true) => {
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Container image \"")); __mm_s.push_str(&*toString(image, false)?); __mm_s.push_str(&*literal!("\" is not a known OpenModelica image.")); ArcStr::from(__mm_s) })?;
            false
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isKnownTag = (::match_deref::match_deref! { match &((image.tag.clone(), isOpenModelicaImage)) {
        (Some(Deref @ "v1.28.0"), true) => true,
        (_, true) => {
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Container image \"")); __mm_s.push_str(&*toString(image, false)?); __mm_s.push_str(&*literal!("\" is not tested for this OpenModelica version.")); ArcStr::from(__mm_s) })?;
            false
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasKnownDigest = (::match_deref::match_deref! { match &((image.digest.clone(), isKnownTag)) {
        (Some(Deref @ "sha256:7f0038259e8de276384dc1a0d7297f947e8f3dc4457c08d5dced32f2e49599d8"), true) => {
            true
        },
        (Some(digest), true) => {
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Container image \"")); __mm_s.push_str(&*toString(image, false)?); __mm_s.push_str(&*literal!("\" has unknown digest \"")); __mm_s.push_str(&*digest); __mm_s.push_str(&*literal!("\".")); ArcStr::from(__mm_s) })?;
            Error::addCompilerNotification(literal!("Check https://github.com/OpenModelica/openmodelica-crossbuild/pkgs/container/crossbuild/ for available cross-build images managed by OpenModelica."))?;
            false
        },
        (None, true) => {
            Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Container image \"")); __mm_s.push_str(&*toString(image, false)?); __mm_s.push_str(&*literal!("\" has no digest. That shouldn't be possible.")); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((isOpenModelicaImage, hasKnownDigest))
}

pub fn pull(mut image: &metamodelica::Ref<ContainerImage>) -> Result<()> {
    let mut pullLogFile: ArcStr;
    let mut imageName: ArcStr = toString(image, false)?;
    let mut cmd: ArcStr;
    pullLogFile = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*image.repository);
        __mm_s.push_str(&*literal!("_pull.log"));
        ArcStr::from(__mm_s)
    };
    cmd = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(containerTool));
        __mm_s.push_str(&*literal!(" pull "));
        __mm_s.push_str(&*quoteForShell(imageName.clone())?);
        ArcStr::from(__mm_s)
    };
    if System::systemCall(cmd, pullLogFile.clone()) != 0 {
        Error::addCompilerError({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Failed to pull container image '"));
            __mm_s.push_str(&*imageName);
            __mm_s.push_str(&*literal!("'."));
            ArcStr::from(__mm_s)
        })?;
        Error::addCompilerNotification({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*System::readFile(pullLogFile.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        })?;
        System::removeFile(pullLogFile.clone());
        return Err("fail");
    }
    Error::addCompilerNotification({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*System::readFile(pullLogFile.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    })?;
    System::removeFile(pullLogFile);
    Ok(())
}

pub fn pullCommand(mut image: &metamodelica::Ref<ContainerImage>) -> Result<ArcStr> {
    let mut cmd: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(containerTool));
        __mm_s.push_str(&*literal!(" pull "));
        __mm_s.push_str(&*quoteForShell(toString(image, false)?)?);
        ArcStr::from(__mm_s)
    };
    Ok(cmd)
}

pub fn isAvailableLocally(mut image: &metamodelica::Ref<ContainerImage>) -> Result<bool> {
    let mut isAvailable: bool;
    let mut inspectLogFile: ArcStr;
    let mut imageName: ArcStr = toString(image, true)?;
    let mut cmd: ArcStr;
    inspectLogFile = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*image.repository);
        __mm_s.push_str(&*literal!("_inspect.log"));
        ArcStr::from(__mm_s)
    };
    if System::regularFileExists(inspectLogFile.clone()) {
        System::removeFile(inspectLogFile.clone());
    }
    cmd = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(containerTool));
        __mm_s.push_str(&*literal!(" image inspect "));
        __mm_s.push_str(&*quoteForShell(imageName)?);
        ArcStr::from(__mm_s)
    };
    isAvailable = System::systemCall(cmd, inspectLogFile.clone()) == 0;
    if System::regularFileExists(inspectLogFile.clone()) {
        System::removeFile(inspectLogFile);
    }
    Ok(isAvailable)
}

pub fn isCosignAvailable() -> Result<bool> {
    let mut hasCosign: bool;
    let mut cosignLogFile: ArcStr = literal!("cosign_version.log");
    if System::regularFileExists(cosignLogFile.clone()) {
        System::removeFile(cosignLogFile.clone());
    }
    hasCosign = System::systemCall(literal!("cosign version"), cosignLogFile.clone()) == 0;
    if !(hasCosign) {
        Error::addCompilerWarning(literal!(
            "Can't find `cosign` from sigstore in PATH. Signatures of container images can't be verified."
        ))?;
        Error::addCompilerNotification(literal!(
            "Install cosign from https://github.com/sigstore/cosign to verify and automatically download container images."
        ))?;
    }
    if System::regularFileExists(cosignLogFile.clone()) {
        System::removeFile(cosignLogFile);
    }
    Ok(hasCosign)
}

pub fn assertSignature(mut image: &metamodelica::Ref<ContainerImage>) -> Result<()> {
    let mut cosignLogFile: ArcStr;
    let mut cmd: ArcStr;
    let mut imageName: ArcStr = toString(image, false)?;
    let mut imageReference: ArcStr = toString(image, true)?;
    cosignLogFile = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*image.repository);
        __mm_s.push_str(&*literal!("_signature.log"));
        ArcStr::from(__mm_s)
    };
    if System::regularFileExists(cosignLogFile.clone()) {
        System::removeFile(cosignLogFile.clone());
    }
    if !(isCosignAvailable()?) {
        Error::addCompilerError({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Can't verify signature of container image '"));
            __mm_s.push_str(&*imageName);
            __mm_s.push_str(&*literal!("' without `cosign` from sigstore."));
            ArcStr::from(__mm_s)
        })?;
        return Err("fail");
    }
    cmd = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("cosign verify "));
        __mm_s.push_str(&*quoteForShell(imageReference)?);
        __mm_s.push_str(&*literal!(" --certificate-identity=https://github.com/OpenModelica/openmodelica-crossbuild/.github/workflows/publish.yml@refs/tags/v1.28.0"));
        __mm_s.push_str(&*literal!(
            " --certificate-oidc-issuer=https://token.actions.githubusercontent.com"
        ));
        ArcStr::from(__mm_s)
    };
    System::appendFile(cosignLogFile.clone(), {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*cmd);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    })?;
    if System::systemCall(cmd, cosignLogFile.clone()) != 0 {
        Error::addCompilerError({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Failed to verify signature of container image '"));
            __mm_s.push_str(&*imageName);
            __mm_s.push_str(&*literal!("'."));
            ArcStr::from(__mm_s)
        })?;
        Error::addCompilerNotification({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*System::readFile(cosignLogFile.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        })?;
        System::removeFile(cosignLogFile.clone());
        return Err("fail");
    }
    System::removeFile(cosignLogFile);
    Ok(())
}

pub fn toString(mut image: &metamodelica::Ref<ContainerImage>, mut useDigest: bool) -> Result<ArcStr> {
    let mut imageString: ArcStr = literal!("");
    imageString = hostToString(image)?;
    if !(metamodelica::stringEq(&imageString, &(literal!("")))) {
        imageString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*imageString);
            __mm_s.push_str(&*literal!("/"));
            ArcStr::from(__mm_s)
        };
    }
    imageString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*imageString);
        __mm_s.push_str(&*nameToString(image)?);
        ArcStr::from(__mm_s)
    };
    if useDigest && (image.digest).is_some() {
        imageString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*imageString);
            __mm_s.push_str(&*literal!("@"));
            __mm_s.push_str(&*Util::getOption(image.digest.clone())?);
            ArcStr::from(__mm_s)
        };
    } else if (image.tag).is_some() {
        imageString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*imageString);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*Util::getOption(image.tag.clone())?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(imageString)
}

pub(crate) fn nameToString(mut image: &metamodelica::Ref<ContainerImage>) -> Result<ArcStr> {
    let mut name: ArcStr = literal!("");
    name = (match &**image {
        ContainerImage {
            namespace: Some(namespace_str),
            repository,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*namespace_str);
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*repository);
            ArcStr::from(__mm_s)
        }
        ContainerImage {
            namespace: None,
            repository,
            ..
        } => repository.clone(),
        _ => {
            Error::addCompilerError(literal!("Failed to get name of image reference."))?;
            return Err("fail");
        }
    });
    Ok(name)
}

fn parseContainerHostPort(mut host_and_port: ArcStr) -> Result<(ArcStr, Option<ArcStr>)> {
    let mut host: ArcStr;
    let mut port: Option<ArcStr> = None;
    host = (::match_deref::match_deref! { match &(Util::stringSplitAtChar(host_and_port.clone(), literal!(":"))?) {
        Deref @ metamodelica::ListNode::Cons { head: host_str, tail: Deref @ metamodelica::ListNode::Cons { head: port_str, tail: Deref @ metamodelica::ListNode::Nil } } => {
            port = Some(port_str.clone());
            host_str.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: host_str, tail: Deref @ metamodelica::ListNode::Nil } => {
            host_str.clone()
        },
        _ => {
            Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to parse container host '")); __mm_s.push_str(&*host_and_port); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((host, port))
}

fn quoteForShell(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut quoted: ArcStr;
    quoted = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("'"));
        __mm_s.push_str(&*System::stringReplace(r#str, literal!("'"), literal!("'\\''"))?);
        __mm_s.push_str(&*literal!("'"));
        ArcStr::from(__mm_s)
    };
    Ok(quoted)
}

fn parseContainerRepository(mut repositoryString: ArcStr) -> Result<(ArcStr, Option<ArcStr>)> {
    let mut repository: ArcStr;
    let mut tag: Option<ArcStr>;
    (repository, tag) = (::match_deref::match_deref! { match &(Util::stringSplitAtChar(repositoryString.clone(), literal!(":"))?) {
        Deref @ metamodelica::ListNode::Cons { head: repository_str, tail: Deref @ metamodelica::ListNode::Cons { head: tag_str, tail: Deref @ metamodelica::ListNode::Nil } } => {
            (repository_str.clone(), Some(tag_str.clone()))
        },
        Deref @ metamodelica::ListNode::Cons { head: repository_str, tail: Deref @ metamodelica::ListNode::Nil } => {
            (repository_str.clone(), None)
        },
        _ => {
            Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to parse container image name '")); __mm_s.push_str(&*repositoryString); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((repository, tag))
}

fn hostToString(mut image: &metamodelica::Ref<ContainerImage>) -> Result<ArcStr> {
    let mut hostPortStr: ArcStr = literal!("");
    hostPortStr = (match &**image {
        ContainerImage {
            host: Some(hostStr),
            port: Some(portStr),
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*hostStr);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*portStr);
            ArcStr::from(__mm_s)
        }
        ContainerImage {
            host: Some(hostStr),
            port: None,
            ..
        } => hostStr.clone(),
        ContainerImage {
            host: None,
            port: Some(_),
            ..
        } => {
            Error::addCompilerError(literal!("Port specified without host."))?;
            return Err("fail");
        }
        ContainerImage {
            host: None, port: None, ..
        } => {
            literal!("")
        }
        _ => {
            Error::addCompilerError(literal!("Failed to get host and port of image reference."))?;
            return Err("fail");
        }
    });
    Ok(hostPortStr)
}
