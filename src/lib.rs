use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::include_str;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This code defines a struct KanjiData that represents the data structure of the kanji_data.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KanjiData {
    strokes: Option<i32>,
    grade: Option<i32>,
    freq: Option<i32>,
    jlpt_old: Option<i32>,
    jlpt_new: Option<i32>,
    meanings: Option<Vec<String>>,
    readings_on: Option<Vec<String>>,
    readings_kun: Option<Vec<String>>,
    wk_level: Option<i32>,
    wk_meanings: Option<Vec<String>>,
    wk_readings_on: Option<Vec<String>>,
    wk_readings_kun: Option<Vec<String>>,
    wk_radicals: Option<Vec<String>>,
}

// This code defines a function read_file that reads the kanjies.json file and deserializes it into a HashMap.

fn read_file() -> HashMap<String, KanjiData> {
    let json = include_str!("../data/kanjies.json");
    let kanji_data: HashMap<String, KanjiData> = serde_json::from_str(json).unwrap();
    kanji_data
}

// This code defines a function search_kanji_by_stroke_count that takes
// an input stroke_count and searches for kanji characters in the kanji_data based on the specified stroke count.
// It iterates through the kanji_data and filters out the kanji characters that have the same number of strokes as the input.
// The filtered results are stored in a new HashMap called result.
// Finally, the result is converted to a JsValue using serde_wasm_bindgen::to_value and returned.

#[wasm_bindgen]
pub extern "C" fn search_kanji_by_stroke_count(stroke_count: i32) -> JsValue {
    let kanji_data = read_file();
    let mut result = HashMap::new();
    for (key, value) in kanji_data.iter() {
        if value.strokes.unwrap() == stroke_count {
            result.insert(key.clone(), value.clone());
        }
    }
    serde_wasm_bindgen::to_value(&result).unwrap_or_else(|err| JsValue::NULL)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let _ = read_file();
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_kanji_data_has_2136_keys() {
        let kanji_data = read_file();
        let num_keys = kanji_data.len();
        assert_eq!(num_keys, 2136);
    }
}
