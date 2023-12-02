use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const MAXIMUM_HAND: Hand = Hand {
    red: 12,
    green: 13,
    blue: 14,
};

struct Game {
    id: usize,
    hands: Vec<Hand>,
}

impl Game {
    pub fn get_max(&self) -> Hand {
        let mut max_hand = Hand::default();

        for hand in self.hands.iter() {
            if max_hand.red < hand.red {
                max_hand.red = hand.red;
            }

            if max_hand.green < hand.green {
                max_hand.green = hand.green;
            }

            if max_hand.blue < hand.blue {
                max_hand.blue = hand.blue;
            }
        }

        max_hand
    }
}

fn parse_hands(input: &str) -> Result<Vec<Hand>, PyErr> {
    let mut hands = Vec::new();
    for hand_text in input.split("; ") {
        let hand = hand_text.parse()?;
        hands.push(hand);
    }
    Ok(hands)
}

impl FromStr for Game {
    type Err = PyErr;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, rest) = input
            .split_once("Game ")
            .ok_or_else(|| PyErr::new::<PyValueError, _>("not a valid line"))?;

        let (id_string, rest) = rest
            .split_once(": ")
            .ok_or_else(|| PyErr::new::<PyValueError, _>("not a valid line"))?;

        let id = id_string.parse()?;
        let hands = parse_hands(rest)?;

        Ok(Game { id, hands })
    }
}

#[derive(Debug, Default, PartialEq)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl Hand {
    pub fn allowed(&self, maximum: &Hand) -> bool {
        self.red <= maximum.red && self.green <= maximum.green && self.blue <= maximum.blue
    }

    pub fn to_score(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for Hand {
    type Err = PyErr;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut hand = Hand::default();

        for item in input.split(", ") {
            let (count_text, colour) = item
                .split_once(' ')
                .ok_or_else(|| PyErr::new::<PyValueError, _>("not a valid item"))?;
            let count = count_text.parse()?;
            match colour {
                "red" => hand.red = count,
                "blue" => hand.blue = count,
                "green" => hand.green = count,
                _ => return Err(PyErr::new::<PyValueError, _>("invalid colour")),
            }
        }

        Ok(hand)
    }
}

#[pyfunction]
pub fn day02a(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let game: Game = line?.parse()?;
        if game.get_max().allowed(&MAXIMUM_HAND) {
            sum += game.id;
        }
    }

    Ok(sum)
}

#[pyfunction]
pub fn day02b(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let game: Game = line?.parse()?;
        sum += game.get_max().to_score()
    }

    Ok(sum)
}

#[pyfunction]
pub fn day02_parse_hand(input: &str) -> PyResult<(usize, usize, usize)> {
    let hand: Hand = input.parse()?;
    Ok((hand.red, hand.green, hand.blue))
}

#[pyfunction]
pub fn day02_parse_game(input: &str) -> PyResult<(usize, Vec<(usize, usize, usize)>)> {
    let game: Game = input.parse()?;

    Ok((
        game.id,
        game.hands
            .into_iter()
            .map(|hand| (hand.red, hand.green, hand.blue))
            .collect(),
    ))
}
