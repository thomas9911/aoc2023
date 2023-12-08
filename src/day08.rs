use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const START: &str = "AAA";
const END: &str = "ZZZ";

struct InstructionIterator {
    instructions: Vec<char>,
    current_index: usize,
}

impl Iterator for InstructionIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.instructions.get(self.current_index) {
            self.current_index += 1;
            Some(*ch)
        } else {
            self.current_index = 0;
            self.next()
        }
    }
}

fn parse_file(
    file_path: &str,
) -> PyResult<(InstructionIterator, HashMap<String, (String, String)>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let instructions_txt = lines
        .next()
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))??;
    let instructions_iter = InstructionIterator {
        instructions: instructions_txt.chars().collect(),
        current_index: 0,
    };
    lines.next();

    let mut lookup = HashMap::new();

    for line in lines {
        let line = line?;
        let (from, left_right_txt) = line
            .split_once(" = (")
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
        let (left, right_txt) = left_right_txt
            .split_once(", ")
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
        let right = &right_txt[..(right_txt.len() - 1)];
        lookup.insert(from.to_string(), (left.to_string(), right.to_string()));
    }

    Ok((instructions_iter, lookup))
}

#[pyfunction]
pub fn day08a(file_path: &str) -> PyResult<usize> {
    let (instructions_iter, lookup) = parse_file(file_path)?;

    let mut current_key = START.to_string();
    let mut latest_index = usize::MAX;
    for (index, instruction) in instructions_iter.enumerate() {
        let value = lookup
            .get(&current_key)
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid lookup"))?;
        let new_key = if instruction == 'L' {
            &value.0
        } else {
            &value.1
        };
        current_key = new_key.to_string();

        if current_key == END {
            latest_index = index;
            break;
        }
    }

    Ok(latest_index + 1)
}

#[pyfunction]
pub fn day08b(file_path: &str) -> PyResult<usize> {
    let (instructions_iter, lookup) = parse_file(file_path)?;
    let _ = instructions_iter.instructions.len();

    let mut counter: HashMap<String, usize> = HashMap::new();
    let mut starting_points: Vec<String> = lookup
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect();
    let mut last_round = usize::MAX / 2;
    for (rounds, instruction) in instructions_iter.enumerate() {
        for points in starting_points.iter_mut() {
            let value = lookup
                .get(points)
                .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid lookup"))?;
            let new_key = if instruction == 'L' {
                &value.0
            } else {
                &value.1
            };

            let counter_entry = counter.entry(new_key.to_string()).or_default();
            *counter_entry += 1;

            *points = new_key.to_string();
        }

        if starting_points.iter().all(|x| x.ends_with('Z')) {
            last_round = rounds;
            break;
        }

        if rounds > 300 {
            dbg!(counter);
            break;
        }
    }

    Ok(last_round + 1)
}
