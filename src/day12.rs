use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = PyErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let ch = match value {
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            '?' => Spring::Unknown,
            _ => return Err(PyErr::new::<PyValueError, _>("invalid char")),
        };

        Ok(ch)
    }
}

#[derive(Debug)]
struct Row {
    numbers: Vec<usize>,
    springs: Vec<Spring>,
}

impl Row {
    fn trim_operational_springs(&mut self) {
        while let Some(Spring::Operational) = self.springs.first() {
            self.springs.remove(0);
        }

        while let Some(Spring::Operational) = self.springs.last() {
            self.springs.pop();
        }
    }
}

#[pyfunction]
pub fn day12a(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let (text, numbers_text) = line
            .split_once(" ")
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
        let numbers: Vec<usize> = numbers_text
            .split(',')
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;
        let springs: Vec<Spring> = text
            .chars()
            .map(Spring::try_from)
            .collect::<Result<_, _>>()?;

        let mut row = Row { numbers, springs };
        row.trim_operational_springs();
        dbg!(row);
    }

    todo!()
}

#[pyfunction]
pub fn day12b(file_path: &str) -> PyResult<usize> {
    todo!()
}
