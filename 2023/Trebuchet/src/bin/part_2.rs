use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Failed to read file!");

    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let mut calval_sum: usize = 0;
    for line in contents.lines() {
        let mut calval = String::with_capacity(2);

        let match_start = re.find_iter(line);
        let matches: Vec<_> = match_start.map(|m| m.as_str()).collect();
        let start_index_last = re.find_iter(line).last().unwrap().start();
        
        let match_end = re.find_iter(&line[start_index_last+1..]);
        let end_matches: Vec<_> = match_end.map(|m| m.as_str()).collect();
        let last_match = if end_matches.len() > 0 { end_matches.last() } else { matches.last() };

        calval.push(normalize_string_digit(matches.first().unwrap()));
        calval.push(normalize_string_digit(last_match.unwrap()));
        calval_sum += calval.parse::<usize>()
            .expect("String does not represent an integer!");
    }

    println!("{}", calval_sum);
}

fn normalize_string_digit(s: &str) -> char {
    match s {
        "one"   => '1',
        "two"   => '2',
        "three" => '3',
        "four"  => '4',
        "five"  => '5',
        "six"   => '6',
        "seven" => '7',
        "eight" => '8',
        "nine"  => '9',
        nr      => nr.chars().nth(0).unwrap()
    }
}
