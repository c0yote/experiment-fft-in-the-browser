use js_sys;
use rustfft::{FftPlanner, num_complex::Complex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FFTAnalysis {
    samples: Vec<Complex<f32>>,
    magnitudes: Vec<f32>,
    phases: Vec<f32>,
    frequencies: Vec<f32>,
    sample_rate: usize,
    channels: usize,
    frame_size: usize,
    resolution_hz: usize,
    scaled_magnitudes: Vec<f32>,
}

#[wasm_bindgen]
impl FFTAnalysis {
    pub fn new(samples: &js_sys::Int16Array, sample_rate: usize, channels: usize, resolution_hz: usize) -> FFTAnalysis {
        // Load the samples into a vector of Complex numbers.
        let mut buffer: Vec<Complex<f32>> = match channels {
            1 => { // Mono - Just copy the samples into complex numbers.
                samples.to_vec().iter().map(|s| { Complex { re: *s as f32, im: 0f32}}).collect()
            },
            2 => { // Stereo - Sum the channels to flatten to mono and copy them into complex numbers.
                samples.to_vec().chunks(2).map(|c| { Complex { re: (c[0] as f32 / 2f32) + (c[1] as f32 / 2f32), im: 0f32}}).collect()
            },
            _ => {
                panic!("Unsupported number of channels: {}", channels);
            }
        };

        // Calculate the frame size from the desired resolution.
        let frame_size = sample_rate / resolution_hz;

        // Pad with zeros to a multiple of the frame size.
        let remainder = buffer.len() % frame_size;
        if remainder != 0 {
            let zeros = vec![Complex { re: 0.0, im: 0.0 }; frame_size - remainder];
            buffer.extend(zeros);
        }

        // Perform the FFT.
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(frame_size);
        fft.process(&mut buffer);

        // Calculate the magnitude and phase.
        let mut magnitudes: Vec<f32> = Vec::with_capacity(buffer.len());
        let mut phases: Vec<f32> = Vec::with_capacity(buffer.len());
        let mut frequencies: Vec<f32> = Vec::with_capacity(buffer.len());
        for (i, sample) in buffer.iter().enumerate() {
            frequencies.push(((i as f32 % frame_size as f32) / frame_size as f32) * sample_rate as f32);
            magnitudes.push((sample.re.powi(2) + sample.im.powi(2)).sqrt());
            phases.push(sample.im.atan2(sample.re));
        }

        let mut magnitude_max = 0.0f32;

        for magnitude in magnitudes.iter() {
            if magnitude > &magnitude_max {
                magnitude_max = *magnitude;
            }
        };

        let mut scaled_magnitudes: Vec<f32> = magnitudes.iter().map(|m| { m / magnitude_max }).collect();

        // Return the analysis.
        FFTAnalysis {
            samples: buffer,
            magnitudes: magnitudes,
            phases: phases,
            frequencies: frequencies,
            sample_rate: sample_rate,
            channels: channels,
            frame_size: frame_size,
            resolution_hz: resolution_hz,
            scaled_magnitudes: scaled_magnitudes,
        }
    }

    pub fn get_magnitude(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(&self.magnitudes[..])
    }

    pub fn get_magnitude_ptr(&self) -> *const f32 {
        self.magnitudes.as_ptr()
    }

    pub fn get_scaled_magnitude_ptr(&self) -> *const f32 {
        self.scaled_magnitudes.as_ptr()
    }

    pub fn length(&self) -> usize {
        self.samples.len()
    }

    pub fn frame_size(&self) -> usize {
        self.frame_size
    }
}
