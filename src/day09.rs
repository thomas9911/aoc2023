use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Line(Vec<i64>);

impl FromStr for Line {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split_ascii_whitespace()
            .map(FromStr::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Line(data))
    }
}

impl Line {
    fn figure_out_next_item(self) -> PyResult<i64> {
        let (mut next, starting_positions) = self.figure_out_starting_positions()?;

        next.0.push(0);
        let mut last_line = next;
        for start_item in starting_positions.into_iter().rev() {
            let mut row = Vec::new();
            row.push(start_item);
            for item in last_line.0.iter() {
                let last = row.last().expect("last always exist");
                row.push(last + item);
            }
            last_line = Line(row);
        }

        let last = last_line.0.last().expect("last always exist");
        Ok(*last)
    }

    fn figure_out_previous_item(self) -> PyResult<i64> {
        let (next, starting_positions) = self.figure_out_starting_positions()?;

        let mut first_number = *next
            .0
            .first()
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid first item"))?;
        for start_item in starting_positions.into_iter().rev() {
            first_number = start_item - first_number;
        }

        Ok(first_number)
    }

    fn figure_out_starting_positions(mut self) -> PyResult<(Self, Vec<i64>)> {
        let mut starting_positions = Vec::new();

        while !self.is_all_zeroes() {
            starting_positions.push(
                *(&self.0)
                    .first()
                    .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid first item"))?,
            );
            self = self.calculate_difference();
        }

        Ok((self, starting_positions))
    }

    fn calculate_difference(&self) -> Line {
        Line(
            self.0
                .iter()
                .zip(self.0.iter().skip(1))
                .map(|(left, right)| right - left)
                .collect(),
        )
    }

    fn is_all_zeroes(&self) -> bool {
        self.0.iter().all(|x| x == &0)
    }
}

#[pyfunction]
pub fn day09a(file_path: &str) -> PyResult<i64> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let data: Line = line?.parse()?;
        sum += data.figure_out_next_item()?;
    }

    Ok(sum)
}

#[pyfunction]
pub fn day09b(file_path: &str) -> PyResult<i64> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let data: Line = line?.parse()?;
        sum += data.figure_out_previous_item()?;
    }

    Ok(sum)
}
