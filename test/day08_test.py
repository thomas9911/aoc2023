import aoc2023
import pytest


def test_day08a():
    assert 14257 == aoc2023.day08a("data/day08.txt")


@pytest.mark.skip(reason="brute force takes too long")
def test_day08b():
    assert 0 == aoc2023.day08b("data/day08.txt")
