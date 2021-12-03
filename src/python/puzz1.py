from typing import List
import csv
import numpy as np

INPUT_PATH = "input/puzz1.csv"


def get_input(path: str) -> List[int]:
    input = []
    with open(path) as f:
        rdr = csv.reader(f)
        input = [int(row[0]) for row in rdr]
    return input


def diff(arr: np.ndarray) -> np.ndarray:
    return np.diff(arr)


def moving_sum(arr: np.ndarray, window_size: int) -> np.ndarray:
    return np.lib.stride_tricks.sliding_window_view(arr, window_size).sum(axis=1)


def count_positive(arr: np.ndarray) -> int:
    count = 0
    for v in arr:
        if v > 0:
            count += 1
    return count


def puzz1():
    input = get_input(INPUT_PATH)
    cnt_larger = count_positive(diff(input))
    print(f"{cnt_larger} measurements larger than previous measurement")
    cnt_larger = count_positive(diff(moving_sum(input, 3)))
    print(f"{cnt_larger} values larger than the previous value in window-summed input")


TEST_INPUT = np.array([199, 200, 208, 210, 200, 207, 240, 269, 260, 263])


def test_part_one():
    count = count_positive(diff(TEST_INPUT))
    assert count == 7


def test_part_two():
    count = count_positive(diff(moving_sum(TEST_INPUT, 3)))
    assert count == 5
