//! # Signal generators
//!
//! Those generators can be used as a sound source.
//! 
//! ## Example
//!
//! Here we will create simple sinusoidal generator with the frequency of 440Hz.
//! And next we will use it to generate few samples
//!
//! ```rust
//! use sounds::generators::sine;
//! 
//! let mut gen = sine(44_000, 440.0);
//! let sample1 = gen.next_sample();
//! let sample2 = gen.next_sample();
//! ```

use std::f32::consts::PI;
#[cfg(feature = "random")]
use rand;
#[cfg(feature = "random")]
use rand_distr::{Normal, Distribution};


/// Signal Generator. 
pub struct SignalGen<F>
where
    F: Fn(f32) -> f32,
{
    sample_rate: f32,
    sample_clock: f32,
    signal: F,
}

impl<F> SignalGen<F>
where
    F: Fn(f32) -> f32,
{
    /// Create a new generator from provided function
    pub fn new(sample_rate: u32, signal: F) -> SignalGen<F> {
        SignalGen { 
            sample_rate: sample_rate as f32, 
            sample_clock: 0.0, 
            signal 
        }
    }

    /// Generate next signal sample.
    pub fn next_sample(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        (self.signal)(self.sample_clock / self.sample_rate)
    }
}

/// Create generator of sinusoidal signal with the given frequency in Hz.
pub fn sine(sample_rate: u32, freq: f32) -> SignalGen<impl Fn(f32) -> f32> {
    let w = 2.0 * PI * freq;
    SignalGen::new(sample_rate, move |i| (i*w).sin())
}

/// Periodic triangle signal.
pub fn triangle(sample_rate: u32, freq: f32) -> SignalGen<impl Fn(f32) -> f32> {
   let w = 2.0 * freq;
   SignalGen::new(sample_rate, move |i| (i*w) % 2.0 - 1.0)
}

/// Periodic square signal.
pub fn square(sample_rate: u32, freq: f32) -> SignalGen<impl Fn(f32) -> f32> {
   let w = freq;
   SignalGen::new(
       sample_rate,
       move |i| {
        let a = w * i % 1.;
        if a < -0.5 || (a > 0.0 && a < 0.5) {
            1.0
        } else {
            -1.0
        }
    })
}

///// A chirp is a signal in which frequency increases with time.
//pub fn chirp(start_freq: f64, end_freq: f64, time: f64) -> SignalGen<impl Fn(f64) -> Complex64> {
//    let slope = (end_freq - start_freq) / time;
//    SignalGen::new(move |i| {
//        if i < 0. || i > time {
//            Complex::new(0., 0.)
//        } else {
//            let f = slope * i + start_freq;
//            let w = 2.0 * PI * f * i;
//            Complex::new(0., w).exp()
//        }
//    })
//}

/// A real noise (without imaginary part)
#[cfg(feature = "random")]
pub fn noise(sample_rate: u32) -> SignalGen<impl Fn(f32) -> f32> {
   let normal = Normal::new(0.0, 0.3);
   SignalGen::new(sample_rate, move |_| {
       normal.sample(&mut rand::thread_rng()) as f32
   })
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine() {
        let mut gen = sine(44_000, 440f32);
        let sample = gen.next_sample();
        assert_eq!(sample.ceil(), 1.0);
    }

    #[test]
    fn test_triangle() {
        let mut gen = triangle(44_000, 440f32);
        let sample = gen.next_sample();
        assert_eq!(sample.floor(), -1.0);
    }

    #[test]
    fn test_square() {
        let mut gen = square(44_000, 440f32);
        let sample = gen.next_sample();
        assert_eq!(sample.ceil(), 1.0);
    }
    
}
