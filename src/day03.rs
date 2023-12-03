use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

type InnerTokens = HashMap<(usize, usize), SchemaToken>;

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaToken {
    Symbol,
    Number(char),
}

impl SchemaToken {
    pub fn is_symbol(&self) -> bool {
        match self {
            SchemaToken::Symbol => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            SchemaToken::Number(_) => true,
            _ => false,
        }
    }

    pub fn from_char(ch: char) -> Option<SchemaToken> {
        match ch {
            number if number.is_ascii_digit() => Some(SchemaToken::Number(number)),
            '.' => None,
            _symbol => Some(SchemaToken::Symbol),
        }
    }
}

impl IntoPy<Py<PyAny>> for SchemaToken {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            Self::Number(number) => number.into_py(py),
            Self::Symbol => String::from("*").into_py(py),
        }
    }
}

#[pyclass]
#[derive(Debug)]
pub struct Tokenizer {
    #[pyo3(get)]
    tokens: HashMap<(usize, usize), SchemaToken>,
    size: (usize, usize),
}

impl Tokenizer {
    pub fn sum_valid_numbers(&self) -> PyResult<usize> {
        let mut current_number_text = String::new();
        let mut total = 0;

        for (y, x) in (0..=self.size.1).flat_map(|a| (0..self.size.0).map(move |b| (a, b))) {
            let item = if let Some(item) = self.tokens.get(&(x, y)) {
                item
            } else {
                continue;
            };

            if let SchemaToken::Number(ch) = item {
                current_number_text.push(*ch);
            }
            // dbg!(
            //     (x, y),
            //     &item,
            //     &current_number_text,
            //     self.tokens.get(&(x + 1 , y))
            // );

            // is number and next is empty
            let next_item = self.tokens.get(&(x + 1, y));
            if item.is_number() && (next_item.is_none() || next_item.unwrap().is_symbol()) {
                if self.has_symbol_around(x - (current_number_text.len() - 1), x, y)? {
                    let number: usize = current_number_text.parse()?;
                    dbg!(number);
                    total += number;
                }

                current_number_text = String::new();
            }
        }

        Ok(total)
    }

    pub fn has_symbol_around(&self, from: usize, to: usize, y: usize) -> PyResult<bool> {
        // dbg!("here", &(from, to, y));

        // for (area_x, area_y) in (x.saturating_sub(1)..=x.saturating_add(1))
        //     .zip(y.saturating_sub(1)..=y.saturating_add(1))
        for (area_y, area_x) in (y.saturating_sub(1)..=y.saturating_add(1))
            .flat_map(|a| (from.saturating_sub(1)..=to.saturating_add(1)).map(move |b| (a, b)))
        {
            if let Some(SchemaToken::Symbol) = self.tokens.get(&(area_x, area_y)) {
                // dbg!((area_x, area_y));
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl FromStr for Tokenizer {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if let Some(item) = SchemaToken::from_char(ch) {
                    tokens.insert((x, y), item);
                }
                max_x = x;
            }
            max_y = y;
        }

        Ok(Tokenizer {
            tokens,
            size: (max_x, max_y),
        })
    }
}

#[pyfunction]
pub fn day03a(file_path: &str) -> PyResult<usize> {
    let data = std::fs::read_to_string(file_path)?;
    let data = data.trim();

    let tokenizer: Tokenizer = data.parse()?;

    tokenizer.sum_valid_numbers()
}

#[pyfunction]
pub fn day03a_parse_tokens(schema: &str) -> PyResult<Tokenizer> {
    schema.parse()
}
