use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    println!["{}", solve(&contents)];
}

fn solve(input: &str) -> usize {
    let value_iter = input
        .split('\n')
        .map(
            |x| x.parse::<i32>().unwrap()
        );
    let mut next_value_iter = value_iter.clone();
    next_value_iter.advance_by(3).unwrap();
    value_iter
        .zip(next_value_iter)
        .filter(|pair| pair.1 > pair.0)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(
            solve("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"),
            5
        );
    }
}