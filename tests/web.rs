use std::collections::HashMap;

use js_sys::Array;
use kanji_search::search_kanji_by_stroke_count;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

pub fn get_mock_kanji_data() -> JsValue {
    // Create a HashMap to hold the kanji data
    let mut kanji_data = HashMap::new();

    // Define the first kanji data
    let kanji1 = KanjiData {
        strokes: Some(1),
        grade: Some(1),
        freq: Some(2),
        jlpt_old: Some(4),
        jlpt_new: Some(5),
        meanings: Some(vec!["One".to_string(), "One Radical (no.1)".to_string()]),
        readings_on: Some(vec!["いち".to_string(), "いつ".to_string()]),
        readings_kun: Some(vec!["ひと-".to_string(), "ひと.つ".to_string()]),
        wk_level: Some(1),
        wk_meanings: Some(vec!["One".to_string()]),
        wk_readings_on: Some(vec!["いち".to_string(), "いつ".to_string()]),
        wk_readings_kun: Some(vec!["!ひと".to_string()]),
        wk_radicals: Some(vec!["Ground".to_string()]),
    };
    kanji_data.insert("一".to_string(), kanji1);

    // Define the second kanji data
    let second_kanji = KanjiData {
        strokes: Some(1),
        grade: Some(8),
        freq: Some(1841),
        jlpt_old: Some(1),
        jlpt_new: Some(1),
        meanings: Some(vec![
            "The Latter".to_string(),
            "Duplicate".to_string(),
            "Strange".to_string(),
            "Witty".to_string(),
            "Fishhook Radical (no. 5)".to_string(),
        ]),
        readings_on: Some(vec!["おつ".to_string(), "いつ".to_string()]),
        readings_kun: Some(vec!["おと-".to_string(), "きのと".to_string()]),
        wk_level: Some(57),
        wk_meanings: Some(vec!["Latter".to_string(), "^B".to_string()]),
        wk_readings_on: Some(vec!["おつ".to_string(), "いつ".to_string()]),
        wk_readings_kun: Some(vec!["!おと".to_string(), "!きのと".to_string()]),
        wk_radicals: Some(vec!["Nose".to_string()]),
    };
    kanji_data.insert("乙".to_string(), second_kanji);

    // Convert the HashMap to JsValue and return it
    serde_wasm_bindgen::to_value(&kanji_data).unwrap_or_else(|err| JsValue::NULL)
}

#[derive(Deserialize, Serialize)]
struct KanjiData {
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

#[wasm_bindgen_test]
fn wasm_test_search_kanji_by_stroke_count() {
    let result: JsValue = search_kanji_by_stroke_count(9);

    // Check if result is an object
    if result.is_object() {
        // Convert JsValue to Object
        let result_array: Array = Array::from(&result);

        assert_eq!(result_array.length(), 193);
    } else {
        // Handle the case when result is not an object
        panic!("Result is not an object");
    }
}

#[wasm_bindgen_test]
fn wasm_test_search_kanji_by_stroke_count_invalid() {
    let result: JsValue = search_kanji_by_stroke_count(199);

    // Check if result is an object
    if result.is_object() {
        // Convert JsValue to Object
        let result_array: Array = Array::from(&result);

        assert_eq!(result_array.length(), 0);
    } else {
        // Handle the case when result is not an object
        panic!("Result is not an object");
    }
}

#[wasm_bindgen_test]
fn wasm_test_compare_mock_data_with_search_kanji_by_stroke_count() {
    let result: JsValue = search_kanji_by_stroke_count(1);
    let expected_result: JsValue = get_mock_kanji_data();
    let result_str = format!("{:?}", result);
    let result_x = format!("{:?}", expected_result);
    assert_eq!(result_str, result_x);

}
