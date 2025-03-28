// Copyright (C) Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    ast,
    utils::duplicate_config_err,
};

/// The configuration arguments to the `#[ink::event(..)]` attribute macro.
#[derive(Debug, PartialEq, Eq)]
pub struct EventConfig {
    /// If set to `false`, a signature topic is generated and emitted for this event.
    /// If set to `true`, **no** signature topic is generated or emitted for this event.,
    /// This is the default value.
    anonymous: bool,

    /// Manually specified signature topic hash.
    signature_topic_hex: Option<String>,
}

impl TryFrom<ast::AttributeArgs> for EventConfig {
    type Error = syn::Error;

    fn try_from(args: ast::AttributeArgs) -> Result<Self, Self::Error> {
        let mut anonymous: Option<syn::LitBool> = None;
        let mut signature_topic: Option<syn::LitStr> = None;
        for arg in args.into_iter() {
            if arg.name.is_ident("anonymous") {
                if let Some(lit_bool) = anonymous {
                    return Err(duplicate_config_err(lit_bool, arg, "anonymous", "event"));
                }
                if let ast::MetaValue::Lit(syn::Lit::Bool(lit_bool)) = &arg.value {
                    anonymous = Some(lit_bool.clone())
                } else {
                    return Err(format_err_spanned!(
                        arg,
                        "expected a bool literal for `anonymous` ink! event item configuration argument",
                    ));
                }
            } else if arg.name.is_ident("signature_topic") {
                if anonymous.is_some() {
                    return Err(format_err_spanned!(
                        arg,
                        "cannot specify `signature_topic` with `anonymous` in ink! event item configuration argument",
                    ));
                }

                if let Some(lit_str) = signature_topic {
                    return Err(duplicate_config_err(lit_str, arg, "anonymous", "event"));
                }
                if let ast::MetaValue::Lit(syn::Lit::Str(lis_str)) = &arg.value {
                    signature_topic = Some(lis_str.clone())
                } else {
                    return Err(format_err_spanned!(
                        arg,
                        "expected a bool literal for `anonymous` ink! event item configuration argument",
                    ));
                }
            } else {
                return Err(format_err_spanned!(
                    arg,
                    "encountered unknown or unsupported ink! event item configuration argument",
                ));
            }
        }

        Ok(EventConfig::new(
            anonymous.map(|lit_bool| lit_bool.value).unwrap_or(false),
            signature_topic.map(|lit_str| lit_str.value()),
        ))
    }
}

impl EventConfig {
    /// Construct a new [`EventConfig`].
    pub fn new(anonymous: bool, signature_topic_hex: Option<String>) -> Self {
        Self {
            anonymous,
            signature_topic_hex,
        }
    }

    /// Returns the anonymous configuration argument.
    pub fn anonymous(&self) -> bool {
        self.anonymous
    }

    /// Returns the manually specified signature topic.
    pub fn signature_topic_hex(&self) -> Option<&str> {
        self.signature_topic_hex.as_deref()
    }
}
