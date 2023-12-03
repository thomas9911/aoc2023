use pyo3::prelude::*;

pub mod day01;
pub mod day02;
pub mod day03;

/// A Python module implemented in Rust.
#[pymodule]
fn aoc2023(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(day01::day01a, m)?)?;
    m.add_function(wrap_pyfunction!(day01::day01b, m)?)?;
    m.add_function(wrap_pyfunction!(day02::day02a, m)?)?;
    m.add_function(wrap_pyfunction!(day02::day02b, m)?)?;
    m.add_function(wrap_pyfunction!(day02::day02_parse_hand, m)?)?;
    m.add_function(wrap_pyfunction!(day02::day02_parse_game, m)?)?;
    m.add_class::<day03::Tokenizer>()?;
    m.add_function(wrap_pyfunction!(day03::day03a, m)?)?;
    m.add_function(wrap_pyfunction!(day03::day03a_parse_tokens, m)?)?;
    m.add_function(wrap_pyfunction!(day03::day03b, m)?)?;
    Ok(())
}
