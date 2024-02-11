use js_sys::Array;
use kanji_search::search_kanji_by_stroke_count;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use serde::Deserialize;

wasm_bindgen_test_configure!(run_in_browser);

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
    let result: JsValue = search_kanji_by_stroke_count(1);

    // Check if result is an object
    if result.is_object() {
        // Convert JsValue to Object
        let result_array: Array = Array::from(&result);

        let first = result_array.get(0);
        let first_kanji: Array = Array::from(&first);
        let first_kanji_name = first_kanji.get(0);

        let second = result_array.get(1);
        let second_kanji: Array = Array::from(&second);
        let second_kanji_name = second_kanji.get(0);

        assert_eq!(first_kanji_name, "一");
        assert_eq!(second_kanji_name, "乙");

    } else {
        // Handle the case when result is not an object
        panic!("Result is not an object");
    }
}

