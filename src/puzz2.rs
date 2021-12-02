use std::error::Error;
use std::path::Path;

static INPUT_PATH: &str = "input/puzz2.csv";

fn get_input(path: &Path) -> Result<Vec<Movement>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&path)?;
    let mut input: Vec<Movement> = Vec::new();
    for result in rdr.records() {
        input.push(result?.as_slice().try_into()?)
    }
    Ok(input)
}

#[derive(Debug, Clone, Copy)]
enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl TryFrom<&str> for Movement {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pair = value.split(' ').collect::<Vec<&str>>();
        if pair.len() != 2 {
            return Err(String::from("Failed to parse movement pair").into());
        }
        let (direction, distance) = (pair[0], pair[1]);
        let distance = distance.parse::<i32>()?;
        match direction {
            "forward" => Ok(Movement::Forward(distance)),
            "down" => Ok(Movement::Down(distance)),
            "up" => Ok(Movement::Up(distance)),
            _ => Err(format!("Failed to parse {} as a direction", direction).into()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct SubPosition {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl SubPosition {
    fn new() -> Self {
        SubPosition {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }
    fn single_move_p1(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(v) => {
                self.horizontal += v;
            }
            Movement::Down(v) => {
                self.depth += v;
            }
            Movement::Up(v) => {
                self.depth -= v;
            }
        }
    }
    fn travel_course_p1(&mut self, movements: &[Movement]) {
        for &m in movements {
            self.single_move_p1(m)
        }
    }
    fn single_move_p2(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(v) => {
                self.horizontal += v;
                self.depth += self.aim * v;
            }
            Movement::Down(v) => {
                self.aim += v;
            }
            Movement::Up(v) => {
                self.aim -= v;
            }
        }
    }
    fn travel_course_p2(&mut self, movements: &[Movement]) {
        for &m in movements {
            self.single_move_p2(m)
        }
    }
}

pub(crate) fn puzz2() {
    let movements = get_input(Path::new(INPUT_PATH)).expect("Could not read input data");
    let mut sub_position = SubPosition::new();
    sub_position.travel_course_p1(&movements);
    let multiplication = sub_position.horizontal * sub_position.depth;
    println!(
        "Part one course: Final (horizontal, depth) positions: ({}, {}); multiplication: {}",
        sub_position.horizontal, sub_position.depth, multiplication
    );
    let mut sub_position = SubPosition::new();
    sub_position.travel_course_p2(&movements);
    let multiplication = sub_position.horizontal * sub_position.depth;
    println!(
        "Part two course: Final (horizontal, depth) positions: ({}, {}); multiplication: {}",
        sub_position.horizontal, sub_position.depth, multiplication
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_part_one() {
        let movements = TEST_INPUT
            .iter()
            .map(|&s| Movement::try_from(s))
            .collect::<Result<Vec<Movement>, Box<dyn Error>>>()
            .unwrap();
        let mut sub_position = SubPosition::new();
        sub_position.travel_course_p1(&movements);
        assert_eq!(150, sub_position.horizontal * sub_position.depth);
    }

    #[test]
    fn test_part_two() {
        let movements = TEST_INPUT
            .iter()
            .map(|&s| Movement::try_from(s))
            .collect::<Result<Vec<Movement>, Box<dyn Error>>>()
            .unwrap();
        let mut sub_position = SubPosition::new();
        sub_position.travel_course_p2(&movements);
        assert_eq!(900, sub_position.horizontal * sub_position.depth);
    }
}
