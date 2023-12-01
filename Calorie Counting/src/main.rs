use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Failed to read file!");

    let mut max: i32 = 0;
    let mut count: i32 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            if count > max {
                max = count;
            }
            count = 0;
            continue;
        }
        count += line.parse::<i32>().expect("Not an integer!");
    }

    println!("{}", max);
}
