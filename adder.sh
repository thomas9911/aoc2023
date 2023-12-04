#!/bin/bash


if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

DAY=$(printf "%02d" "$1")

function previous_day() {
    input_number=${DAY/^0*//}
    result=$((input_number - 1))
    result_with_zero=$(printf "%02d" "$result")
    echo "$result_with_zero"
}

function add_to_lib() {
    file_path='./src/lib.rs'
    pattern="pub mod day$(previous_day);"
    extra_text="\npub mod day$DAY;"

    # Check if the file exists
    if [ ! -e "$file_path" ]; then
        echo "File not found: $file_path"
        exit 1
    fi

    # Find the last occurrence of the pattern and add extra text
    last_occurrence=$(awk '/'"$pattern"'/ {last_match=$0} END {print last_match}' "$file_path")
    if [ -n "$last_occurrence" ]; then
        sed -i '/'"$last_occurrence"'/s/$/'"$extra_text"'/' "$file_path"
        echo "Extra text added after the last occurrence of the pattern."
    else
        echo "Pattern not found in the file."
    fi

    text_to_add="    m.add_function(wrap_pyfunction!(day${DAY}::day${DAY}a, m)?)?;
    m.add_function(wrap_pyfunction!(day${DAY}::day${DAY}b, m)?)?;"

    escaped_text=$(printf "%s\n" "$text_to_add" | sed 's/[\&/]/\\&/g')
    awk -v text="$escaped_text" '/Ok(())/ && !done {print text; done=1} 1' "$file_path" > temp_file && mv temp_file "$file_path"
}

function new_rust_file {
    content="use pyo3::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[pyfunction]
pub fn day${DAY}a(file_path: &str) -> PyResult<usize> {
    todo!()
}

#[pyfunction]
pub fn day${DAY}b(file_path: &str) -> PyResult<usize> {
    todo!()
}
"

    echo "$content" > ./src/day${DAY}.rs
}

function new_pytest_file() {
        content="import aoc2023


def test_day${DAY}a():
    assert 0 == aoc2023.day${DAY}a(\"data/day${DAY}_debug.txt\")


def test_day${DAY}b():
    assert 0 == aoc2023.day${DAY}b(\"data/day${DAY}_debug.txt\")
"

    echo "$content" > ./test/day${DAY}_test.py
}

function create_new_day() {
    touch "./data/day$DAY.txt"
    touch "./data/day${DAY}_debug.txt"
    
    new_rust_file
    add_to_lib
    new_pytest_file
}

create_new_day

# new_rust_file
    # add_to_lib
