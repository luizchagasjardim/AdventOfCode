use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/sousajo/etudes/adventofcode/input")
        .expect("Something went wrong reading the file");

    let mut splitted = contents.split('\n');
    let mut aux : i32 =  i32::MAX;
    let mut cnt : i32 = 0;
    for s in splitted {
        let s_as_int : i32 = s.parse().unwrap();
        if s_as_int > aux {
            cnt += 1;
        }
        aux = s_as_int;
    }
    println!("{}", cnt);
}
