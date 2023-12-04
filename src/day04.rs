use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct Counter {
    table: BTreeMap<usize, usize>,
}

impl Counter {
    fn add(&mut self, number: usize, amount: usize) {
        let entry = self.table.entry(number).or_default();
        *entry += amount;
    }

    fn add_one(&mut self, number: usize) {
        self.add(number, 1)
    }

    fn get(&self, number: usize) -> Option<&usize> {
        self.table.get(&number)
    }

    fn sum(&self) -> usize {
        self.table.values().sum()
    }
}

fn find_matches(winning: &str, ours: &str) -> PyResult<usize> {
    let winning_set: HashSet<usize> = winning
        .split_ascii_whitespace()
        .map(std::str::FromStr::from_str)
        .collect::<Result<_, _>>()?;

    let mut matched: usize = 0;

    for text in ours.split_ascii_whitespace() {
        let number: usize = text.parse()?;
        if winning_set.contains(&number) {
            matched += 1;
        }
    }

    Ok(matched)
}

fn parse_line(line: &str) -> PyResult<(usize, &str, &str)> {
    let (round_text, rest) = line
        .split_once(": ")
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
    let (_, round_number_text) = round_text
        .split_once(" ")
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
    let round_number: usize = round_number_text.trim().parse()?;
    let (winning, ours) = rest
        .split_once(" | ")
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;

    Ok((round_number, winning, ours))
}

fn get_match_score(line: &str) -> PyResult<usize> {
    let (_, winning, ours) = parse_line(line)?;

    let matched = find_matches(winning, ours)? as u32;

    let score = match matched {
        0 => 0,
        1 => 1,
        other => 2usize.pow(other.saturating_sub(1)),
    };

    Ok(score)
}

fn add_to_counter(line: &str, counter: &mut Counter) -> PyResult<()> {
    let (round_number, winning, ours) = parse_line(line)?;
    let matched = find_matches(winning, ours)?;
    let multiplier = 1 + counter.get(round_number).copied().unwrap_or(0);
    counter.add_one(round_number);

    for extra_card in (round_number + 1)..=(round_number + matched) {
        counter.add(extra_card, multiplier)
    }

    Ok(())
}

#[pyfunction]
pub fn day04a(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut scores = 0;
    for line in reader.lines() {
        scores += get_match_score(&line?)?;
    }

    Ok(scores)
}

#[pyfunction]
pub fn day04b(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut counter = Counter::default();
    for line in reader.lines() {
        add_to_counter(&line?, &mut counter)?;
    }

    Ok(counter.sum())
}
