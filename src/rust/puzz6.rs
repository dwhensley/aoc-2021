use std::error::Error;
use std::path::Path;

static INPUT_PATH: &str = "input/puzz6.txt";

fn get_input(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(std::fs::read_to_string(path)?
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>())
}

fn sim_day_simple(fish_list: &mut Vec<u8>) {
    let mut new_fish: Vec<u8> = Vec::new();
    for f in fish_list.iter_mut() {
        if *f > 0 {
            *f -= 1;
        } else {
            *f = 6;
            new_fish.push(8);
        }
    }
    fish_list.append(&mut new_fish);
}

fn sim_day(breeder_state: &mut [usize; 9]) {
    let breeders = breeder_state[0];
    breeder_state.copy_within(1..9, 0);
    breeder_state[6] += breeders;
    breeder_state[8] = breeders;
}

pub(crate) fn puzz6() {
    let mut fish_list: Vec<u8> = get_input(Path::new(&INPUT_PATH)).expect("Could not parse input");
    for _ in 0..80 {
        sim_day_simple(&mut fish_list);
    }
    println!(
        "Part One: There will be {} lantern fish after 80 days",
        fish_list.len()
    );
    let mut breeder_state: [usize; 9] = [0; 9];
    let fish_list: Vec<u8> = get_input(Path::new(&INPUT_PATH)).expect("Could not parse input");
    for fish_state in fish_list {
        breeder_state[fish_state as usize] += 1;
    }
    for _ in 0..256 {
        sim_day(&mut breeder_state);
    }
    let num_fish = breeder_state.iter().sum::<usize>();
    println!(
        "Part Two: There will be {} lantern fish after 256 days",
        num_fish
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part_one() {
        let mut fish_list = TEST_INPUT
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        for _ in 0..18 {
            sim_day_simple(&mut fish_list);
        }
        assert_eq!(26, fish_list.len());
        for _ in 18..80 {
            sim_day_simple(&mut fish_list);
        }
        assert_eq!(5934, fish_list.len());
    }

    #[test]
    fn test_part_two() {
        let mut breeder_state: [usize; 9] = [0; 9];
        let fish_list = TEST_INPUT
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        for fish_state in fish_list {
            breeder_state[fish_state] += 1;
        }
        for _ in 0..256 {
            sim_day(&mut breeder_state);
        }
        let num_fish = breeder_state.iter().sum::<usize>();
        assert_eq!(26984457539, num_fish);
    }
}
