use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

static INPUT_PATH: &str = "input/puzz4.csv";

fn get_input(path: &Path) -> Result<(Vec<u32>, Vec<BingoBoard>), Box<dyn Error>> {
    let raw_input = std::fs::read_to_string(path)?;
    let (raw_numbers, raw_boards) = raw_input.split_once('\n').unwrap();
    let mut numbers: Vec<u32> = Vec::new();
    for raw_num in raw_numbers.split(',') {
        numbers.push(raw_num.parse::<u32>()?);
    }
    let board_elems = raw_boards
        .split('\n')
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.split(' ').filter(|s| !s.is_empty()))
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let boards = board_elems
        .chunks_exact(25)
        .map(BingoBoard::initialize_from_row_major_1d_array)
        .collect::<Vec<BingoBoard>>();
    Ok((numbers, boards))
}

#[derive(Debug, Clone, Default)]
struct BingoBoard {
    data: [(u32, bool); 25],
}

impl BingoBoard {
    fn initialize_from_2d_array(arr: &[[u32; 5]; 5]) -> Self {
        let mut board = BingoBoard::default();
        let mut bidx = 0;
        for row in arr.iter().take(5) {
            for &elem in row.iter().take(5) {
                board.data[bidx] = (elem, false);
                bidx += 1;
            }
        }
        board
    }
    fn initialize_from_row_major_1d_array(arr: &[u32]) -> Self {
        assert!(arr.len() == 25);
        let mut board = BingoBoard::default();
        for (&elem, board_elem) in arr.iter().zip(board.data.iter_mut()) {
            *board_elem = (elem, false);
        }
        board
    }
}

impl BingoBoard {
    fn mark_number(&mut self, num: u32) {
        for &mut (v, ref mut mark) in self.data.iter_mut() {
            if v == num {
                *mark = true;
            }
        }
    }
    fn clear_marks(&mut self) {
        for &mut (_, ref mut mark) in self.data.iter_mut() {
            *mark = false;
        }
    }
    fn check_for_bingo_row(&self) -> Option<usize> {
        let row_bingos = self
            .data
            .as_slice()
            .chunks_exact(5)
            .map(|ch| ch.iter().map(|(_, b)| b).all(|&v| v))
            .collect::<Vec<bool>>();
        for (ridx, &all_marked) in row_bingos.iter().enumerate() {
            if all_marked {
                return Some(ridx);
            }
        }
        None
    }
    fn check_for_bingo_col(&self) -> Option<usize> {
        for cidx in 0..5 {
            let mut all_marked = true;
            for ridx in 0..5 {
                all_marked &= self.data[cidx + ridx * 5].1
            }
            if all_marked {
                return Some(cidx);
            }
        }
        None
    }
    fn check_for_bingo(&self) -> bool {
        if self.check_for_bingo_row().is_some() {
            return true;
        }
        if self.check_for_bingo_col().is_some() {
            return true;
        }
        false
    }
}

#[derive(Debug, Copy, Clone)]
struct BingoWinningState {
    board_idx: usize,
    num_idx: usize,
    number: u32,
    score: u32,
}

fn play_bingo(boards: &mut [BingoBoard], numbers: &[u32]) -> BingoWinningState {
    for board in boards.iter_mut() {
        board.clear_marks();
    }
    let mut winning_board_idx: usize = 0;
    let mut winning_num_idx: usize = 0;
    let mut winning_number: u32 = numbers[0];
    'outer: for (nidx, &num) in numbers.iter().enumerate() {
        for (bidx, board) in boards.iter_mut().enumerate() {
            board.mark_number(num);
            if board.check_for_bingo() {
                winning_board_idx = bidx;
                winning_number = num;
                winning_num_idx = nidx;
                break 'outer;
            }
        }
    }
    let sum_non_marks =
        boards[winning_board_idx]
            .data
            .iter()
            .fold(0u32, |acc, v| if !v.1 { acc + v.0 } else { acc });
    let winning_score = sum_non_marks * winning_number;
    BingoWinningState {
        board_idx: winning_board_idx,
        num_idx: winning_num_idx,
        number: winning_number,
        score: winning_score,
    }
}

fn play_bingo_to_last_winning_board(
    boards: &mut [BingoBoard],
    numbers: &[u32],
) -> BingoWinningState {
    let mut winners = 0;
    let target_num_winners = boards.len();
    for board in boards.iter_mut() {
        board.clear_marks();
    }
    let mut last_winning_board_idx: usize = 0;
    let mut last_winning_num_idx: usize = 0;
    let mut last_winning_number: u32 = numbers[0];
    let mut won_set: HashSet<usize> = HashSet::new();
    'outer: for (nidx, &num) in numbers.iter().enumerate() {
        for (bidx, board) in boards.iter_mut().enumerate() {
            if won_set.contains(&bidx) {
                continue;
            }
            board.mark_number(num);
            if board.check_for_bingo() {
                won_set.insert(bidx);
                winners += 1;
                if winners == target_num_winners {
                    last_winning_board_idx = bidx;
                    last_winning_number = num;
                    last_winning_num_idx = nidx;
                    break 'outer;
                }
            }
        }
    }
    let sum_non_marks = boards[last_winning_board_idx]
        .data
        .iter()
        .fold(0u32, |acc, v| if !v.1 { acc + v.0 } else { acc });
    let last_winning_score = sum_non_marks * last_winning_number;
    BingoWinningState {
        board_idx: last_winning_board_idx,
        num_idx: last_winning_num_idx,
        number: last_winning_number,
        score: last_winning_score,
    }
}

pub(crate) fn puzz4() {
    let (numbers, mut boards) =
        get_input(Path::new(INPUT_PATH)).expect("Could not parse input data");
    let winning_stats = play_bingo(&mut boards, numbers.as_slice());
    println!(
        "Part One: Winning score: {}, from the {}-th board",
        winning_stats.score, winning_stats.board_idx
    );
    let last_winning_stats = play_bingo_to_last_winning_board(&mut boards, numbers.as_slice());
    println!(
        "Part Two: Last winning score: {}, from the {}-th board",
        last_winning_stats.score, last_winning_stats.board_idx
    );
}

#[cfg(test)]

mod tests {
    use super::*;
    static TEST_NUMS: [u32; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    static TEST_BOARDS: [[[u32; 5]; 5]; 3] = [
        [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ],
        [
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ],
        [
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ],
    ];

    #[test]
    fn test_part_one() {
        let mut boards: Vec<BingoBoard> = TEST_BOARDS
            .iter()
            .map(|b| BingoBoard::initialize_from_2d_array(b))
            .collect();
        let winning_stats = play_bingo(&mut boards, TEST_NUMS.as_slice());
        assert_eq!(2, winning_stats.board_idx);
        assert_eq!(11, winning_stats.num_idx);
        assert_eq!(24, winning_stats.number);
        assert_eq!(4512, winning_stats.score);
    }

    #[test]
    fn test_part_two() {
        let mut boards: Vec<BingoBoard> = TEST_BOARDS
            .iter()
            .map(|b| BingoBoard::initialize_from_2d_array(b))
            .collect();
        let last_winner_stats = play_bingo_to_last_winning_board(&mut boards, TEST_NUMS.as_slice());
        assert_eq!(1, last_winner_stats.board_idx);
        assert_eq!(14, last_winner_stats.num_idx);
        assert_eq!(13, last_winner_stats.number);
        assert_eq!(1924, last_winner_stats.score);
    }
}
