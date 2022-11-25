// Note: &dyn Fn(freq: f64, amp: f64, time: f64) -> f64 is the shape of a wave function 

// TODO: Functions to provide a way to multiply the frequency and amplitude of a wave
// TODO: Functions to add together waves (maybe implement fourier transforms)

use std::f64::consts::PI;

/// A sinusoidal wave. The first argument is the frequency, the second argument is the amplitude, and the third argument is the x value for the wave.
pub fn sine(freq: f64, amp: f64, time: f64) -> f64 {
    (time * PI * 2.0 * freq).sin() * amp
}

#[test]
fn test_sine() {
    assert_eq!(sine(1.0, 1.0, 0.25), 1.0);

    assert_eq!(sine(1.0, 5.0, 0.25), 5.0);
    assert_eq!(sine(2.0, 5.0, 0.125), 5.0);
    
    assert_ne!(sine(1.0, 1.0, 0.25), 2.0);
}