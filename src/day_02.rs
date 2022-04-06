use crate::aoc::AOCPart;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    direction: Direction,
    units: i32,
}

fn retrieve_command_list(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let direction = match words.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!(),
            };
            let units = words.next().unwrap().parse().unwrap();
            Command { direction, units }
        })
        .collect()
}

pub struct Part1 {}

impl AOCPart for Part1 {
    fn new() -> Self {
        Self {}
    }

    fn solve(&mut self, input: &str) -> String {
        let commands = retrieve_command_list(input);
        let (depth, horizontal) = run_commands(&commands);
        (depth * horizontal).to_string()
    }
}

fn run_commands(commands: &[Command]) -> (i32, i32) {
    commands
        .iter()
        .fold((0, 0), |(depth, horizontal), command| {
            match command.direction {
                Direction::Down => (depth + command.units, horizontal),
                Direction::Up => (depth - command.units, horizontal),
                Direction::Forward => (depth, horizontal + command.units),
            }
        })
}

pub struct Part2 {}

impl AOCPart for Part2 {
    fn new() -> Self {
        Self {}
    }

    fn solve(&mut self, input: &str) -> String {
        let commands = retrieve_command_list(input);
        let (depth, horizontal, _) = run_commands_with_aim(&commands);
        (depth * horizontal).to_string()
    }
}

fn run_commands_with_aim(commands: &[Command]) -> (i32, i32, i32) {
    commands.iter().fold(
        (0, 0, 0),
        |(depth, horizontal, aim), command| match command.direction {
            Direction::Down => (depth, horizontal, aim + command.units),
            Direction::Up => (depth, horizontal, aim - command.units),
            Direction::Forward => (depth + aim * command.units, horizontal + command.units, aim),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retrieve_command_list_example() {
        assert_eq!(
            retrieve_command_list(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ),
            vec![
                Command {
                    direction: Direction::Forward,
                    units: 5
                },
                Command {
                    direction: Direction::Down,
                    units: 5
                },
                Command {
                    direction: Direction::Forward,
                    units: 8
                },
                Command {
                    direction: Direction::Up,
                    units: 3
                },
                Command {
                    direction: Direction::Down,
                    units: 8
                },
                Command {
                    direction: Direction::Forward,
                    units: 2
                }
            ]
        );
    }

    #[test]
    fn run_commands_example() {
        assert_eq!(
            run_commands(&[
                Command {
                    direction: Direction::Forward,
                    units: 5
                },
                Command {
                    direction: Direction::Down,
                    units: 5
                },
                Command {
                    direction: Direction::Forward,
                    units: 8
                },
                Command {
                    direction: Direction::Up,
                    units: 3
                },
                Command {
                    direction: Direction::Down,
                    units: 8
                },
                Command {
                    direction: Direction::Forward,
                    units: 2
                }
            ]),
            (10, 15)
        );
    }

    #[test]
    fn run_commands_with_aim_example() {
        let (depth, horizontal, _) = run_commands_with_aim(&[
            Command {
                direction: Direction::Forward,
                units: 5,
            },
            Command {
                direction: Direction::Down,
                units: 5,
            },
            Command {
                direction: Direction::Forward,
                units: 8,
            },
            Command {
                direction: Direction::Up,
                units: 3,
            },
            Command {
                direction: Direction::Down,
                units: 8,
            },
            Command {
                direction: Direction::Forward,
                units: 2,
            },
        ]);
        assert_eq!(depth, 60);
        assert_eq!(horizontal, 15);
    }
}
