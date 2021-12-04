use std::error::Error;
use std::path::Path;

static INPUT_PATH: &str = "input/puzz3.csv";

fn get_input(path: &Path) -> Result<DiagnosticData, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&path)?;
    let mut input: Vec<String> = Vec::new();
    for result in rdr.records() {
        input.push(result?.as_slice().into())
    }
    input.as_slice().try_into()
}

#[derive(Debug, Copy, Clone)]
enum ArrayOrder {
    ColMajor,
    RowMajor,
}

#[derive(Debug, Copy, Clone)]
enum LifeSupportRating {
    OxyGen,
    Co2Scrub,
}

#[derive(Debug, Clone)]
struct DiagnosticData {
    data: Vec<u8>,
    num_cols: usize,
    num_rows: usize,
    order: ArrayOrder,
}

impl DiagnosticData {
    fn to_row_major(&self) -> Self {
        let data = match self.order {
            ArrayOrder::RowMajor => self.data.clone(),
            ArrayOrder::ColMajor => {
                let mut new_vec = Vec::with_capacity(self.data.len());
                for ridx in 0..self.num_rows {
                    for cidx in 0..self.num_cols {
                        new_vec.push(self.data[(cidx * self.num_rows) + ridx]);
                    }
                }
                new_vec
            }
        };
        Self {
            data,
            num_cols: self.num_cols,
            num_rows: self.num_rows,
            order: ArrayOrder::RowMajor,
        }
    }
    fn to_col_major(&self) -> Self {
        let data = match self.order {
            ArrayOrder::ColMajor => self.data.clone(),
            ArrayOrder::RowMajor => {
                let mut new_vec = Vec::with_capacity(self.data.len());
                for cidx in 0..self.num_cols {
                    for ridx in 0..self.num_rows {
                        new_vec.push(self.data[(ridx * self.num_cols) + cidx]);
                    }
                }
                new_vec
            }
        };
        Self {
            data,
            num_cols: self.num_cols,
            num_rows: self.num_rows,
            order: ArrayOrder::ColMajor,
        }
    }
}

impl TryFrom<&[String]> for DiagnosticData {
    type Error = Box<dyn Error>;
    fn try_from(input: &[String]) -> Result<Self, Self::Error> {
        let num_rows = input.len();
        let num_cols = input[0].len();
        let mut data = Vec::with_capacity(num_rows * num_cols);
        for row in input.iter() {
            for &v in row.as_bytes() {
                match v {
                    48 => data.push(0),
                    49 => data.push(1),
                    _ => return Err(format!("Encountered non-0/1 ASCII: {}", v).into()),
                }
            }
        }
        Ok(Self {
            data,
            num_cols,
            num_rows,
            order: ArrayOrder::RowMajor,
        })
    }
}

fn compute_gamma_bits(data: &DiagnosticData) -> Vec<u8> {
    data.to_col_major()
        .data
        .chunks_exact(data.num_rows)
        .map(|slc| {
            let sum: usize = slc.iter().map(|&v| v as usize).sum();
            if sum > data.num_rows / 2 {
                1
            } else {
                0
            }
        })
        .collect::<Vec<u8>>()
}

fn bits_to_integer(bits: &[u8]) -> usize {
    let mut integer = 0;
    for (idx, &b) in bits.iter().rev().enumerate() {
        if b > 0 {
            integer += 1 << idx;
        }
    }
    integer
}

fn compute_gamma_and_eps_values(gamma_bits: &[u8]) -> (usize, usize) {
    let mut gamma_value = 0usize;
    let mut epsilon_value = 0usize;
    for (idx, &v) in gamma_bits.iter().rev().enumerate() {
        if v > 0 {
            gamma_value += 1 << idx;
        } else {
            epsilon_value += 1 << idx;
        }
    }
    (gamma_value, epsilon_value)
}

fn filter_rows_at_cidx(
    data: &DiagnosticData,
    cidx: usize,
    life_support: LifeSupportRating,
) -> DiagnosticData {
    let start = data.num_rows * cidx;
    let stop = start + data.num_rows;
    let data_cm = data.to_col_major();
    let col_data = &data_cm.data[start..stop];
    let sum = col_data.iter().map(|&v| v as usize).sum::<usize>();
    let more_ones = if data.num_rows % 2 == 0 {
        sum >= data.num_rows / 2
    } else {
        sum > data.num_rows / 2
    };
    let mut keep_ridxs = Vec::new();
    match life_support {
        LifeSupportRating::OxyGen => {
            for (ridx, &val) in col_data.iter().enumerate() {
                if (more_ones && val == 1) || (!more_ones && val == 0) {
                    keep_ridxs.push(ridx);
                }
            }
        }
        LifeSupportRating::Co2Scrub => {
            for (ridx, &val) in col_data.iter().enumerate() {
                if (more_ones && val == 0) || (!more_ones && val == 1) {
                    keep_ridxs.push(ridx);
                }
            }
        }
    }
    let mut filtered_rows = Vec::new();
    let data_rm = data.to_row_major().data;
    for &ridx in keep_ridxs.iter() {
        let start = ridx * data.num_cols;
        let stop = start + data.num_cols;
        for &v in data_rm[start..stop].iter() {
            filtered_rows.push(v);
        }
    }
    DiagnosticData {
        data: filtered_rows,
        num_cols: data.num_cols,
        num_rows: keep_ridxs.len(),
        order: ArrayOrder::RowMajor,
    }
}

fn compute_oxygen_generator_rating(data: &DiagnosticData) -> usize {
    let mut filtered_rows = data.to_row_major();
    for cidx in 0..data.num_cols {
        filtered_rows = filter_rows_at_cidx(&filtered_rows, cidx, LifeSupportRating::OxyGen);
        if filtered_rows.data.len() == data.num_cols {
            break;
        }
    }
    bits_to_integer(filtered_rows.data.as_slice())
}

fn compute_co2_scrubber_rating(data: &DiagnosticData) -> usize {
    let mut filtered_rows = data.to_row_major();
    for cidx in 0..data.num_cols {
        filtered_rows = filter_rows_at_cidx(&filtered_rows, cidx, LifeSupportRating::Co2Scrub);
        if filtered_rows.data.len() == data.num_cols {
            break;
        }
    }
    bits_to_integer(filtered_rows.data.as_slice())
}

pub(crate) fn puzz3() {
    let data = get_input(Path::new(INPUT_PATH))
        .expect("Could not parse input data")
        .to_col_major();
    let gamma_bits = compute_gamma_bits(&data);
    let (gamma_value, epsilon_value) = compute_gamma_and_eps_values(gamma_bits.as_slice());
    println!(
        "Part one | (gamma, epsilon) ({}, {}); multiplication: {}",
        gamma_value,
        epsilon_value,
        gamma_value * epsilon_value
    );
    let oxygen_generator_rating = compute_oxygen_generator_rating(&data);
    let co2_scrubber_rating = compute_co2_scrubber_rating(&data);
    println!(
        "Part two | oxygen generator rating: {}, CO2 scrubber rating: {}; multiplication: {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    );
}

#[cfg(test)]

mod tests {
    use super::*;
    static TEST_INPUT: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_part_one() {
        let data = TEST_INPUT
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        let data = DiagnosticData::try_from(data.as_slice())
            .expect("Could not parse data")
            .to_col_major();
        let gamma_bits = compute_gamma_bits(&data);
        let (gamma_value, epsilon_value) = compute_gamma_and_eps_values(gamma_bits.as_slice());
        assert_eq!(22, gamma_value);
        assert_eq!(9, epsilon_value);
        assert_eq!(198, gamma_value * epsilon_value);
    }

    #[test]
    fn test_part_two() {
        let data = TEST_INPUT
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        let data = DiagnosticData::try_from(data.as_slice()).expect("Could not parse data");
        let oxygen_generator_rating = compute_oxygen_generator_rating(&data);
        let co2_scrubber_rating = compute_co2_scrubber_rating(&data);
        assert_eq!(23, oxygen_generator_rating);
        assert_eq!(10, co2_scrubber_rating);
        assert_eq!(230, oxygen_generator_rating * co2_scrubber_rating);
    }
}
