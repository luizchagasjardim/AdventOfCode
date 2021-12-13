use std::convert::TryInto;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    println!["{}", solve(&contents)];
}

fn solve(input: &str) -> i32 {
    let number_size = input.chars().position(|character| character == '\n').unwrap();
    let input_count = input.len() / (number_size+1);
    let mut count_of_ones = Vec::<usize>::with_capacity(number_size);
    count_of_ones.resize(number_size, 0);
    let gamma_rate = input
        .chars()
        .enumerate()
        .fold(
            &mut count_of_ones,
            |count_of_ones, (position, character)| {
                if character == '1' {
                    let digit_position = position % (number_size+1);
                    count_of_ones[digit_position] += 1;
                }
                count_of_ones
            }
        )
        .into_iter()
        .map(|count_of_ones| {
            count_of_ones > &mut (input_count/2)
        })
        .enumerate()
        .fold(
            0,
            |rate, (position, bit)| {
                if bit {
                    rate + 2_i32.pow((number_size-1-position).try_into().unwrap())
                } else {
                    rate
                }
            }
        );
    let epsilon_rate = 2_i32.pow(number_size.try_into().unwrap())-1 - gamma_rate;
    gamma_rate*epsilon_rate
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(
            solve("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"),
            198
        );
    }
}