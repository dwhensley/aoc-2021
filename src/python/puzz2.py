from typing import List
from enum import Enum, auto
from dataclasses import dataclass
import csv

INPUT_PATH = "input/puzz2.csv"


class Direction(Enum):
    FORWARD = auto()
    DOWN = auto()
    UP = auto()


@dataclass(frozen=True)
class Movement:
    direction: Direction
    distance: int

    @staticmethod
    def from_str(s: str) -> "Movement":
        direction, distance = s.split(" ")
        distance = int(distance)
        if direction == "forward":
            return Movement(Direction.FORWARD, distance)
        elif direction == "down":
            return Movement(Direction.DOWN, distance)
        elif direction == "up":
            return Movement(Direction.UP, distance)
        else:
            raise ValueError(f"Failed to parse {direction} as a direction")


@dataclass
class SubPosition:
    aim: int = 0
    horizontal: int = 0
    depth: int = 0

    def single_move_p1(self, movement: Movement):
        if movement.direction == Direction.FORWARD:
            self.horizontal += movement.distance
        elif movement.direction == Direction.DOWN:
            self.depth += movement.distance
        elif movement.direction == Direction.UP:
            self.depth -= movement.distance
        else:
            raise ValueError("Unreachable")

    def travel_course_p1(self, movements: List[Movement]):
        for m in movements:
            self.single_move_p1(m)

    def single_move_p2(self, movement: Movement):
        if movement.direction == Direction.FORWARD:
            self.horizontal += movement.distance
            self.depth += self.aim * movement.distance
        elif movement.direction == Direction.DOWN:
            self.aim += movement.distance
        elif movement.direction == Direction.UP:
            self.aim -= movement.distance
        else:
            raise ValueError("Unreachable")

    def travel_course_p2(self, movements: List[Movement]):
        for m in movements:
            self.single_move_p2(m)


def get_input(path: str) -> List[Movement]:
    input = []
    with open(path) as f:
        rdr = csv.reader(f)
        input = [Movement.from_str(row[0]) for row in rdr]
    return input


def puzz2():
    movements = get_input(INPUT_PATH)
    sub_position = SubPosition()
    sub_position.travel_course_p1(movements)
    multiplication = sub_position.horizontal * sub_position.depth
    print(
        f"Part one course: Final (horizontal, depth) positions: ({sub_position.horizontal}, {sub_position.depth}); multiplication: {multiplication}"
    )
    sub_position = SubPosition()
    sub_position.travel_course_p2(movements)
    multiplication = sub_position.horizontal * sub_position.depth
    print(
        f"Part two course: Final (horizontal, depth) positions: ({sub_position.horizontal}, {sub_position.depth}); multiplication: {multiplication}"
    )


TEST_INPUT = [
    "forward 5",
    "down 5",
    "forward 8",
    "up 3",
    "down 8",
    "forward 2",
]


def test_part_one():
    movements = [Movement.from_str(s) for s in TEST_INPUT]
    sub_position = SubPosition()
    sub_position.travel_course_p1(movements)
    assert sub_position.horizontal * sub_position.depth == 150


def test_part_two():
    movements = [Movement.from_str(s) for s in TEST_INPUT]
    sub_position = SubPosition()
    sub_position.travel_course_p2(movements)
    assert sub_position.horizontal * sub_position.depth == 900
