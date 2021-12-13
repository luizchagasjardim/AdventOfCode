use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    println!["{}", solve(&contents)];
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn from_string(command: &str) -> Command {
        let splitted = command.split(' ').collect::<Vec<_>>();
        let command_name = splitted[0].to_string();
        let distance = splitted[1].parse::<i32>().unwrap();
        if command_name == "forward" {
            Command::Forward(distance)
        } else if command_name == "down" {
            Command::Down(distance)
        } else {
            Command::Up(distance)
        }
    }
}

struct Position {
    aim: i32,
    forward: i32,
    depth: i32,
}

impl Position {
    fn new() -> Position {
        Position{aim: 0, forward: 0, depth: 0}
    }
}

fn solve(input: &str) -> i32 {
    let position = input
        .split('\n')
        .map(Command::from_string)
        .fold(Position::new(), |mut position, command| {
            match command {
                Command::Forward(distance) => {
                    position.forward += distance;
                    position.depth += distance*position.aim;
                },
                Command::Up(distance) => position.aim -= distance,
                Command::Down(distance) => position.aim += distance,
            }
            position
        });
    position.forward * position.depth
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(
            solve("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            900
        );
    }
}