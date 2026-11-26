use std::f64::consts::PI;

pub const SAMPLE_RATE: usize = 44_100;

const FREQ_ZERO: f64 = 1200.0;
const FREQ_ONE: f64 = 2200.0;
const BIT_RATE: usize = 100;
const SAMPLES_PER_BIT: usize = SAMPLE_RATE / BIT_RATE;
const AMPLITUDE: f64 = 16000.0;

pub fn encode(data: &[u8]) -> Vec<i16> {
    let mut samples = Vec::with_capacity(
        data.len() * 8 * SAMPLES_PER_BIT
    );

    for &byte in data {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;

            let frequency = if bit == 0 {
                FREQ_ZERO
            } else {
                FREQ_ONE
            };

            for n in 0..SAMPLES_PER_BIT {
                let t = n as f64 / SAMPLE_RATE as f64;

                let sample = AMPLITUDE * (2.0 * PI * frequency * t).sin();

                samples.push(sample as i16);
            }
        }
    }

    samples
}

pub fn decode(samples: &[i16]) -> Result<Vec<u8>, &'static str> {
    if samples.len() % (SAMPLES_PER_BIT * 8) != 0 {
        return Err("Invalid number of audio samples");
    }

    let mut bytes = Vec::with_capacity(
        samples.len() / (SAMPLES_PER_BIT * 8)
    );

    for chunk in samples.chunks(SAMPLES_PER_BIT * 8) {
        let mut byte: u8 = 0;

        for symbol in chunk.chunks(SAMPLES_PER_BIT) {
            let zero_power = detect_power(symbol, FREQ_ZERO);
            let one_power = detect_power(symbol, FREQ_ONE);

            let bit = if one_power > zero_power {
                1
            } else {
                0
            };

            byte = (byte << 1) | bit;
        }

        bytes.push(byte);
    }

    Ok(bytes)
}

pub fn write_wav(
    path: &str,
    samples: &[i16],
) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;

    for &sample in samples {
        writer.write_sample(sample)?;
    }

    writer.finalize()
}

pub fn read_wav(path: &str) -> Result<Vec<i16>, hound::Error> {
    let mut reader = hound::WavReader::open(path)?;

    let spec = reader.spec();

    if spec.sample_rate != SAMPLE_RATE as u32
        || spec.channels != 1
        || spec.bits_per_sample != 16
        || spec.sample_format != hound::SampleFormat::Int
    {
        return Err(hound::Error::FormatError(
            "Unsupported audio format",
        ));
    }

    reader.samples::<i16>().collect()
}

fn detect_power(samples: &[i16], frequency: f64) -> f64 {
    let omega = 2.0 * PI * frequency / SAMPLE_RATE as f64;
    let coefficient = 2.0 * omega.cos();

    let mut s1 = 0.0;
    let mut s2 = 0.0;

    for &sample in samples {
        let s0 = sample as f64 + coefficient * s1 - s2;

        s2 = s1;
        s1 = s0;
    }

    s1 * s1 + s2 * s2 - coefficient * s1 * s2
}