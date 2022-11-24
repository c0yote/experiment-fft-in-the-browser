use js_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn sum_array(a: &[f32]) -> f32 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i];
    }
    sum
}

#[wasm_bindgen]
pub struct Analysis {
    samples: Vec<i16>,
    output: Vec<f32>,
}

#[wasm_bindgen]
impl Analysis {
    pub fn new() -> Analysis {
        Analysis {
            samples: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, sample: i16) {
        self.samples.push(sample);
    }

    pub fn add_samples(&mut self, samples: &js_sys::Int16Array) {
        self.samples = samples.to_vec();
    }

    pub fn process(&mut self) {
        for i in 0..self.samples.len() {
            self.output.push(self.samples[i] as f32);
        }
    }

    pub fn get_output(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(&self.output[..])
    }

    pub fn render(&self) -> String {
        let mut s = String::new();
        for i in 0..self.output.len() {
            s.push_str(&format!("{}, ", self.output[i]));
        }
        s
    }

    pub fn get_output_ptr(&self) -> *const f32 {
        self.output.as_ptr()
    }
}
