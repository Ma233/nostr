// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::str::FromStr;

use nostr::hashes::sha256::Hash as Sha256Hash;
use nostr::nips::nip94::FileMetadata;
use nostr::Url;
use wasm_bindgen::prelude::*;

use crate::error::{into_err, Result};
use crate::event::tag::JsImageDimensions;

#[wasm_bindgen(js_name = Aes256Gcm)]
pub struct JsAes256Gcm {
    key: String,
    iv: String,
}

impl From<JsAes256Gcm> for (String, String) {
    fn from(value: JsAes256Gcm) -> Self {
        (value.key, value.iv)
    }
}

impl From<(String, String)> for JsAes256Gcm {
    fn from(value: (String, String)) -> Self {
        Self {
            key: value.0,
            iv: value.1,
        }
    }
}

impl JsAes256Gcm {
    pub fn new(key: String, iv: String) -> Self {
        Self { key, iv }
    }
}

#[wasm_bindgen(js_name = FileMetadata)]
pub struct JsFileMetadata {
    inner: FileMetadata,
}

impl From<FileMetadata> for JsFileMetadata {
    fn from(inner: FileMetadata) -> Self {
        Self { inner }
    }
}

impl Deref for JsFileMetadata {
    type Target = FileMetadata;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[wasm_bindgen(js_class = FileMetadata)]
impl JsFileMetadata {
    pub fn new(url: &str, mime_type: String, hash: &str) -> Result<JsFileMetadata> {
        Ok(Self {
            inner: FileMetadata::new(
                Url::from_str(&url).map_err(into_err)?,
                mime_type,
                Sha256Hash::from_str(&hash).map_err(into_err)?,
            ),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn urls(&self) -> String {
        self.inner.url.to_string()
    }

    #[wasm_bindgen(getter, js_name = mimeType)]
    pub fn mime_type(&self) -> String {
        self.inner.mime_type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> String {
        self.inner.hash.to_string()
    }

    #[wasm_bindgen(getter, js_name = aes256Gcm)]
    pub fn aes_256_gcm(&self) -> Option<JsAes256Gcm> {
        self.inner.aes_256_gcm.clone().map(|t| t.into())
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> Option<f64> {
        self.inner.size.map(|n| n as f64)
    }

    #[wasm_bindgen(getter)]
    pub fn dim(&self) -> Option<JsImageDimensions> {
        self.inner.dim.map(|i| i.into())
    }

    #[wasm_bindgen(getter)]
    pub fn magnet(&self) -> Option<String> {
        self.inner.magnet.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn blurhash(&self) -> Option<String> {
        self.inner.blurhash.clone()
    }
}
