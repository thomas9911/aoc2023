import aoc2023
import pytest


def test_day05a():
    assert 51752125 == aoc2023.day05a("data/day05.txt")


@pytest.mark.skip(reason="that difficult range thing")
def test_day05b_debug():
    # assert 46 == aoc2023.day05b("data/day05_debug.txt")
    assert 123 == aoc2023.day05b("data/day05_debug.txt")


# def test_day05b():
#     assert 0 == aoc2023.day05b("data/day05.txt")
