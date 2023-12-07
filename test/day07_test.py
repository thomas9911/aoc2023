import aoc2023


def test_day07_sort_cards():
    cards = """32T3K 765
T55J5 684
44344 71
44K44 89
KK677 28
22222 2
333KA 87
444QA 74
KTJJT 220
45678 92
KK999 947
KKK99 814
56789 10
KKK8A 85
QQQJA 483"""

    assert [
        [6, 5, 4, 3, 2],
        [7, 6, 5, 4, 3],
        [11, 8, 1, 1, 0],
        [11, 9, 9, 8, 8],
        [11, 11, 5, 5, 4],
        [12, 11, 1, 1, 1],
        [12, 10, 2, 2, 2],
        [9, 8, 3, 3, 3],
        [12, 10, 10, 10, 9],
        [12, 11, 11, 11, 6],
        [11, 11, 7, 7, 7],
        [11, 11, 11, 7, 7],
        [2, 2, 2, 2, 1],
        [11, 2, 2, 2, 2],
        [0, 0, 0, 0, 0],
    ] == aoc2023.day07_sort_cards(cards)


def test_day07a():
    assert 0 == aoc2023.day07a("data/day07.txt")


def test_day07b():
    assert 0 == aoc2023.day07b("data/day07_debug.txt")
