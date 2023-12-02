import aoc2023


def test_parse_hand():
    assert (2, 1, 3) == aoc2023.day02_parse_hand("1 green, 2 red, 3 blue")


def test_parse_game():
    assert (4, [(3, 1, 6), (6, 3, 0), (14, 3, 15)]) == aoc2023.day02_parse_game(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
    )


def test_day02a():
    assert 1734 == aoc2023.day02a("data/day02.txt")


def test_day02b():
    assert 70387 == aoc2023.day02b("data/day02.txt")
