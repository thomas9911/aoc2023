use lazy_static::lazy_static;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

lazy_static! {
    static ref CARDS: Vec<char> = "23456789TJQKA".chars().collect();
}

#[derive(Debug, Eq, Ord)]
struct Play {
    hand: [usize; 5],
    original_hand: [usize; 5],
    bid: usize,
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_rank().partial_cmp(&other.as_rank())
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Play {
    pub fn as_rank(&self) -> (usize, [usize; 5]) {
        match &self {
            x if x.as_five_of_a_kind().is_some() => (6, x.original_hand),
            x if x.as_four_of_a_kind().is_some() => (5, x.original_hand),
            x if x.as_full_house().is_some() => (4, x.original_hand),
            x if x.as_three_of_a_kind().is_some() => (3, x.original_hand),
            x if x.as_two_pair().is_some() => (2, x.original_hand),
            x if x.as_one_pair().is_some() => (1, x.original_hand),
            x => (0, x.original_hand),
        }
    }

    fn as_five_of_a_kind(&self) -> Option<[usize; 5]> {
        let first_card = &self.hand[0];
        if self.hand.iter().all(|x| x == first_card) {
            Some(self.hand)
        } else {
            None
        }
    }

    fn as_four_of_a_kind(&self) -> Option<[usize; 5]> {
        for (i, cards) in self.hand.windows(4).enumerate() {
            if cards[0] == cards[1] && cards[1] == cards[2] && cards[2] == cards[3] {
                if i == 0 {
                    return Some([cards[0], cards[1], cards[2], cards[3], self.hand[4]]);
                } else {
                    return Some([cards[0], cards[1], cards[2], cards[3], self.hand[0]]);
                }
            }
        }

        None
    }

    fn as_full_house(&self) -> Option<[usize; 5]> {
        let cards = self.hand;
        if cards[0] == cards[1] && cards[1] == cards[2] && cards[3] == cards[4] {
            Some([cards[0], cards[1], cards[2], cards[3], cards[4]])
        } else if cards[0] == cards[1] && cards[2] == cards[3] && cards[3] == cards[4] {
            Some([cards[3], cards[4], cards[0], cards[1], cards[2]])
        } else {
            None
        }
    }

    fn as_three_of_a_kind(&self) -> Option<[usize; 5]> {
        for (i, cards) in self.hand.windows(3).enumerate() {
            if cards[0] == cards[1] && cards[1] == cards[2] {
                match i {
                    0 => return Some([cards[0], cards[1], cards[2], self.hand[3], self.hand[4]]),
                    1 => return Some([cards[0], cards[1], cards[2], self.hand[0], self.hand[4]]),
                    2 => return Some([cards[0], cards[1], cards[2], self.hand[0], self.hand[1]]),
                    _ => unreachable!(),
                }
            }
        }

        None
    }

    fn as_two_pair(&self) -> Option<[usize; 5]> {
        if self.hand[0] == self.hand[1] && self.hand[2] == self.hand[3] {
            Some([
                self.hand[0],
                self.hand[1],
                self.hand[2],
                self.hand[3],
                self.hand[4],
            ])
        } else if self.hand[0] == self.hand[1] && self.hand[3] == self.hand[4] {
            Some([
                self.hand[0],
                self.hand[1],
                self.hand[3],
                self.hand[4],
                self.hand[2],
            ])
        } else if self.hand[1] == self.hand[2] && self.hand[3] == self.hand[4] {
            Some([
                self.hand[1],
                self.hand[2],
                self.hand[3],
                self.hand[4],
                self.hand[0],
            ])
        } else {
            None
        }
    }

    fn as_one_pair(&self) -> Option<[usize; 5]> {
        for (i, cards) in self.hand.windows(2).enumerate() {
            if cards[0] == cards[1] {
                match i {
                    0 => {
                        return Some([cards[0], cards[1], self.hand[2], self.hand[3], self.hand[4]])
                    }
                    1 => {
                        return Some([cards[0], cards[1], self.hand[0], self.hand[3], self.hand[4]])
                    }
                    2 => {
                        return Some([cards[0], cards[1], self.hand[0], self.hand[1], self.hand[4]])
                    }
                    3 => {
                        return Some([cards[0], cards[1], self.hand[0], self.hand[1], self.hand[2]])
                    }
                    _ => unreachable!(),
                }
            }
        }

        None
    }
}

impl FromStr for Play {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_text, bid_str) = s
            .split_once(" ")
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
        let mut original_hand = [0, 0, 0, 0, 0];

        for (i, ch) in hand_text.chars().enumerate() {
            original_hand[i] = CARDS
                .iter()
                .position(|n| n == &ch)
                .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid card"))?;
        }

        let mut hand = original_hand.clone();
        hand.sort_unstable_by(|a, b| b.cmp(a));

        Ok(Play {
            bid: bid_str.parse()?,
            original_hand,
            hand,
        })
    }
}

#[pyfunction]
pub fn day07_sort_cards(cards_text: &str) -> PyResult<Vec<Vec<usize>>> {
    let mut plays: Vec<Play> = Vec::new();
    for line in cards_text.lines() {
        plays.push(line.parse()?);
    }

    plays.sort();

    Ok(plays.into_iter().map(|x| x.hand.to_vec()).collect())
}

#[pyfunction]
pub fn day07a(file_path: &str) -> PyResult<usize> {
    let input = File::open(file_path)?;
    let reader = BufReader::new(input);

    let mut plays: Vec<Play> = Vec::new();
    for line in reader.lines() {
        plays.push(line?.parse()?);
    }

    plays.sort();

    let mut total = 0;

    for (index, play) in plays.into_iter().enumerate() {
        total += (index + 1) * play.bid
    }

    Ok(total)
}

#[pyfunction]
pub fn day07b(_file_path: &str) -> PyResult<usize> {
    todo!()
}
