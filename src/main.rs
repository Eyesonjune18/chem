#![allow(dead_code)]
#![allow(mixed_script_confusables)]

mod constants;
mod measurements;
mod readers;

use readers::*;
use measurements::*;

fn main() {
    // // * This is temporary
    // loop {
	//     println!("{}", Wavelength::from(Energy::from(WorkFunction::prompt())));
    // }

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
