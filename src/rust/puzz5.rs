use std::error::Error;
use std::path::Path;

static INPUT_PATH: &str = "input/puzz5.txt";

fn get_horizontal_or_vertical_input(path: &Path) -> Result<Vec<Vent>, Box<dyn Error>> {
    let raw_input = std::fs::read_to_string(path)?;
    Ok(raw_input
        .lines()
        .map(parse_horizontal_or_vertical_vent)
        .flatten()
        .collect::<Vec<Vent>>())
}

fn parse_horizontal_or_vertical_vent(line: &str) -> Option<Vent> {
    let coord_split = line
        .split("->")
        .map(|s| s.trim().split(',').map(|s| s.parse::<usize>().unwrap()))
        .flatten()
        .collect::<Vec<usize>>();
    assert_eq!(4, coord_split.len());
    let start = Point::new(coord_split[0], coord_split[1]);
    let stop = Point::new(coord_split[2], coord_split[3]);
    if start.x == stop.x || start.y == stop.y {
        Some(Vent::new(start, stop))
    } else {
        None
    }
}

fn get_all_input(path: &Path) -> Result<Vec<Vent>, Box<dyn Error>> {
    let raw_input = std::fs::read_to_string(path)?;
    Ok(raw_input.lines().map(parse_vent).collect::<Vec<Vent>>())
}

fn parse_vent(line: &str) -> Vent {
    let coord_split = line
        .split("->")
        .map(|s| s.trim().split(',').map(|s| s.parse::<usize>().unwrap()))
        .flatten()
        .collect::<Vec<usize>>();
    assert_eq!(4, coord_split.len());
    let start = Point::new(coord_split[0], coord_split[1]);
    let stop = Point::new(coord_split[2], coord_split[3]);
    Vent::new(start, stop)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Pos,
    Neg,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Vent {
    start: Point,
    stop: Point,
}

impl Vent {
    fn new(start: Point, stop: Point) -> Self {
        Self { start, stop }
    }
    fn get_all_points(&self) -> Vec<Point> {
        let mut vec = Vec::new();
        if self.start.x == self.stop.x {
            let (y_start, y_stop) = if self.stop.y > self.start.y {
                (self.start.y, self.stop.y)
            } else {
                (self.stop.y, self.start.y)
            };
            for yval in y_start..=y_stop {
                vec.push(Point::new(self.start.x, yval));
            }
        } else if self.start.y == self.stop.y {
            let (x_start, x_stop) = if self.stop.x > self.start.x {
                (self.start.x, self.stop.x)
            } else {
                (self.stop.x, self.start.x)
            };
            for xval in x_start..=x_stop {
                vec.push(Point::new(xval, self.start.y));
            }
        } else {
            let (x_dir, num_diag_points) = if self.stop.x > self.start.x {
                (Direction::Pos, self.stop.x - self.start.x)
            } else {
                (Direction::Neg, self.start.x - self.stop.x)
            };
            let y_dir = if self.stop.y > self.start.y {
                Direction::Pos
            } else {
                Direction::Neg
            };
            let mut xval = self.start.x;
            let mut yval = self.start.y;
            vec.push(Point::new(xval, yval));
            match (x_dir, y_dir) {
                (Direction::Pos, Direction::Pos) => {
                    for _ in 0..num_diag_points {
                        xval += 1;
                        yval += 1;
                        vec.push(Point::new(xval, yval));
                    }
                }
                (Direction::Pos, Direction::Neg) => {
                    for _ in 0..num_diag_points {
                        xval += 1;
                        yval -= 1;
                        vec.push(Point::new(xval, yval));
                    }
                }
                (Direction::Neg, Direction::Pos) => {
                    for _ in 0..num_diag_points {
                        xval -= 1;
                        yval += 1;
                        vec.push(Point::new(xval, yval));
                    }
                }
                (Direction::Neg, Direction::Neg) => {
                    for _ in 0..num_diag_points {
                        xval -= 1;
                        yval -= 1;
                        vec.push(Point::new(xval, yval));
                    }
                }
            }
        }
        vec
    }
}

#[derive(Debug, Clone)]
struct Grid<const N: usize> {
    grid: [[u8; N]; N],
}

impl<const N: usize> Grid<N> {
    fn new() -> Self {
        Self {
            grid: [[0u8; N]; N],
        }
    }
    fn mark_vent(&mut self, vent: &Vent) {
        let points = vent.get_all_points();
        for point in points {
            self.grid[point.x][point.y] += 1;
        }
    }
    fn count_marks_ge(&self, mark_number: u8) -> usize {
        let mut count = 0;
        for row in self.grid.iter().take(N) {
            for &v in row.iter().take(N) {
                if v >= mark_number {
                    count += 1;
                }
            }
        }
        count
    }
}

pub(crate) fn puzz5() {
    let vents_part_one =
        get_horizontal_or_vertical_input(Path::new(INPUT_PATH)).expect("Could not parse input");
    let mut grid = Grid::<1_000>::new();
    for vent in vents_part_one.iter() {
        grid.mark_vent(vent);
    }
    let two_line_overlap_count = grid.count_marks_ge(2);
    println!(
        "Part One: {} points have an overlap from two or more lines",
        two_line_overlap_count
    );
    let vents_part_two = get_all_input(Path::new(INPUT_PATH)).expect("Could not parse input");
    let mut grid = Grid::<1_000>::new();
    for vent in vents_part_two.iter() {
        grid.mark_vent(vent);
    }
    let two_line_overlap_count = grid.count_marks_ge(2);
    println!(
        "Part Two: {} points have an overlap from two or more lines",
        two_line_overlap_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn test_part_one() {
        let vents = TEST_INPUT
            .lines()
            .map(parse_horizontal_or_vertical_vent)
            .flatten()
            .collect::<Vec<Vent>>();
        let mut grid = Grid::<10>::new();
        for vent in vents.iter() {
            grid.mark_vent(vent);
        }
        let two_line_overlap_count = grid.count_marks_ge(2);
        assert_eq!(5, two_line_overlap_count);
    }

    #[test]
    fn test_part_two() {
        let vents = TEST_INPUT.lines().map(parse_vent).collect::<Vec<Vent>>();
        let mut grid = Grid::<10>::new();
        for vent in vents.iter() {
            grid.mark_vent(vent);
        }
        let two_line_overlap_count = grid.count_marks_ge(2);
        assert_eq!(12, two_line_overlap_count);
    }
}
