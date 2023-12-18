// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Profile

use core::cmp::Ordering;

use nostr::secp256k1::XOnlyPublicKey;

use crate::Metadata;

/// Profile
#[derive(Debug, Clone)]
pub struct Profile {
    public_key: XOnlyPublicKey,
    metadata: Metadata,
}

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.public_key == other.public_key
    }
}

impl Eq for Profile {}

impl PartialOrd for Profile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Profile {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.public_key == other.public_key {
            Ordering::Equal
        } else {
            self.name().to_lowercase().cmp(&other.name().to_lowercase())
        }
    }
}

impl From<XOnlyPublicKey> for Profile {
    fn from(public_key: XOnlyPublicKey) -> Self {
        Self::new(public_key, Metadata::default())
    }
}

impl Profile {
    /// Compose new profile
    pub fn new(public_key: XOnlyPublicKey, metadata: Metadata) -> Self {
        Self {
            public_key,
            metadata,
        }
    }

    /// Get profile public key
    pub fn public_key(&self) -> XOnlyPublicKey {
        self.public_key
    }

    /// Get profile metadata
    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    /// Get profile name
    ///
    /// Steps (go to next step if field is `None` or `empty`):
    /// * Check `display_name` field
    /// * Check `name` field
    /// * Return cutted public key (ex. `00000000:00000002`)
    pub fn name(&self) -> String {
        if let Some(display_name) = &self.metadata.display_name {
            if !display_name.is_empty() {
                return display_name.clone();
            }
        }

        if let Some(name) = &self.metadata.name {
            if !name.is_empty() {
                return name.clone();
            }
        }

        cut_public_key(self.public_key)
    }
}

/// Get the first and last 8 chars of a [`XOnlyPublicKey`]
///
/// Ex. `00000000:00000002`
pub fn cut_public_key(pk: XOnlyPublicKey) -> String {
    let pk = pk.to_string();
    format!("{}:{}", &pk[0..8], &pk[pk.len() - 8..])
}
