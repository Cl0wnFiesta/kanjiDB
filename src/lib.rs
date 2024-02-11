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
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KanjiData {
    // Define your data structure here based on your JSON format
    // For example:
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

fn read_file() -> HashMap<String, KanjiData> {
    let json = include_str!("../data/kanjies.json");
    let kanji_data: HashMap<String, KanjiData> = serde_json::from_str(json).unwrap();
    kanji_data
}

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

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let _ = read_file();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::{Object, Reflect};
    use wasm_bindgen_test::*;

    #[test]
    fn test_kanji_data_has_2135_keys() {
        let kanji_data = read_file();
        let num_keys = kanji_data.len();
        assert_eq!(num_keys, 2136);
    }
    #[wasm_bindgen_test]
    fn test_search_kanji_by_stroke_count() {
        let result = search_kanji_by_stroke_count(1);
        let result: Object = result.into();
        let keys = Reflect::own_keys(&result).unwrap();
        assert_eq!(keys.length(), 2);
    }
    #[wasm_bindgen_test]
    fn test_search_kanji_by_stroke_count_and_get_name() {
        let result = search_kanji_by_stroke_count(1);
        let result: Object = result.into();
        let keys = Reflect::own_keys(&result).unwrap();
        let key = keys.get(0);
        let key = key.as_string().unwrap();
        let key = JsValue::from_str(&key); // Convert key to JsValue
        let value = Reflect::get(&result, &key).unwrap();
        let value: KanjiData = serde_wasm_bindgen::from_value(value).unwrap();
        assert_eq!(value.strokes.unwrap(), 1);
    }
}
