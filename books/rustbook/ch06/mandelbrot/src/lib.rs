mod logic;
mod utils;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! measure_elapsed_time {
    ($t:tt,$s:block) => {{
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let start = performance.now();
        let result = { $s };
        let end = performance.now();
        console_log!("{}:{}[ms]", $t, end - start);
        result
    }};
}

// JavaScript側から呼び出されることを想定
// src/logic.rsのgenerate_mandelbrot_set関数の実行時間を計測するラッパー
#[wasm_bindgen]
pub fn generate_mandelbrot_set(
    canvas_w: usize,
    canvas_h: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iter: usize,
) -> Vec<u8> {
    measure_elapsed_time!("generate:wasm\telapsed:", {
        logic::generate_mandelbrot_set(canvas_w, canvas_h, x_min, x_max, y_min, y_max, max_iter)
    })
}

