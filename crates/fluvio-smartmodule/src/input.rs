use std::{collections::BTreeMap, fmt::Display};
use std::fmt;

use fluvio_protocol::record::Offset;
use fluvio_protocol::{Encoder, Decoder, record::Record};

#[derive(Debug, Default, Clone, Encoder, Decoder)]
pub struct SmartModuleExtraParams {
    inner: BTreeMap<String, String>,
}

impl From<BTreeMap<String, String>> for SmartModuleExtraParams {
    fn from(inner: BTreeMap<String, String>) -> SmartModuleExtraParams {
        SmartModuleExtraParams { inner }
    }
}

impl SmartModuleExtraParams {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }
}

/// A single SmartModule input record
#[derive(Debug, Default, Clone, Encoder, Decoder)]
pub struct SmartModuleInput {
    /// The base offset of this batch of records
    base_offset: Offset,
    /// encoded version of Record
    raw_bytes: Vec<u8>,
    /// This is deprecrated, extra parameters should not be passed, they will be removed in the future
    #[deprecated]
    params: SmartModuleExtraParams,
    #[fluvio(min_version = 16)]
    join_record: Vec<u8>,
}

impl SmartModuleInput {
    pub fn new(raw_bytes: Vec<u8>, base_offset: Offset) -> Self {
        Self {
            base_offset,
            raw_bytes,
            ..Default::default()
        }
    }

    pub fn base_offset(&self) -> Offset {
        self.base_offset
    }

    pub fn set_base_offset(&mut self, base_offset: Offset) {
        self.base_offset = base_offset;
    }

    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }

    pub fn into_raw_bytes(self) -> Vec<u8> {
        self.raw_bytes
    }

    pub fn parts(self) -> (Vec<u8>, Vec<u8>) {
        (self.raw_bytes, self.join_record)
    }
}

impl std::convert::TryFrom<Vec<Record>> for SmartModuleInput {
    type Error = std::io::Error;
    fn try_from(records: Vec<Record>) -> Result<Self, Self::Error> {
        let mut raw_bytes = Vec::new();
        records.encode(&mut raw_bytes, 0)?;
        Ok(SmartModuleInput {
            raw_bytes,
            ..Default::default()
        })
    }
}

impl Display for SmartModuleInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SmartModuleInput {{ base_offset: {:?}, record_data: {:?}, join_data: {:#?} }}",
            self.base_offset,
            self.raw_bytes.len(),
            self.join_record.len()
        )
    }
}

/// A type to pass input to an Aggregate SmartModule WASM module
#[derive(Debug, Default, Clone, Encoder, Decoder)]
pub struct SmartModuleAggregateInput {
    /// The base input required by all SmartModules
    pub base: SmartModuleInput,
    /// The current value of the Aggregate's accumulator
    pub accumulator: Vec<u8>,
}

/// Input to SmartModule Init
#[derive(Debug, Default, Clone, Encoder, Decoder)]
pub struct SmartModuleInitInput {
    pub params: SmartModuleExtraParams,
}
