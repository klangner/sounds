//! Signal generators
use std::f32::consts::PI;


/// Trait implemented by each Signal Generator
pub trait SignalGen {

    /// Return generator next sample
    fn next_sample(&mut self) -> f32;
}



/// Sinusoidal generator
pub struct SineGen {
    sample_rate: f32,
    freq: f32,
    sample_clock: f32,
}

impl SineGen {
    /// Create Sinusoidal generator with a given frequency
    pub fn new(sample_rate: u32, freq: f32) -> SineGen {
        SineGen{
            sample_rate: sample_rate as f32,
            freq,
            sample_clock: 0.0,
        }
    }
}

impl SignalGen for SineGen {

    fn next_sample(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        (self.sample_clock * self.freq * 2.0 * PI / self.sample_rate).sin()
    }

}

///// Real value periodic triangle signal (with period of 1 second).
//pub fn triangle(freq: f64) -> SignalGen<impl Fn(f64) -> Complex64> {
//    let w = 2.0 * freq;
//    SignalGen::new(move |i| Complex::new((w * (i + 0.5)) % 2. - 1., 0.))
//}
//
///// Real value periodic square signal (with period of 1 second).
//pub fn square(freq: f64) -> SignalGen<impl Fn(f64) -> Complex64> {
//    let w = freq;
//    SignalGen::new(move |i| {
//        let a = w * i % 1.;
//        let b = if a < -0.5 || (a > 0.0 && a < 0.5) {
//            1.0
//        } else {
//            -1.0
//        };
//        Complex::new(b, 0.)
//    })
//}
//
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
//
///// A real noise (without imaginary part)
//pub fn noise(std: f64) -> SignalGen<impl Fn(f64) -> Complex64> {
//    let normal = Normal::new(0.0, std);
//    SignalGen::new(move |_| {
//        Complex::new(normal.sample(&mut rand::thread_rng()), 0.0)
//    })
//}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine() {
        let mut signal = SineGen::new(44_000, 440f32);
        let sample = signal.next_sample();
        assert_eq!(sample.ceil(), 1.0);
    }

}
