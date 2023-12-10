import aoc2023
import pytest


def test_day10a():
    assert 7005 == aoc2023.day10a("data/day10.txt")


@pytest.mark.skip(reason="figure out how to find the inclosed area")
def test_day10b():
    assert 0 == aoc2023.day10b("data/day10_debug.txt")
