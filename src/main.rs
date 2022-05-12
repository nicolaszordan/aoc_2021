use aoc_2021::aoc::AOCPart;
use aoc_2021::day_04::Part2 as Part;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();
    let mut part = Part::new();
    println!("{}", part.solve(&input));
}
