// does not work =(

use std::convert::TryInto;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    println!["{}", solve(&contents)];
}

trait FromBinary {
    fn from_binary(&self) -> i32;
}

impl FromBinary for String {
    fn from_binary(&self) -> i32 {
        self
            .chars()
            .rev()
            .enumerate()
            .fold(
                0,
                |rate, (position, character)| {
                    if character == '1' {
                        rate + 2_i32.pow(position.try_into().unwrap())
                    } else {
                        //assume '0'
                        rate
                    }
                }
            )
    }
}

fn solve(input: &str) -> i32 {
    let number_size = input.chars().position(|character| character == '\n').unwrap();
    let input_count = input.len() / (number_size+1);
    let mut count_of_ones = Vec::<usize>::with_capacity(number_size);
    count_of_ones.resize(number_size, 0);
    let gamma_rate: String = input
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
            if count_of_ones > &mut (input_count/2) {
                '1'
            } else {
                '0'
            }
        })
        .collect();
    let oxygen_generator_rating = input
        .split('\n')
        .fold(
            ("", 0),
            |(best_match, best_match_first_different_bit), n| {
                let first_different_bit = match n.chars()
                    .zip(gamma_rate.chars().clone())
                    .enumerate()
                    .find(|(_index, (bit, gamma_rate_bit))| bit != gamma_rate_bit) {
                        None => 0,
                        Some((index, (_, _))) => index,
                    };
                if first_different_bit > best_match_first_different_bit {
                    (n, first_different_bit)
                } else {
                    (best_match, best_match_first_different_bit)
                }
            }
        )
        .0
        .to_string()
        .from_binary();
    let co2_scrubber_rating = input
        .split('\n')
        .fold(
            ("", 0),
            |(best_match, best_match_first_equal_bit), n| {
                let first_equal_bit = match n.chars()
                    .zip(gamma_rate.chars().clone())
                    .enumerate()
                    .find(|(_index, (bit, gamma_rate_bit))| bit == gamma_rate_bit) {
                        None => 0,
                        Some((index, (_, _))) => index,
                    };
                if first_equal_bit > best_match_first_equal_bit {
                    (n, first_equal_bit)
                } else {
                    (best_match, best_match_first_equal_bit)
                }
            }
        )
        .0
        .to_string()
        .from_binary();
    oxygen_generator_rating*co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(
            solve("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"),
            230
        );
    }
}