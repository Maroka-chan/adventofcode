use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Failed to read file!");

    let mut maxes: [i32; 3] = [0; 3];
    let mut count: i32 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            if count > maxes[0] {
                maxes[0] = count;
                maxes.sort();
            }
            count = 0;
            continue;
        }
        count += line.parse::<i32>().expect("Not an integer!");
    }

    println!("{}", maxes.iter().sum::<i32>());
}
