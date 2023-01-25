use std::fmt::{Display, Formatter, Result};

use crate::readers::*;
use crate::constants::*;

pub struct Frequency {
    pub value_hz: f64,
}

impl Frequency {
    pub fn prompt() -> Self {
        Self {
            value_hz: read_f64("Enter the frequency (in Hz): "),
        }
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Frequency: {} Hz", self.value_hz)
    }
}

// Frequency from Wavelength
// Uses formula c = λν, where
    // c is the speed of light (m/s)
    // λ is the wavelength (m)
    // ν is the frequency (Hz)
impl From<Wavelength> for Frequency {
    fn from(wavelength: Wavelength) -> Self {
        Self {
            value_hz: SPEED_OF_LIGHT / wavelength.value_m,
        }
    }
}

// Frequency from Energy
// Uses formula E = hν, where
    // E is the energy (J)
    // h is Planck's constant (J*s)
    // ν is the frequency (Hz)
impl From<Energy> for Frequency {
    fn from(energy: Energy) -> Self {
        Self {
            value_hz: energy.value_j / PLANCK,
        }
    }
}

pub struct Wavelength {
    pub value_m: f64,
}

impl Wavelength {
    pub fn prompt() -> Self {
        Self {
            value_m: read_f64("Enter the wavelength (in m): "),
        }
    }
}

impl Display for Wavelength {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Wavelength: {} m", self.value_m)
    }
}

// Wavelength from Frequency
// Uses formula c = λν, where
    // c is the speed of light (m/s)
    // λ is the wavelength (m)
    // ν is the frequency (Hz)
impl From<Frequency> for Wavelength {
    fn from(frequency: Frequency) -> Self {
        Self {
            value_m: SPEED_OF_LIGHT / frequency.value_hz,
        }
    }
}

// Wavelength from Energy
// First, uses the formula E = hν, where
    // E is the energy (J)
    // h is Planck's constant (J*s)
    // ν is the frequency (Hz)
// Then, uses the formula c = λν, where
    // c is the speed of light (m/s)
    // λ is the wavelength (m)
    // ν is the frequency (Hz)
impl From<Energy> for Wavelength {
    fn from(energy: Energy) -> Self {
        Self::from(Frequency::from(energy))
    }
}

pub struct Energy {
    pub value_j: f64,
}

impl Energy {
    pub fn prompt() -> Self {
        Self {
            value_j: read_f64("Enter the energy (in J): "),
        }
    }
}

impl Display for Energy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Energy: {} J", self.value_j)
    }
}

// Energy from Frequency
// Uses formula E = hν, where
    // E is the energy (J)
    // h is Planck's constant (J*s)
    // ν is the frequency (Hz)
impl From<Frequency> for Energy {
    fn from(frequency: Frequency) -> Self {
        Self {
            value_j: frequency.value_hz * PLANCK,
        }
    }
}

// Energy from Wavelength
// First, uses the formula c = λν, where
    // c is the speed of light (m/s)
    // λ is the wavelength (m)
    // ν is the frequency (Hz)
// Then, uses the formula E = hν, where
    // E is the energy (J)
    // h is Planck's constant (J*s)
    // ν is the frequency (Hz)
impl From<Wavelength> for Energy {
    fn from(wavelength: Wavelength) -> Self {
        Self::from(Frequency::from(wavelength))
    }
}
