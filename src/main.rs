#![allow(dead_code)]
#![allow(mixed_script_confusables)]

use std::fmt::{Display, Formatter};

mod constants;
mod measurements;
mod readers;

use readers::*;
use measurements::*;

fn main() {
    // Give the user a menu of calculations to choose from
    // ? How should I denote this first one?
    println!("1. Electronic Transition  -> Energy Release");
    println!("2. Work Function          -> Threshold Frequency");
    println!("3. Work Function          -> Threshold Wavelength");
    println!("4. Threshold Frequency    -> Work Function");
    println!("5. Frequency              -> Wavelength");
    println!("6. Wavelength             -> Frequency");
    println!("7. Frequency              -> Energy");
    println!("8. Wavelength             -> Energy");
    println!("9. Element                -> Electron Configuration");

    let choice = read_i32("Enter menu selection: ");

    println!("{}", match choice {
        1 => calculate_electronic_transition_energy(),
        2 => Frequency::from(WorkFunction::prompt()).to_string(),
        3 => Wavelength::from(WorkFunction::prompt()).to_string(),
        4 => WorkFunction::from(Frequency::prompt()).to_string(),
        5 => Wavelength::from(Frequency::prompt()).to_string(),
        6 => Frequency::from(Wavelength::prompt()).to_string(),
        7 => Energy::from(Frequency::prompt()).to_string(),
        8 => Energy::from(Wavelength::prompt()).to_string(),
        9 => {
            let element = read_string("Enter the element symbol: ");
            match get_electron_configuration(&element) {
                Some(config) => config,
                None => String::from("Invalid element symbol."),
            }
        }
        _ => String::from("Invalid choice, try again."),
    });
}

// ! Not sure how best to do this
// Given two electron energy levels (n and m), calculates the energy released (in J)
pub fn calculate_electronic_transition_energy() -> String {
    let n = read_i32("Enter the first energy level (n): ");
    let m = read_i32("Enter the second energy level (m): ");
    let significant_figures = prompt_sigfigs();

    let energy_release = constants::RYDBERG * (1.0 / (n * n) as f64 - 1.0 / (m * m) as f64);

    format!("The energy released is {:.1$e} J", energy_release, significant_figures)
}

// Takes an atomic number and returns the electron configuration as a string
fn get_electron_configuration(element: &str) -> Option<String> {
    enum ShellType {
        S,
        P,
        D,
        F,
    }

    struct Shell {
        shell_type: ShellType,
        electrons: u32,
    }

    impl Shell {
        fn new(shell_type: ShellType) -> Self {
            Self {
                shell_type,
                electrons: 0,
            }
        }

        // Gets all possible shells for a given n, and populates them with electrons
        fn from_principle_value(n: u32, remaining_electrons: &mut u32) -> Vec<Shell> {
            let mut shells = Self::get_possible_shells(n);

            for shell in &mut shells {
                let max_electrons = shell.max_electrons();

                if *remaining_electrons >= max_electrons {
                    shell.electrons = max_electrons;
                    *remaining_electrons -= max_electrons;
                } else {
                    shell.electrons = *remaining_electrons;
                    *remaining_electrons = 0;
                }
            }

            shells
        }

        // Gets all possible shells for a given n
        fn get_possible_shells(n: u32) -> Vec<Shell> {
            use ShellType::*;

            let mut shells: Vec<Shell> = Vec::new();

            if n > 0 {
                shells.push(Shell::new(S));
            }

            if n > 1 {
                shells.push(Shell::new(P));
            }

            if n > 2 {
                shells.push(Shell::new(D));
            }

            if n > 3 {
                shells.push(Shell::new(F));
            }

            shells
        }

        // Gets the maximum number of electrons for a given shell type
        fn max_electrons(&self) -> u32 {
            use ShellType::*;

            match self.shell_type {
                S => 2,
                P => 6,
                D => 10,
                F => 14,
            }
        }
    }

    impl Display for Shell {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}{}", match self.shell_type {
                ShellType::S => "s",
                ShellType::P => "p",
                ShellType::D => "d",
                ShellType::F => "f",
            }, self.electrons)
        }
    }

    // Represents all the shells in a given energy level (n)
    struct EnergyLevel {
        n: u32,
        shells: Vec<Shell>,
    }

    impl EnergyLevel {
        fn new(n: u32, remaining_electrons: &mut u32) -> Self {
            Self {
                n,
                shells: Shell::from_principle_value(n, remaining_electrons),
            }
        }
    }

    impl Display for EnergyLevel {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut output = String::new();

            for shell in &self.shells {
                if shell.electrons > 0 {
                    output.push_str(&format!("{}{} ", self.n, shell));
                }
            }

            write!(f, "{}", output)
        }
    }

    let mut electrons_remaining = constants::atomic_number(element).expect("Invalid element used");
    let mut energy_levels: Vec<EnergyLevel> = Vec::new();
    let mut n = 1;

    while electrons_remaining > 0 {
        energy_levels.push(EnergyLevel::new(n, &mut electrons_remaining));
        n += 1;
    }

    let mut electron_configuration = String::new();

    for energy_level in energy_levels {
        electron_configuration.push_str(&format!("{}", energy_level));
    }

    // Remove the trailing space and return
    Some(electron_configuration.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electron_configuration() {
        assert_eq!(get_electron_configuration("H"), Some(String::from("1s1")));
        assert_eq!(get_electron_configuration("He"), Some(String::from("1s2")));
        assert_eq!(get_electron_configuration("Li"), Some(String::from("1s2 2s1")));
        assert_eq!(get_electron_configuration("Be"), Some(String::from("1s2 2s2")));
        assert_eq!(get_electron_configuration("B"), Some(String::from("1s2 2s2 2p1")));
        assert_eq!(get_electron_configuration("C"), Some(String::from("1s2 2s2 2p2")));
        assert_eq!(get_electron_configuration("N"), Some(String::from("1s2 2s2 2p3")));
        assert_eq!(get_electron_configuration("O"), Some(String::from("1s2 2s2 2p4")));
        assert_eq!(get_electron_configuration("F"), Some(String::from("1s2 2s2 2p5")));
        assert_eq!(get_electron_configuration("Ne"), Some(String::from("1s2 2s2 2p6")));
        assert_eq!(get_electron_configuration("Na"), Some(String::from("1s2 2s2 2p6 3s1")));
        assert_eq!(get_electron_configuration("Mg"), Some(String::from("1s2 2s2 2p6 3s2")));
        assert_eq!(get_electron_configuration("Al"), Some(String::from("1s2 2s2 2p6 3s2 3p1")));
        assert_eq!(get_electron_configuration("Si"), Some(String::from("1s2 2s2 2p6 3s2 3p2")));
        assert_eq!(get_electron_configuration("P"), Some(String::from("1s2 2s2 2p6 3s2 3p3")));
        assert_eq!(get_electron_configuration("S"), Some(String::from("1s2 2s2 2p6 3s2 3p4")));
    }
}
