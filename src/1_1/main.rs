use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    println!["{}", solve(&contents)];
}

fn solve(input: &str) -> usize {
    input
        .split('\n')
        .map(
            |x| x.parse::<i32>().unwrap()
        )
        .scan(
            i32::MAX,
            |previous, current| {
                let increased = current > *previous;
                *previous = current;
                Some(increased)
            }
        )
        .filter(|increased| *increased)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(
            solve("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"),
            7
        );
    }
}
