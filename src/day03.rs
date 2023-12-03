use pyo3::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

type InnerTokens = HashMap<(usize, usize), SchemaToken>;

#[derive(Debug, Default)]
struct Locations {
    location: (usize, usize),
    diagonal_left_up: bool,
    diagonal_right_up: bool,
    diagonal_left_down: bool,
    diagonal_right_down: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Locations {
    pub fn new(x: usize, y: usize) -> Locations {
        Locations {
            location: (x, y),
            ..Default::default()
        }
    }

    pub fn resolve(&mut self, tokens: &InnerTokens) {
        let (gear_x, gear_y) = self.location;

        // check diagonals
        self.diagonal_left_up = gear_x
            .checked_sub(1)
            .and_then(|valid_x| gear_y.checked_sub(1).map(|valid_y| (valid_x, valid_y)))
            .and_then(|(valid_x, valid_y)| tokens.get(&(valid_x, valid_y)))
            .map(|token| token.is_number())
            .unwrap_or(false);

        self.diagonal_right_up = gear_x
            .checked_add(1)
            .and_then(|valid_x| gear_y.checked_sub(1).map(|valid_y| (valid_x, valid_y)))
            .and_then(|(valid_x, valid_y)| tokens.get(&(valid_x, valid_y)))
            .map(|token| token.is_number())
            .unwrap_or(false);

        self.diagonal_left_down = gear_x
            .checked_sub(1)
            .and_then(|valid_x| gear_y.checked_add(1).map(|valid_y| (valid_x, valid_y)))
            .and_then(|(valid_x, valid_y)| tokens.get(&(valid_x, valid_y)))
            .map(|token| token.is_number())
            .unwrap_or(false);

        self.diagonal_right_down = gear_x
            .checked_add(1)
            .and_then(|valid_x| gear_y.checked_add(1).map(|valid_y| (valid_x, valid_y)))
            .and_then(|(valid_x, valid_y)| tokens.get(&(valid_x, valid_y)))
            .map(|token| token.is_number())
            .unwrap_or(false);

        // check adjacent

        self.up = gear_y
            .checked_sub(1)
            .map(|valid_y| (gear_x, valid_y))
            .and_then(|(valid_x, valid_y)| tokens.get(&(valid_x, valid_y)))
            .map(|token| token.is_number())
            .unwrap_or(false);

        self.down = gear_y
            .checked_add(1)
            .map(|valid_y| (gear_x, valid_y))
            .and_then(|(valid_x, valid_y)| tokens.get(&(valid_x, valid_y)))
            .map(|token| token.is_number())
            .unwrap_or(false);

        self.left = gear_x
            .checked_sub(1)
            .map(|valid_x| (valid_x, gear_y))
            .and_then(|(valid_x, valid_y)| tokens.get(&(valid_x, valid_y)))
            .map(|token| token.is_number())
            .unwrap_or(false);

        self.right = gear_x
            .checked_add(1)
            .map(|valid_x| (valid_x, gear_y))
            .and_then(|(valid_x, valid_y)| tokens.get(&(valid_x, valid_y)))
            .map(|token| token.is_number())
            .unwrap_or(false);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaToken {
    Symbol,
    Gear,
    Number(char),
}

impl SchemaToken {
    pub fn is_symbol(&self) -> bool {
        match self {
            SchemaToken::Symbol => true,
            SchemaToken::Gear => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            SchemaToken::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_gear(&self) -> bool {
        match self {
            SchemaToken::Gear => true,
            _ => false,
        }
    }

    pub fn as_number(&self) -> Option<char> {
        match self {
            SchemaToken::Number(ch) => Some(*ch),
            _ => None,
        }
    }

    pub fn from_char(ch: char) -> Option<SchemaToken> {
        match ch {
            number if number.is_ascii_digit() => Some(SchemaToken::Number(number)),
            '.' => None,
            '*' => Some(SchemaToken::Gear),
            _symbol => Some(SchemaToken::Symbol),
        }
    }
}

impl IntoPy<Py<PyAny>> for SchemaToken {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            Self::Number(number) => number.into_py(py),
            Self::Gear => String::from("*").into_py(py),
            Self::Symbol => String::from("#").into_py(py),
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

            let next_item = self.tokens.get(&(x + 1, y));

            // is number and next is empty or symbol
            if item.is_number() && (next_item.is_none() || next_item.unwrap().is_symbol()) {
                if self.has_symbol_around(x - (current_number_text.len() - 1), x, y)? {
                    let number: usize = current_number_text.parse()?;
                    total += number;
                }

                current_number_text = String::new();
            }
        }

        Ok(total)
    }

    pub fn has_symbol_around(&self, from: usize, to: usize, y: usize) -> PyResult<bool> {
        for (area_y, area_x) in (y.saturating_sub(1)..=y.saturating_add(1))
            .flat_map(|a| (from.saturating_sub(1)..=to.saturating_add(1)).map(move |b| (a, b)))
        {
            if let Some(true) = self.tokens.get(&(area_x, area_y)).map(|x| x.is_symbol()) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn gather_gear_locations(&self) -> PyResult<Vec<(usize, usize)>> {
        let gears = self
            .tokens
            .iter()
            .flat_map(|(location, value)| {
                if value.is_gear() {
                    Some(*location)
                } else {
                    None
                }
            })
            .collect();

        Ok(gears)
    }

    pub fn find_valid_gear_ratios(&self, gears: &[(usize, usize)]) -> PyResult<Vec<usize>> {
        let mut ratios = Vec::new();

        for (gear_x, gear_y) in gears {
            let mut locations = Locations::new(*gear_x, *gear_y);
            locations.resolve(&self.tokens);
            let mut numbers: Vec<usize> = Vec::new();

            if locations.diagonal_left_up && locations.up == false {
                let mut reverse_string = String::new();
                for x_pointer in (0..*gear_x).rev() {
                    if let Some(token) = self.tokens.get(&(x_pointer, gear_y - 1)) {
                        token.as_number().map(|x| reverse_string.push(x));
                    } else {
                        break;
                    }
                }
                let string: String = reverse_string.chars().rev().collect();
                numbers.push(string.parse()?);
            }
            if locations.diagonal_left_down && locations.down == false {
                let mut reverse_string = String::new();
                for x_pointer in (0..*gear_x).rev() {
                    if let Some(token) = self.tokens.get(&(x_pointer, gear_y + 1)) {
                        token.as_number().map(|x| reverse_string.push(x));
                    } else {
                        break;
                    }
                }
                let string: String = reverse_string.chars().rev().collect();
                numbers.push(string.parse()?);
            }

            if locations.diagonal_right_up && locations.up == false {
                let mut string = String::new();
                for x_pointer in (gear_x + 1)..=self.size.0 {
                    if let Some(token) = self.tokens.get(&(x_pointer, gear_y - 1)) {
                        token.as_number().map(|x| string.push(x));
                    } else {
                        break;
                    }
                }
                numbers.push(string.parse()?);
            }

            if locations.diagonal_right_down && locations.down == false {
                let mut string = String::new();
                for x_pointer in (gear_x + 1)..=self.size.0 {
                    if let Some(token) = self.tokens.get(&(x_pointer, gear_y + 1)) {
                        token.as_number().map(|x| string.push(x));
                    } else {
                        break;
                    }
                }
                numbers.push(string.parse()?);
            }

            if locations.left {
                let mut reverse_string = String::new();
                for x_pointer in (0..*gear_x).rev() {
                    if let Some(token) = self.tokens.get(&(x_pointer, *gear_y)) {
                        token.as_number().map(|x| reverse_string.push(x));
                    } else {
                        break;
                    }
                }
                let string: String = reverse_string.chars().rev().collect();
                numbers.push(string.parse()?);
            }

            if locations.right {
                let mut string = String::new();
                for x_pointer in (gear_x + 1)..=self.size.0 {
                    if let Some(token) = self.tokens.get(&(x_pointer, *gear_y)) {
                        token.as_number().map(|x| string.push(x));
                    } else {
                        break;
                    }
                }
                numbers.push(string.parse()?);
            }

            if locations.up {
                if locations.diagonal_left_up && locations.diagonal_right_up == false {
                    let mut reverse_string = String::new();
                    for x_pointer in (0..=*gear_x).rev() {
                        if let Some(token) = self.tokens.get(&(x_pointer, gear_y - 1)) {
                            token.as_number().map(|x| reverse_string.push(x));
                        } else {
                            break;
                        }
                    }
                    let string: String = reverse_string.chars().rev().collect();
                    numbers.push(string.parse()?);
                }
                if locations.diagonal_right_up && locations.diagonal_left_up == false {
                    let mut string = String::new();
                    for x_pointer in *gear_x..=self.size.0 {
                        if let Some(token) = self.tokens.get(&(x_pointer, gear_y - 1)) {
                            token.as_number().map(|x| string.push(x));
                        } else {
                            break;
                        }
                    }
                    numbers.push(string.parse()?);
                }
                if locations.diagonal_left_up == false && locations.diagonal_right_up == false {
                    let mut string = String::new();
                    self.tokens
                        .get(&(*gear_x, gear_y - 1))
                        .and_then(SchemaToken::as_number)
                        .map(|x| string.push(x));
                    numbers.push(string.parse()?);
                }
                // can we assume that a number is a maximum of length 3 ?
                if locations.diagonal_left_up && locations.up && locations.diagonal_right_up {
                    let mut string = String::new();
                    self.tokens
                        .get(&(*gear_x - 1, gear_y - 1))
                        .and_then(SchemaToken::as_number)
                        .map(|x| string.push(x));
                    self.tokens
                        .get(&(*gear_x, gear_y - 1))
                        .and_then(SchemaToken::as_number)
                        .map(|x| string.push(x));
                    self.tokens
                        .get(&(*gear_x + 1, gear_y - 1))
                        .and_then(SchemaToken::as_number)
                        .map(|x| string.push(x));
                    numbers.push(string.parse()?);
                }
            }

            if locations.down {
                if locations.diagonal_left_down && locations.diagonal_right_down == false {
                    let mut reverse_string = String::new();
                    for x_pointer in (0..=*gear_x).rev() {
                        if let Some(token) = self.tokens.get(&(x_pointer, gear_y + 1)) {
                            token.as_number().map(|x| reverse_string.push(x));
                        } else {
                            break;
                        }
                    }
                    let string: String = reverse_string.chars().rev().collect();
                    numbers.push(string.parse()?);
                }

                if locations.diagonal_right_down && locations.diagonal_left_down == false {
                    let mut string = String::new();
                    for x_pointer in *gear_x..=self.size.0 {
                        if let Some(token) = self.tokens.get(&(x_pointer, gear_y + 1)) {
                            token.as_number().map(|x| string.push(x));
                        } else {
                            break;
                        }
                    }
                    numbers.push(string.parse()?);
                }
                if locations.diagonal_left_down == false && locations.diagonal_right_down == false {
                    let mut string = String::new();
                    self.tokens
                        .get(&(*gear_x, gear_y + 1))
                        .and_then(SchemaToken::as_number)
                        .map(|x| string.push(x));
                    numbers.push(string.parse()?);
                }
                // can we assume that a number is a maximum of length 3 ?
                if locations.diagonal_left_down && locations.down && locations.diagonal_right_down {
                    let mut string = String::new();
                    self.tokens
                        .get(&(*gear_x - 1, gear_y + 1))
                        .and_then(SchemaToken::as_number)
                        .map(|x| string.push(x));
                    self.tokens
                        .get(&(*gear_x, gear_y + 1))
                        .and_then(SchemaToken::as_number)
                        .map(|x| string.push(x));
                    self.tokens
                        .get(&(*gear_x + 1, gear_y + 1))
                        .and_then(SchemaToken::as_number)
                        .map(|x| string.push(x));
                    numbers.push(string.parse()?);
                }
            }

            if numbers.len() == 2 {
                ratios.push(numbers[0] * numbers[1]);
            }
        }

        Ok(ratios)
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
            size: (max_x + 1, max_y + 1),
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
pub fn day03b(file_path: &str) -> PyResult<usize> {
    let data = std::fs::read_to_string(file_path)?;
    let data = data.trim();

    let tokenizer: Tokenizer = data.parse()?;
    let gear_locations = tokenizer.gather_gear_locations()?;
    let gear_ratios = tokenizer.find_valid_gear_ratios(&gear_locations)?;

    Ok(gear_ratios.into_iter().sum())
}

#[pyfunction]
pub fn day03a_parse_tokens(schema: &str) -> PyResult<Tokenizer> {
    schema.parse()
}
