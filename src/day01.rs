use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[pyfunction]
pub fn day01a(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let all_chars: Vec<_> = line?.chars().filter(|ch| ch.is_numeric()).collect();
        if all_chars.is_empty() {
            return Err(PyErr::new::<PyValueError, _>("not a valid line"));
        }
        let first_char = all_chars.first().expect("checked that list is not empty");
        let last_char = all_chars.last().expect("checked that list is not empty");
        let formatted_number = format!("{}{}", first_char, last_char);
        let number = formatted_number.parse::<usize>()?;
        sum += number;
    }

    Ok(sum)
}

fn scan(line: &str, collected: &mut Vec<usize>) {
    if line.is_empty() {
        return;
    }

    for (index, digit) in DIGIT_WORDS.iter().enumerate() {
        if line.starts_with(digit) {
            collected.push(index + 1);
            let (_, rest) = line.split_at(1);
            return scan(rest, collected);
        }
    }

    for (index, digit) in DIGITS.iter().enumerate() {
        if let Some(rest) = line.strip_prefix(digit) {
            collected.push(index);
            return scan(rest, collected);
        }
    }

    let (_, rest) = line.split_at(1);
    return scan(rest, collected);
}

#[pyfunction]
pub fn day01b(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let mut collected = Vec::new();
        scan(&line?, &mut collected);

        if collected.is_empty() {
            return Err(PyErr::new::<PyValueError, _>("not a valid line"));
        }
        let first_char = collected.first().expect("checked that list is not empty");
        let last_char = collected.last().expect("checked that list is not empty");
        let formatted_number = format!("{}{}", first_char, last_char);
        let number = formatted_number.parse::<usize>()?;
        sum += number;
    }

    Ok(sum)
}
