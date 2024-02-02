use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use std::collections::HashMap;
use std::convert::TryInto;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console; // Add this line to import the `from_value` function from the `serde_json` crate.
use web_sys::js_sys::Object;
use web_sys::js_sys::Reflect;

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
    strokes: i32,
    grade: i32,
    freq: i32,
    jlpt_old: i32,
    jlpt_new: i32,
    meanings: Vec<String>,
    readings_on: Vec<String>,
    readings_kun: Vec<String>,
    wk_level: i32,
    wk_meanings: Vec<String>,
    wk_readings_on: Vec<String>,
    wk_readings_kun: Vec<String>,
    wk_radicals: Vec<String>,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// src/lib.rs
#[wasm_bindgen]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    console::log_1(&JsValue::from_str("Hello world!"));
    a + b
}

#[wasm_bindgen]
pub extern "C" fn search_kanji_by_stroke_count(stroke_count: i32, json_object: Object) -> JsValue {
    // An empty HashMap to store matching items
    let mut matching_kanji: HashMap<String, KanjiData> = HashMap::new();

    // Get the keys of the object
    let keys = Reflect::own_keys(&json_object).unwrap();

    // Loop through each key in the object
    for i in 0..keys.length() {
        let key = keys.get(i).as_string().unwrap();

        // Get the value (KanjiData) associated with the key
        let value = Reflect::get(&json_object, &JsValue::from_str(&key));

        // Match on the Result to handle potential errors
        if let Ok(value) = value {
            // Deserialize the value into a KanjiData struct
            let kanji_data: Result<KanjiData, _> = serde_wasm_bindgen::from_value(value);

            // Match on the Result to handle potential errors
            if let Ok(kanji_data) = kanji_data {
                // Check if the strokes match the input stroke_count
                if kanji_data.strokes == stroke_count {
                    // Insert into the HashMap if there is a match
                    matching_kanji.insert(key, kanji_data);
                }
            } 
        } 
    }

    // Serialize the result (HashMap) to JsValue
    serde_wasm_bindgen::to_value(&matching_kanji).unwrap_or_else(|err| {
        JsValue::NULL
    })
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
