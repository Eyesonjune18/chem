use crate::constants::{RYDBERG, PLANCK, SPEED_OF_LIGHT, AVOGADRO};

// TODO: Use unicode names for variables

// Given two electron energy levels (n and m), calculates the energy released (in J)
pub fn calculate_electronic_transition_energy(n: i32, m: i32) -> f64 {
    RYDBERG * (1.0 / (n * n) as f64 - 1.0 / (m * m) as f64)
}

// Given a work function (in kJ/mol), calculates the threshold frequency of a photoelectric effect
// ? Should this be done with work -> energy -> frequency chain?
pub fn calculate_threshold_frequency(w: f64) -> f64 {
    // Convert to energy (J/photon)
    let e = (w * 1000.0) / AVOGADRO;

    // Convert energy (J/photon) to frequency (Hz)
    e / PLANCK
}

// Given a work function (in kJ/mol), calculates the threshold wavelength of a photoelectric effect
pub fn calculate_threshold_wavelength(w: f64) -> f64 {
    // Get the threshold frequency
    let ν = calculate_threshold_frequency(w);

    // Convert frequency (Hz) to wavelength (m)
    calculate_wavelength_from_frequency(ν)
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
