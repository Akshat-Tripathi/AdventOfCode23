use regex::Regex;
use std::fs;

fn parse_line(line: &str) -> Vec<u32> {
    let mut v = Vec::new();

    //let re = Regex::new(r"[0-9]").unwrap();
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();

    for i in 0..line.len() {
        let Some(digit) = re.find_at(line, i) else {
            continue;
        };

        let digit = match digit.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            c => c.parse().unwrap(),
        };

        v.push(digit);
    }

    v
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    //let re = Regex::new(r"[0-9]").unwrap();
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();

    let mut total = 0;
    for line in data.split("\n") {
        let captures = parse_line(line);
        if captures.len() == 0 {
            continue;
        }

        let fst = captures.first().unwrap();
        let lst = captures.last().unwrap();

        println!("{:?}", captures);

        total += 10 * fst + lst;
    }

    println!("{}", total);
}
