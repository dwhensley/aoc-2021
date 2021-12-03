use std::error::Error;
use std::path::Path;

static INPUT_PATH: &str = "input/puzz1.csv";

fn get_input(path: &Path) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&path)?;
    let mut input: Vec<i32> = Vec::new();
    for result in rdr.records() {
        input.push(result?.as_slice().parse::<i32>()?);
    }
    Ok(input)
}

fn diff(slc: &[i32]) -> Vec<i32> {
    slc[0..slc.len() - 1]
        .iter()
        .zip(slc[1..].iter())
        .map(|(&v0, &v1)| v1 - v0)
        .collect::<Vec<i32>>()
}

fn moving_sum(slc: &[i32], window_size: usize) -> Vec<i32> {
    slc.windows(window_size)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
}

fn count_positive(slc: &[i32]) -> u32 {
    slc.iter()
        .fold(0u32, |acc, &v| if v > 0 { acc + 1 } else { acc })
}

pub(crate) fn puzz1() {
    let input = get_input(Path::new(INPUT_PATH)).expect("Could not read input data");
    let cnt_larger = count_positive(&diff(&input));
    println!(
        "{} measurements larger than the previous measurement",
        cnt_larger
    );
    let cnt_larger = count_positive(&diff(&moving_sum(&input, 3)));
    println!(
        "{} values larger than the previous value in window-summed input",
        cnt_larger
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part_one() {
        let count = count_positive(&diff(&TEST_INPUT));
        assert_eq!(7, count);
    }

    #[test]
    fn test_part_two() {
        let count = count_positive(&diff(&moving_sum(&TEST_INPUT, 3)));
        assert_eq!(5, count);
    }
}
