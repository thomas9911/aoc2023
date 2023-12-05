use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ranges::{GenericRange, OperationResult, Ranges};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Bound, Range, RangeBounds};

type LinesIter = std::io::Lines<BufReader<File>>;
// type RangeList = Vec<Range<usize>>;
type RangeList = Ranges<usize>;

const START_NODE: &str = "seed";
const END_NODE: &str = "location";

#[derive(Debug)]
struct MapChain {
    maps: HashMap<String, Map>,
}

impl MapChain {
    fn from_lines(lines: &mut LinesIter) -> PyResult<MapChain> {
        let mut maps = HashMap::new();

        while let Some(line) = lines.next() {
            let line = line?;

            if line == "" {
                continue;
            }

            let map = Map::from_lines(&line, lines)?;
            maps.insert(map.from.clone(), map);
        }

        Ok(MapChain { maps })
    }

    fn walk_to_location(&self, start_from_seed: usize) -> PyResult<usize> {
        let mut current_node = self
            .maps
            .get(START_NODE)
            .ok_or_else(|| PyErr::new::<PyValueError, _>("node not found"))?;
        let mut next_seed = start_from_seed;

        while current_node.from != END_NODE {
            next_seed = current_node.convert(next_seed)?;
            if current_node.to == END_NODE {
                break;
            }

            current_node = self
                .maps
                .get(&current_node.to)
                .ok_or_else(|| PyErr::new::<PyValueError, _>("node not found"))?;
        }

        Ok(next_seed)
    }

    fn walk_range_to_location(
        &self,
        start_from_seed_range: GenericRange<usize>,
    ) -> PyResult<RangeList> {
        let mut current_node = self
            .maps
            .get(START_NODE)
            .ok_or_else(|| PyErr::new::<PyValueError, _>("node not found"))?;
        let mut next_seed_ranges = RangeList::from(start_from_seed_range);

        while current_node.from != END_NODE {
            next_seed_ranges = current_node.convert_ranges(next_seed_ranges)?;
            if current_node.to == END_NODE {
                break;
            }

            current_node = self
                .maps
                .get(&current_node.to)
                .ok_or_else(|| PyErr::new::<PyValueError, _>("node not found"))?;
        }

        Ok(next_seed_ranges)
    }
}

#[derive(Debug, Default)]
struct Map {
    from: String,
    to: String,
    entries: Vec<MapEntry>,
}

impl Map {
    fn from_lines(first_line: &str, lines: &mut LinesIter) -> PyResult<Map> {
        let mut map = Map::default();

        // parse name
        let (mapping, _) = first_line
            .split_once(" ")
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
        let (from, to) = mapping
            .split_once("-to-")
            .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;
        map.from = from.to_string();
        map.to = to.to_string();

        while let Some(line) = lines.next() {
            let line = line?;

            if line == "" {
                return Ok(map);
            }

            let (source, destination, amount) =
                if let Some((source_txt, destination_txt, amount_txt)) =
                    line.split_once(" ").and_then(|(destination, rest)| {
                        rest.split_once(" ")
                            .map(|(source, amount)| (source, destination, amount))
                    })
                {
                    (
                        source_txt.parse()?,
                        destination_txt.parse()?,
                        amount_txt.parse()?,
                    )
                } else {
                    return Err(PyErr::new::<PyValueError, _>("invalid line"));
                };

            let new_entry = MapEntry::new(source, destination, amount);
            map.entries.push(new_entry);
        }

        Ok(map)
    }

    fn convert(&self, source: usize) -> PyResult<usize> {
        for entry in self.entries.iter() {
            if entry.source.contains(&source) {
                let offset = source - entry.source.start;
                return Ok(entry.destination.start + offset);
            }
        }

        // not found return itself
        Ok(source)
    }

    fn convert_ranges(&self, ranges: RangeList) -> PyResult<RangeList> {
        // let mut output: Vec<usize> = Vec::new();
        // for entry in self.entries.iter() {
        //     // for range in ranges {

        //     // }

        // }
        let source_ranges = self.source_ranges();

        let intersect = (source_ranges.clone()) & (ranges.clone());
        let difference = source_ranges ^ ranges;
        let mut output = difference;

        // map intersect to correct output range

        for entry in self.entries.iter() {
            for range in intersect.as_slice() {
                // let intersect = (entry.source.clone()) & (ranges.clone());
                let intersection = range.clone() & GenericRange::from(entry.source.clone());

                match intersection {
                    OperationResult::Empty => (),
                    OperationResult::Single(found_range) => {
                        let source = deref_bound(found_range.start_bound());
                        let destination = deref_bound(found_range.end_bound());

                        let offset_start =
                            bound_to_inner(sub_off_bound(source, entry.source.start));
                        let offset_end =
                            bound_to_inner(sub_off_bound(destination, entry.source.start));

                        dbg!(offset_start, offset_end);

                        let final_source = entry.destination.start + offset_start;
                        let final_destination = entry.destination.end + offset_end;

                        // let final_source = deref_bound(found_range.start_bound());
                        // let final_destination = deref_bound(found_range.end_bound());

                        // let updated_source = add_to_bound(final_source, offset);
                        // let updated_destination = add_to_bound(final_destination, offset);

                        output.insert(final_source..final_destination);
                    }
                    _ => unreachable!(),
                }
            }
        }

        // dbg!(&intersect);

        Ok(RangeList::from(output))
    }

    fn source_ranges(&self) -> RangeList {
        self.entries
            .iter()
            .map(|x| GenericRange::from(x.source.clone()))
            .collect()
    }
}

fn deref_bound<T: Copy>(bound: Bound<&T>) -> Bound<T> {
    match bound {
        Bound::Excluded(x) => Bound::Excluded(*x),
        Bound::Included(x) => Bound::Included(*x),
        Bound::Unbounded => Bound::Unbounded,
    }
}

fn add_to_bound<T: std::ops::Add<Output = T>>(bound: Bound<T>, number: T) -> Bound<T> {
    match bound {
        Bound::Excluded(x) => Bound::Excluded(x + number),
        Bound::Included(x) => Bound::Included(x + number),
        Bound::Unbounded => Bound::Unbounded,
    }
}

fn sub_off_bound<T: std::ops::Sub<Output = T>>(bound: Bound<T>, number: T) -> Bound<T> {
    match bound {
        Bound::Excluded(x) => Bound::Excluded(x - number),
        Bound::Included(x) => Bound::Included(x - number),
        Bound::Unbounded => Bound::Unbounded,
    }
}

fn bound_to_inner(bound: Bound<usize>) -> usize {
    match bound {
        Bound::Excluded(x) => x,
        Bound::Included(x) => x,
        Bound::Unbounded => usize::MAX,
    }
}

#[derive(Debug)]
struct MapEntry {
    source: Range<usize>,
    destination: Range<usize>,
}

impl MapEntry {
    pub fn new(source: usize, destination: usize, amount: usize) -> MapEntry {
        MapEntry {
            source: source..(source + amount),
            destination: destination..(destination + amount),
        }
    }
}

fn parse_single_seeds(text: &str) -> PyResult<Vec<usize>> {
    let (_, numbers_text) = text
        .split_once(": ")
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;

    numbers_text.split(" ").map(|x| Ok(x.parse()?)).collect()
}

fn parse_range_seeds(text: &str) -> PyResult<RangeList> {
    let (_, numbers_text) = text
        .split_once(": ")
        .ok_or_else(|| PyErr::new::<PyValueError, _>("invalid line"))?;

    let list_of_numbers: Vec<_> = numbers_text
        .split(" ")
        .map(|x| Ok::<_, PyErr>(x.parse()?))
        .collect::<Result<_, _>>()?;

    let mut ranges = Vec::new();
    for items in list_of_numbers.chunks(2) {
        if items.len() != 2 {
            return Err(PyErr::new::<PyValueError, _>("invalid seeds range"));
        }

        let start = items[0];
        let amount = items[1];
        ranges.push(start..(start + amount))
    }

    Ok(RangeList::from(ranges))
}

#[pyfunction]
pub fn day05a(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();
    let seeds_text = lines_iter
        .next()
        .ok_or_else(|| PyErr::new::<PyValueError, _>("empty file"))?;
    let seeds = parse_single_seeds(&seeds_text?)?;

    let map_chain = MapChain::from_lines(&mut lines_iter)?;

    let mut minimum = usize::MAX;

    for seed in seeds {
        let location_number = map_chain.walk_to_location(seed)?;
        if location_number < minimum {
            minimum = location_number;
        }
    }

    Ok(minimum)
}

#[pyfunction]
pub fn day05b(file_path: &str) -> PyResult<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();
    let seeds_text = lines_iter
        .next()
        .ok_or_else(|| PyErr::new::<PyValueError, _>("empty file"))?;
    let seeds = parse_range_seeds(&seeds_text?)?;

    let map_chain = MapChain::from_lines(&mut lines_iter)?;

    let mut minimum = usize::MAX;

    for seed_range in seeds.as_slice() {
        let location_ranges = map_chain.walk_range_to_location(seed_range.clone())?;
        let location_number = location_ranges
            .as_slice()
            .iter()
            .inspect(|x| {
                dbg!(x);
            })
            .map(|range| bound_to_inner(deref_bound(range.start_bound())))
            .min()
            .unwrap_or(usize::MAX);

        if location_number < minimum {
            minimum = location_number;
        }
    }

    Ok(minimum)
}
