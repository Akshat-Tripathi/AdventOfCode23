use std::fs;

fn parse(line: &str) -> u32 {
  let mut n = 0;
  let mut fst = 0;
  let mut lst = 0;

  for c in line.chars() {
    match c {
        '0'..='9' => {
          if n == 0 {
            fst = c.to_digit(10).unwrap();
            n = 1;
          } else {
            lst = c.to_digit(10).unwrap();
            n = 2;
          }
        },
        _ => {}
    };
  }


  if n == 1 {
    lst = fst;
  }

  println!("{} {}", fst, lst);
  10 * fst + lst
}

fn main() {
  let data = fs::read_to_string("input.txt").expect("Unable to read file");

  let mut total = 0;
  for line in data.split("\n") {
    total += parse(line);
  }

  println!("{}", total);
}
