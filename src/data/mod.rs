//! Record data formats and utility types

mod blob;
mod kv;
mod loader;
mod tag;

pub use self::{
    blob::Blob,
    kv::{Kv, Value},
    tag::{Tag, TagSet},
};
use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted},
    diff::{Diff, DiffExt},
    Id, Result,
};

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A record header
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    /// A unique record ID
    pub id: Id,
    /// Public set of search tags
    pub tags: BTreeSet<Tag>,
    /// The encrypted header
    sec: Encrypted<SecHeader, KeyPair>,
}

/// Distinguishes between the type of records
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum Type {
    /// Key-value mapped store
    Kv,
    /// Large binary object
    Blob,
}

/// The secret header is encrypted
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SecHeader {
    /// Record type
    pub(crate) t: Type,
    /// Total payload size
    pub(crate) size: u64,
    /// Beginning chunk markers
    pub(crate) chunks: Vec<u32>,
}

impl DetachedKey<KeyPair> for SecHeader {}

/// A record data body
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum Body {
    Kv(Kv),
    Blob(Blob),
}

impl Body {
    fn apply(&mut self, d: Diff) -> Result<()> {
        match self {
            Self::Kv(ref mut kv) => kv.apply(d),
            Self::Blob(ref mut b) => unimplemented!(),
        }
    }
}

impl DetachedKey<KeyPair> for Body {}

/// A single record in alexandria, defined by a header and body
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    /// The clear record header
    pub header: Header,
    /// A handle to the data body
    body: Encrypted<Body, KeyPair>,
}

impl Record {
    pub(crate) fn create(tags: TagSet, diff: Diff) -> Result<Self> {
        // Create the body from the diff
        let (t, mut body) = match diff {
            Diff::Map(_) => (Type::Kv, Body::Kv(Kv::new())),
            Diff::Binary(_) => unimplemented!(),
        };
        body.apply(diff)?;
        let body = Encrypted::new(body);

        // Secret header with no disk info present
        let sec = Encrypted::new(SecHeader {
            t,
            size: 0,
            chunks: vec![],
        });

        // Primary search header
        let header = Header {
            id: Id::random(),
            tags: tags.into(),
            sec,
        };

        Ok(Self { header, body })
    }

    /// Apply a diff to a record
    pub(crate) fn apply(&mut self, diff: Diff) -> Result<()> {
        match self.body.deref_mut()? {
            Body::Kv(kv) => kv.apply(diff),
            Body::Blob(b) => unimplemented!(),
        }
    }

    pub fn kv(&self) -> &Kv {
        match self.body.deref() {
            Ok(Body::Kv(ref kv)) => kv,
            _ => unimplemented!(),
        }
    }
}
