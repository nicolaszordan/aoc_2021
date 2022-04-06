use crate::aoc::AOCPart;

pub struct Part1 {}

impl AOCPart for Part1 {
    fn new() -> Self {
        Self {}
    }

    fn solve(&mut self, input: &str) -> String {
        let measurements = retrieve_depth_measurements(input);
        count_depth_increases(&measurements).to_string()
    }
}

pub struct Part2 {}

impl AOCPart for Part2 {
    fn new() -> Self {
        Self {}
    }

    fn solve(&mut self, input: &str) -> String {
        let measurements = retrieve_depth_measurements(input);
        let measurements = rafine_measurements(&measurements);
        count_depth_increases(&measurements).to_string()
    }
}

fn retrieve_depth_measurements(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn rafine_measurements(measurements: &[u32]) -> Vec<u32> {
    measurements
        .windows(3)
        .map(|wind| wind.iter().sum())
        .collect()
}

fn count_depth_increases(measurements: &[u32]) -> usize {
    measurements
        .windows(2)
        .filter(|wind| match wind {
            [first, second] => second > first,
            _ => panic!(),
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_depth_increases_example() {
        assert_eq!(
            count_depth_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
            7
        );
    }

    #[test]
    fn rafine_measurements_example() {
        assert_eq!(
            rafine_measurements(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
            vec![607, 618, 618, 617, 647, 716, 769, 792]
        );
    }
}
