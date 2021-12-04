use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/sousajo/etudes/adventofcode/input")
        .expect("Something went wrong reading the file");

    let splitted = contents.split('\n');
    let mut cnt: i32 = 0;
    let vec = splitted.map(|x| -> i32 { x.parse::<i32>().unwrap() }).collect::<Vec<i32>>();
    for n in 0..vec.len() - 3 {
        if vec[n] < vec[n+3] {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
