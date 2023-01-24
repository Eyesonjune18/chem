use crate::constants::{RYDBERG, PLANCK, SPEED_OF_LIGHT};

// TODO: Use unicode names for variables

// Given two electron energy levels (n and m), calculates the energy released (in J)
pub fn calculate_electronic_transition_energy(n: i32, m: i32) -> f64 {
    RYDBERG * (1.0 / (n * n) as f64 - 1.0 / (m * m) as f64)
}

// Given a work function (in J/mol), calculates the threshold frequency of a photoelectric effect
pub fn calculate_threshold_frequency(w: f64) -> f64 {
    w / PLANCK
}

// Given a threshold frequency (in Hz), calculates the work function of a photoelectric effect
pub fn calculate_work_function(ν: f64) -> f64 {
    ν * PLANCK
}

// Given a frequency (in Hz), calculates the wavelength (in m)
pub fn calculate_wavelength_from_frequency(ν: f64) -> f64 {
    SPEED_OF_LIGHT / ν
}

// Given a wavelength (in m), calculates the frequency (in Hz)
pub fn calculate_frequency_from_wavelength(λ: f64) -> f64 {
    SPEED_OF_LIGHT / λ
}

// Given a frequency (in Hz), calculates the energy (in J)
pub fn calculate_energy_from_frequency(ν: f64) -> f64 {
    PLANCK * ν
}

// Given a wavelength (in m), calculates the energy (in J)
pub fn calculate_energy_from_wavelength(λ: f64) -> f64 {
    PLANCK * SPEED_OF_LIGHT / λ
}
