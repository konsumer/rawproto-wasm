// expose a raw protobuf parser to wasm
// code mostly came from // this is from https://github.com/confio/decode_raw
// with a little wasm wrapping

use decode_raw::{try_parse_entries, Entry, EntryValue, ParseConfig};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// this wraps try_parse_entries and outputs JSON
#[wasm_bindgen]
pub fn parse_raw(bytes: &[u8], js_path: &str, js_config: &JsValue) -> JsValue {
    let config: SerdeParseConfig = js_config.into_serde().unwrap();
    let path = parse_select_query(js_path);
    let mut ret: Vec<SerdeEntry> = vec![];

    if let Some(entries) = try_parse_entries(bytes, &[], config.into()) {
        for entry in entries {
            if !entry.path.starts_with(&path) {
                continue;
            }
            ret.push(entry.into());
        }
    }

    let js_ret = JsValue::from_serde(&ret).unwrap();

    // log(&format!("bytes: {:?}", bytes));
    // log(&format!("config: {:?}", config));
    // log(&format!("js_path: {:?}", js_path));
    // log(&format!("path: {:?}", path));
    // log(&format!("ret: {:?}", ret));

    js_ret
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
    Int(u128),
    /// Wire type 2 (length delimited).
    Bytes(Vec<u8>),
    OpenNested,
    CloseNested,
}

impl From<EntryValue> for SerdeEntryValue {
    fn from(source: EntryValue) -> SerdeEntryValue {
        match source {
            EntryValue::Int(v) => SerdeEntryValue::Int(v),
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

/// parse a raw-field-query from str
pub fn parse_select_query(input: &str) -> Vec<u64> {
    let prepared = input.trim_start_matches('.');
    if prepared.len() == 0 {
        return Vec::default();
    }
    let components: Vec<&str> = prepared.split('.').collect();
    let path: Vec<u64> = components
        .into_iter()
        .map(|str| str.parse::<u64>().unwrap())
        .collect();
    path
}

#[cfg(test)]
mod tests {

    // TODO: not sure why, but these throw type annotations needed error

    // #[test]
    // fn can_parse_empty() {
    //     assert_eq!(parse_select_query(""), &[]);
    //     assert_eq!(parse_select_query("."), &[]);
    // }

    // #[test]
    // fn can_parse_simple() {
    //     assert_eq!(parse_select_query("1"), &[1]);
    //     assert_eq!(parse_select_query(".1"), &[1]);
    // }

    // #[test]
    // fn can_parse_multi() {
    //     assert_eq!(parse_select_query("1.2"), &[1, 2]);
    //     assert_eq!(parse_select_query(".1.2"), &[1, 2]);
    //     assert_eq!(parse_select_query("3.3"), &[3, 3]);
    // }
}
