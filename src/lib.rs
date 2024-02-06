use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::console; 
use web_sys::js_sys::Object;
use web_sys::js_sys::Reflect;
use std::include_str;


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

fn read_file() -> HashMap<String, KanjiData>{
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


#[wasm_bindgen]
pub fn search_kanji_by_stroke_count_for_test(stroke_count: i32) -> JsValue {
    search_kanji_by_stroke_count(stroke_count)
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

pub mod some_module {
    pub fn function_to_test() {
        // function body
    }
}

pub use some_module::function_to_test;