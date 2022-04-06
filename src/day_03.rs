use crate::aoc::AOCPart;
use std::cmp::Ordering;
pub struct Part1 {}

impl AOCPart for Part1 {
    fn new() -> Self {
        Self {}
    }

    fn solve(&mut self, input: &str) -> String {
        let comsumption_report = retrieve_consumption(input);
        let gamma = calc_gamma(&comsumption_report.report, comsumption_report.mask_size);
        let epsylon = calc_epsylon(&comsumption_report.report, comsumption_report.mask_size);
        (gamma * epsylon).to_string()
    }
}

pub struct Part2 {}

impl AOCPart for Part2 {
    fn new() -> Self {
        Self {}
    }

    fn solve(&mut self, input: &str) -> String {
        let comsumption_report = retrieve_consumption(input);
        let oxygen = calc_oxygen_generator_rating(
            comsumption_report.report.clone(),
            comsumption_report.mask_size,
            0,
        );
        let co2 =
            calc_co2_scrubber_rating(comsumption_report.report, comsumption_report.mask_size, 0);
        (oxygen * co2).to_string()
    }
}

struct ConsumptionReport {
    report: Vec<u32>,
    mask_size: u32,
}

fn retrieve_consumption(input: &str) -> ConsumptionReport {
    ConsumptionReport {
        report: input
            .lines()
            .map(|line| u32::from_str_radix(line, 2).unwrap())
            .collect(),
        mask_size: input.lines().next().unwrap().len() as u32,
    }
}

fn calc_epsylon(input: &[u32], mask_size: u32) -> u32 {
    (0..mask_size).into_iter().fold(0u32, |acc, mask| {
        let sum = input
            .iter()
            .map(|elem| (elem & (1 << mask)) >> mask)
            .sum::<u32>();
        let average_msb = (sum >= input.len() as u32 / 2) as u32;
        acc | (average_msb << mask)
    })
}

fn calc_gamma(input: &[u32], mask_size: u32) -> u32 {
    (0..mask_size).into_iter().fold(0u32, |acc, mask| {
        acc | (((input
            .iter()
            .map(|elem| (elem & (1 << mask)) >> mask)
            .sum::<u32>()
            <= input.len() as u32 / 2) as u32)
            << mask)
    })
}

fn calc_oxygen_generator_rating(input: Vec<u32>, mask_size: u32, current_bit: u32) -> u32 {
    match input.len() {
        0 => panic!("empty input"),
        1 => input[0],
        _ => {
            let bit_offset = (mask_size - current_bit) - 1;

            let mask = 1 << bit_offset;

            let masked_bit_sum = input
                .iter()
                .map(|elem| (elem & mask) >> bit_offset)
                .sum::<u32>();

            let average_msb = match (input.len() as u32 - masked_bit_sum).cmp(&masked_bit_sum) {
                Ordering::Greater => 0,
                Ordering::Equal => 1,
                Ordering::Less => 1,
            };

            let filtered_input = input
                .iter()
                .filter(|elem| (*elem >> bit_offset) & 1 == average_msb)
                .copied()
                .collect::<Vec<u32>>();

            calc_oxygen_generator_rating(filtered_input, mask_size, current_bit + 1)
        }
    }
}

fn calc_co2_scrubber_rating(input: Vec<u32>, mask_size: u32, current_bit: u32) -> u32 {
    match input.len() {
        0 => panic!("empty input"),
        1 => input[0],
        _ => {
            let bit_offset = (mask_size - current_bit) - 1;

            let mask = 1 << bit_offset;

            let masked_bit_sum = input
                .iter()
                .map(|elem| (elem & mask) >> bit_offset)
                .sum::<u32>();

            let average_lsb = match (input.len() as u32 - masked_bit_sum).cmp(&masked_bit_sum) {
                Ordering::Greater => 1,
                Ordering::Equal => 0,
                Ordering::Less => 0,
            };

            let filtered_input = input
                .iter()
                .filter(|elem| (*elem >> ((mask_size - current_bit) - 1)) & 1 == average_lsb)
                .copied()
                .collect::<Vec<u32>>();

            calc_co2_scrubber_rating(filtered_input, mask_size, current_bit + 1)
        }
    }
}

use std::fmt;
struct BinaryPrinter<'a>(&'a [u32]);
impl fmt::Binary for BinaryPrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, n) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, " ")?;
            }
            write!(f, "{:b}", n)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_epsylon_example_01() {
        assert_eq!(
            calc_epsylon(
                &[
                    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100,
                    0b10000, 0b11001, 0b00010, 0b01010,
                ],
                5
            ),
            22
        );
    }

    #[test]
    fn calc_epsylon_example_02() {
        assert_eq!(calc_epsylon(&[0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0,], 1), 0);
        assert_eq!(calc_epsylon(&[0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0,], 1), 1);
    }

    #[test]
    fn calc_gamma_example_01() {
        assert_eq!(
            calc_gamma(
                &[
                    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100,
                    0b10000, 0b11001, 0b00010, 0b01010,
                ],
                5
            ),
            9
        );
    }

    #[test]
    fn calc_gamma_example_02() {
        assert_eq!(calc_gamma(&[0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0,], 1), 1);
        assert_eq!(calc_gamma(&[0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0,], 0), 0);
    }

    #[test]
    fn calc_oxygen_example_01() {
        assert_eq!(
            calc_oxygen_generator_rating(
                vec![
                    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100,
                    0b10000, 0b11001, 0b00010, 0b01010,
                ],
                5,
                0
            ),
            23
        );
    }

    #[test]
    fn calc_co2_example_01() {
        assert_eq!(
            calc_co2_scrubber_rating(
                vec![
                    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100,
                    0b10000, 0b11001, 0b00010, 0b01010,
                ],
                5,
                0
            ),
            10
        );
    }
}
