// wrap decode_raw with wasm

use decode_raw::{is_selected, try_parse_entries, Entry, EntryValue, ParseConfig, SelectQuery};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// this wraps try_parse_entries and outputs JSON
#[wasm_bindgen]
pub fn parse_raw(bytes: &[u8], js_path: &str, js_config: &JsValue) -> JsValue {
    console_error_panic_hook::set_once();
    let query = SelectQuery::parse(js_path).unwrap();
    let config: SerdeParseConfig = js_config.into_serde().unwrap();

    match try_parse_entries(bytes, config.into()) {
        Some(entries) => {
            let mut ret: Vec<SerdeEntry> = vec![];
            for entry in entries.into_iter().filter(|e| is_selected(e, &query)) {
                ret.push(entry.into());
            }
            JsValue::from_serde(&ret).unwrap()
        }
        None => {
            panic!("Input bytes is not a valid protobuf serialization");
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SerdeEntry {
    pub path: Vec<u64>,
    pub value: SerdeEntryValue,
}

impl From<Entry> for SerdeEntry {
    fn from(source: Entry) -> SerdeEntry {
        SerdeEntry {
            path: source.path,
            value: source.value.into(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SerdeEntryValue {
    Fixed64([u8; 8]),
    Fixed32([u8; 4]),
    Varint(u128),
    /// Wire type 2 (length delimited).
    Bytes(Vec<u8>),
    OpenNested,
    CloseNested,
}

impl From<EntryValue> for SerdeEntryValue {
    fn from(source: EntryValue) -> SerdeEntryValue {
        match source {
            EntryValue::Fixed64(v) => SerdeEntryValue::Fixed64(v),
            EntryValue::Fixed32(v) => SerdeEntryValue::Fixed32(v),
            EntryValue::Varint(v) => SerdeEntryValue::Varint(v),
            EntryValue::Bytes(v) => SerdeEntryValue::Bytes(v),
            EntryValue::OpenNested => SerdeEntryValue::OpenNested,
            EntryValue::CloseNested => SerdeEntryValue::CloseNested,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SerdeParseConfig {
    pub no_fixed64: bool,
    pub no_fixed32: bool,
}

impl From<SerdeParseConfig> for ParseConfig {
    fn from(source: SerdeParseConfig) -> ParseConfig {
        ParseConfig {
            no_fixed64: source.no_fixed64,
            no_fixed32: source.no_fixed32,
        }
    }
}
