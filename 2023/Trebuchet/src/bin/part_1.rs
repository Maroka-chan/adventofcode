use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Failed to read file!");

    let mut calval_sum: usize = 0;
    for line in contents.lines() {
        let mut calval = String::with_capacity(2);
        let first_digit_index: usize =
            line.find(|s: char| s.is_digit(10))
                .expect("No digit in string.");
        let last_digit_index: usize =
            line.rfind(|s: char| s.is_digit(10))
                .unwrap();
        calval.push(line.chars().nth(first_digit_index).unwrap());
        calval.push(line.chars().nth(last_digit_index).unwrap());
        calval_sum += calval.parse::<usize>()
            .expect("String does not represent an integer!");
    }

    println!("{}", calval_sum);
}
