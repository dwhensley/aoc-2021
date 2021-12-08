use std::error::Error;
use std::path::Path;

static INPUT_PATH: &str = "input/puzz7.txt";

fn get_input(path: &Path) -> Result<Vec<isize>, Box<dyn Error>> {
    Ok(std::fs::read_to_string(path)?
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>())
}

pub(crate) fn puzz7() {
    let mut crab_hpos: Vec<isize> =
        get_input(Path::new(&INPUT_PATH)).expect("Could not parse input");
    crab_hpos.sort_unstable();
    let num_crabs = crab_hpos.len();
    let median_pos = if num_crabs % 2 == 0 {
        ((crab_hpos[num_crabs / 2 - 1] + crab_hpos[(num_crabs / 2)]) as f64 / 2.0_f64).floor()
            as isize
    } else {
        crab_hpos[(num_crabs - 1) / 2]
    };
    let fuel_cost = crab_hpos
        .iter()
        .map(|&p| (p - median_pos).abs())
        .sum::<isize>();
    println!(
        "Part One: Optimal fuel spend {} aligning at horizontal position {}",
        fuel_cost, median_pos
    );

    let mean_pos1 = (crab_hpos.iter().sum::<isize>() as f64 / num_crabs as f64).floor() as isize;
    let mean_pos2 = (crab_hpos.iter().sum::<isize>() as f64 / num_crabs as f64).ceil() as isize;
    let fuel_cost1 = crab_hpos
        .iter()
        .map(|&p| {
            let num_moves = (p - mean_pos1).abs();
            (1..=num_moves).into_iter().sum::<isize>()
        })
        .sum::<isize>();
    let fuel_cost2 = crab_hpos
        .iter()
        .map(|&p| {
            let num_moves = (p - mean_pos2).abs();
            (1..=num_moves).into_iter().sum::<isize>()
        })
        .sum::<isize>();
    let (mean_pos, fuel_cost) = if fuel_cost1 < fuel_cost2 {
        (mean_pos1, fuel_cost1)
    } else {
        (mean_pos2, fuel_cost2)
    };
    println!(
        "Part Two: Optimal fuel spend {} aligning at horizontal position {}",
        fuel_cost, mean_pos
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_one() {
        let mut crab_hpos = TEST_INPUT
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        crab_hpos.sort_unstable();
        let num_crabs = crab_hpos.len();
        let median_pos = if num_crabs % 2 == 0 {
            ((crab_hpos[num_crabs / 2 - 1] + crab_hpos[(num_crabs / 2)]) as f64 / 2.0_f64).floor()
                as isize
        } else {
            crab_hpos[(num_crabs - 1) / 2]
        };
        let fuel_cost = crab_hpos
            .iter()
            .map(|&p| (p - median_pos).abs())
            .sum::<isize>();
        assert_eq!(37, fuel_cost);
    }

    #[test]
    fn test_part_two() {
        let crab_hpos = TEST_INPUT
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let num_crabs = crab_hpos.len();
        let mean_pos1 =
            (crab_hpos.iter().sum::<isize>() as f64 / num_crabs as f64).floor() as isize;
        let mean_pos2 = (crab_hpos.iter().sum::<isize>() as f64 / num_crabs as f64).ceil() as isize;
        let fuel_cost1 = crab_hpos
            .iter()
            .map(|&p| {
                let num_moves = (p - mean_pos1).abs();
                (1..=num_moves).into_iter().sum::<isize>()
            })
            .sum::<isize>();
        let fuel_cost2 = crab_hpos
            .iter()
            .map(|&p| {
                let num_moves = (p - mean_pos2).abs();
                (1..=num_moves).into_iter().sum::<isize>()
            })
            .sum::<isize>();
        let fuel_cost = [fuel_cost1, fuel_cost2].into_iter().min().unwrap();
        assert_eq!(168, fuel_cost);
    }
}
