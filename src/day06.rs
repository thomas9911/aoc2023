use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs::read_to_string;
use std::str::Lines;

fn parse_line_part_one<'a>(lines: &'a mut Lines) -> PyResult<(&'a str, Vec<i32>)> {
    let (title, data_text) = lines
        .next()
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?
        .split_once(":")
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
    let data = data_text
        .split_ascii_whitespace()
        .map(|x| x.parse())
        .collect::<Result<_, _>>()?;
    Ok((title, data))
}

fn parse_line_part_two<'a>(lines: &'a mut Lines) -> PyResult<(&'a str, i64)> {
    let (title, data_text) = lines
        .next()
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?
        .split_once(":")
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
    let data = data_text.replace(" ", "").parse()?;
    Ok((title, data))
}

#[pyfunction]
pub fn day06a(file_path: &str) -> PyResult<i32> {
    let input = read_to_string(file_path)?;
    let mut lines = input.lines();
    let (first_title, time_data) = parse_line_part_one(&mut lines)?;
    if first_title != "Time" {
        return Err(PyErr::new::<PyValueError, _>("invalid Time data"));
    }
    let (second_title, distance_data) = parse_line_part_one(&mut lines)?;
    if second_title != "Distance" {
        return Err(PyErr::new::<PyValueError, _>("invalid Distance data"));
    }

    let mut score = 1;
    for (time, distance) in time_data.into_iter().zip(distance_data.into_iter()) {
        let mut i = 0;

        while ((time * i - i * i) - distance) <= 0 {
            i += 1;
        }

        let out = time - i * 2 + 1;
        score *= out;
    }

    Ok(score)
}

#[pyfunction]
pub fn day06b(file_path: &str) -> PyResult<i64> {
    let input = read_to_string(file_path)?;
    let mut lines = input.lines();
    let (first_title, time) = parse_line_part_two(&mut lines)?;
    if first_title != "Time" {
        return Err(PyErr::new::<PyValueError, _>("invalid Time data"));
    }
    let (second_title, distance) = parse_line_part_two(&mut lines)?;
    if second_title != "Distance" {
        return Err(PyErr::new::<PyValueError, _>("invalid Distance data"));
    }

    let mut i = 0;
    while ((time * i - i * i) - distance) <= 0 {
        i += 1;
    }

    let score = time - i * 2 + 1;

    Ok(score)
}
