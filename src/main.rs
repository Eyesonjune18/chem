#![allow(dead_code)]

mod constants;
mod equations;

use std::io::{stdin, stdout, Write};

use equations::{
    calculate_electronic_transition_energy,
    calculate_frequency_from_wavelength,
    calculate_threshold_frequency,
    calculate_wavelength_from_frequency,
    calculate_work_function,
};

fn main() {
    // Give the user a menu of calculations to choose from
    println!("1. Calculate energy release from electronic transition");
    println!("2. Calculate threshold frequency from work function");
    println!("3. Calculate work function from threshold frequency");
    println!("4. Calculate wavelength from frequency");
    println!("5. Calculate frequency from wavelength");

    let choice = read_i32("Enter selection: ");

    match choice {
        1 => prompt_electronic_transition_energy(),
        2 => prompt_threshold_frequency(),
        3 => prompt_work_function(),
        4 => prompt_wavelength_from_frequency(),
        5 => prompt_frequency_from_wavelength(),
        _ => println!("Invalid choice!"),
    }
}

fn prompt_electronic_transition_energy() {
    let n = read_i32("Enter the first energy level (n): ");
    let m = read_i32("Enter the second energy level (m): ");
    let significant_figures = read_usize("Enter the number of significant figures needed: ") - 1;

    let energy_difference = calculate_electronic_transition_energy(n, m);

    println!("The energy difference between the two energy levels is {:.1$e} J", energy_difference, significant_figures);
}

fn prompt_threshold_frequency() {
    // ! Is this unit correct?
    let work_function = read_f64("Enter the work function (in J/mol): ");
    let significant_figures = read_usize("Enter the number of significant figures needed: ") - 1;

    let threshold_frequency = calculate_threshold_frequency(work_function);

    println!("The threshold frequency is {:.1$e} Hz", threshold_frequency, significant_figures);
}

fn prompt_work_function() {
    let threshold_frequency = read_f64("Enter the threshold frequency (in Hz): ");
    let significant_figures = read_usize("Enter the number of significant figures needed: ") - 1;

    let work_function = calculate_work_function(threshold_frequency);

    println!("The work function is {:.1$e} J/mol", work_function, significant_figures);
}

fn prompt_wavelength_from_frequency() {
    let frequency = read_f64("Enter the frequency (in Hz): ");
    let significant_figures = read_usize("Enter the number of significant figures needed: ") - 1;

    let wavelength = calculate_wavelength_from_frequency(frequency);

    println!("The wavelength is {:.1$e} m", wavelength, significant_figures);
}

fn prompt_frequency_from_wavelength() {
    let wavelength = read_f64("Enter the wavelength (in m): ");
    let significant_figures = read_usize("Enter the number of significant figures needed: ") - 1;

    let frequency = calculate_frequency_from_wavelength(wavelength);

    println!("The frequency is {:.1$e} Hz", frequency, significant_figures);
}

fn read_i32(prompt: &str) -> i32 {
    prompt_string(prompt).parse().expect("Please type a number!")
}

fn read_usize(prompt: &str) -> usize {
    prompt_string(prompt).parse().expect("Please type a number!")
}

fn read_f64(prompt: &str) -> f64 {
    prompt_string(prompt).parse().expect("Please type a number!")
}

fn prompt_string(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
