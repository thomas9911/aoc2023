use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            'S' => Pipe::Start,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::BendNorthEast,
            'J' => Pipe::BendNorthWest,
            '7' => Pipe::BendSouthWest,
            'F' => Pipe::BendSouthEast,
            '.' => return Err(()),
            _ => unreachable!(),
        };

        Ok(res)
    }
}

impl Pipe {
    fn allowed_to_go_right(&self, right: &Pipe) -> bool {
        match self {
            Pipe::Start | Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast => {
                [Pipe::Horizontal, Pipe::BendNorthWest, Pipe::BendSouthWest].contains(right)
            }
            _ => false,
        }
    }

    fn allowed_to_go_left(&self, left: &Pipe) -> bool {
        match self {
            Pipe::Start | Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest => {
                [Pipe::Horizontal, Pipe::BendNorthEast, Pipe::BendSouthEast].contains(left)
            }
            _ => false,
        }
    }

    fn allowed_to_go_up(&self, up: &Pipe) -> bool {
        match self {
            Pipe::Start | Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest => {
                [Pipe::Vertical, Pipe::BendSouthEast, Pipe::BendSouthWest].contains(up)
            }
            _ => false,
        }
    }

    fn allowed_to_go_down(&self, up: &Pipe) -> bool {
        match self {
            Pipe::Start | Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest => {
                [Pipe::Vertical, Pipe::BendNorthEast, Pipe::BendNorthWest].contains(up)
            }
            _ => false,
        }
    }
}

/// for once I did it properly, so x and y are like math graphing (instead of computer graphing)
/// so x goes left to right increasing, and y goes down to up increasing.
/// also the indexes are 1 based (instead of zero)
#[derive(Debug)]
struct Field {
    pipes: BTreeMap<(usize, usize), Pipe>,
    size: (usize, usize),
    start_position: Option<(usize, usize)>,
}

impl FromStr for Field {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pipes = BTreeMap::new();
        let mut start_position = None;
        for (line, y) in s.lines().rev().zip(1..) {
            for (ch, x) in line.chars().zip(1..) {
                if let Ok(pipe) = ch.try_into() {
                    if pipe == Pipe::Start {
                        start_position = Some((x, y));
                    }
                    pipes.insert((x, y), pipe);
                }
            }
        }

        let size_y = s.lines().count();
        let size_x = s
            .lines()
            .next()
            .map(|line| line.len())
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid amount of lines"))?;

        Ok(Field {
            pipes,
            size: (size_x, size_y),
            start_position,
        })
    }
}

impl Field {
    fn find_loop(&self) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        if self.start_position.is_none() {
            return path;
        }

        let starting_position = self.start_position.unwrap();
        let mut current_position = starting_position;
        let mut current_pipe = self.pipes.get(&starting_position).unwrap();

        loop {
            path.push(current_position);

            if let Some(right) = self
                .pipes
                .get(&(current_position.0 + 1, current_position.1))
            {
                if path.contains(&(current_position.0 + 1, current_position.1)) {
                    ()
                } else if current_pipe.allowed_to_go_right(right) {
                    current_pipe = right;
                    current_position = (current_position.0 + 1, current_position.1);
                    continue;
                }
            }

            if let Some(up) = self
                .pipes
                .get(&(current_position.0, current_position.1 + 1))
            {
                if path.contains(&(current_position.0, current_position.1 + 1)) {
                    ()
                } else if current_pipe.allowed_to_go_up(up) {
                    current_pipe = up;
                    current_position = (current_position.0, current_position.1 + 1);
                    continue;
                }
            }

            if let Some(x_next) = current_position.0.checked_sub(1) {
                if path.contains(&(x_next, current_position.1)) {
                    ()
                } else if let Some(left) = self.pipes.get(&(x_next, current_position.1)) {
                    if current_pipe.allowed_to_go_left(left) {
                        current_pipe = left;
                        current_position = (x_next, current_position.1);
                        continue;
                    }
                }
            }

            if let Some(y_next) = current_position.1.checked_sub(1) {
                if path.contains(&(current_position.0, y_next)) {
                    ()
                } else if let Some(down) = self.pipes.get(&(current_position.0, y_next)) {
                    if current_pipe.allowed_to_go_down(down) {
                        current_pipe = down;
                        current_position = (current_position.0, y_next);
                        continue;
                    }
                }
            }

            // we have all the paths
            break;
        }

        path
    }
}

#[pyfunction]
pub fn day10a(file_path: &str) -> PyResult<usize> {
    let text = read_to_string(file_path)?;
    let field: Field = text.parse()?;
    let path = field.find_loop();
    Ok(path.len() / 2)
}

#[pyfunction]
pub fn day10b(file_path: &str) -> PyResult<usize> {
    todo!()
}
