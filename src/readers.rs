use std::io::{stdin, stdout, Write};

pub fn prompt_sigfigs() -> usize {
    read_usize("Enter the number of significant figures needed: ") - 1
}

pub fn read_i32(prompt: &str) -> i32 {
    prompt_string(prompt).parse().expect("Please type a number!")
}

fn read_usize(prompt: &str) -> usize {
    prompt_string(prompt).parse().expect("Please type a number!")
}

pub fn read_f64(prompt: &str) -> f64 {
    prompt_string(prompt).parse().expect("Please type a number!")
}

pub fn read_string(prompt: &str) -> String {
    prompt_string(prompt)
}

fn prompt_string(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
