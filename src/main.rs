#![allow(dead_code)]
#![allow(mixed_script_confusables)]

mod constants;
mod measurement_types;
mod readers;
mod equations;

use readers::*;
use measurement_types::*;

fn main() {
    // Give the user a menu of calculations to choose from
    // println!("1. Calculate energy release from electronic transition");
    println!("2. Work Function          -> Threshold Frequency");
    println!("3. Work Function          -> Threshold Wavelength");
    println!("4. Threshold Frequency    -> Work Function");
    println!("5. Frequency              -> Wavelength");
    println!("6. Wavelength             -> Frequency");
    println!("7. Frequency              -> Energy");
    println!("8. Wavelength             -> Energy");

    let choice = read_i32("Enter selection: ");

    // ! Significant figures???
    println!("{}", match choice {
        2 => Frequency::from(WorkFunction::prompt()).to_string(),
        3 => Wavelength::from(WorkFunction::prompt()).to_string(),
        5 => Wavelength::from(Frequency::prompt()).to_string(),
        6 => Frequency::from(Wavelength::prompt()).to_string(),
        7 => Energy::from(Frequency::prompt()).to_string(),
        8 => Energy::from(Wavelength::prompt()).to_string(),
        _ => String::from("Invalid choice!"),
    });
}

// fn prompt_electronic_transition_energy() {
//     let n = read_i32("Enter the first energy level (n): ");
//     let m = read_i32("Enter the second energy level (m): ");
//     let significant_figures = prompt_sigfigs();

//     let energy_difference = calculate_electronic_transition_energy(n, m);

//     println!("The energy difference between the two energy levels is {:.1$e} J", energy_difference, significant_figures);
// }
