use std::env;

use csv;
use hound;
use rustfft::{FftPlanner, num_complex::Complex};

fn fft_wav_file(filename: &str, resolution: usize) {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let spec = reader.spec();
    let samples = reader.samples::<i16>();
    let mut buffer: Vec<Complex<f32>> = samples.map(|s| { Complex { re: s.unwrap() as f32, im: 0f32}}).collect();

    println!("    Sample rate: {}", spec.sample_rate);
    println!("       Channels: {}", spec.channels);
    println!("Bits per sample: {}", spec.bits_per_sample);
    println!("        Samples: {}", buffer.len());

    if spec.channels == 2 {
        // Sum left and right channels.
        buffer = buffer.chunks(2).map(|c| { Complex { re: (c[0].re / 2f32) + (c[1].re / 2f32), im: 0f32}}).collect();
        println!(" Summed Samples: {}", buffer.len());
    }
    else if spec.channels > 2 {
        panic!("Unsupported number of channels: {}", spec.channels);
    };

    let frame_size = (spec.sample_rate as usize) / resolution;
    println!("     Frame size: {}", frame_size);
    let remainder = buffer.len() % frame_size;
    if remainder != 0 {
        let zeros = vec![Complex { re: 0.0, im: 0.0 }; frame_size - remainder];
        buffer.extend(zeros);
    }
    println!(" Padded Samples: {}", buffer.len());

    // Run the FFT.
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(frame_size);
    fft.process(&mut buffer);

    // Calculate frequency, magnitude, and phase; then write the results to a CSV file.
    let mut writer = csv::Writer::from_path("spectrum.csv").unwrap();
    writer.write_record(&["index", "frequency", "real", "imaginary", "magnitude", "phase"]).unwrap();
    for i in 0..buffer.len() {
        let frequency = ((i as f32 % frame_size as f32) / frame_size as f32) * spec.sample_rate as f32;
        let magnitude = (buffer[i].re.powi(2) + buffer[i].im.powi(2)).sqrt();
        let phase = buffer[i].im.atan2(buffer[i].re);
        writer.write_record(&[i.to_string(), frequency.to_string(), (buffer[i].re).to_string(), (buffer[i].im).to_string(), magnitude.to_string(), phase.to_string()]).unwrap();
    }

    let mut writer = csv::Writer::from_path("specs.csv").unwrap();
    writer.write_record(&["sample-rate", "channels", "bits", "sample-count", "frame-size"]).unwrap();
    writer.write_record(&[spec.sample_rate.to_string(), spec.channels.to_string(), spec.bits_per_sample.to_string(), buffer.len().to_string(), frame_size.to_string()]).unwrap();
    
    writer.flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let resolution = match &args[2].parse::<usize>() {
        Ok(resolution) => *resolution,
        Err(_) => 2,
    };

    fft_wav_file(filename, resolution);
}
