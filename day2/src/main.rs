use std::fs;
use std::cmp;
use regex::Regex;

struct Game {
    id: u32,
    turns: Vec<(u32, u32, u32)>
}

fn parse_turn(turn: &str) -> (u32, u32, u32) {
    let re = Regex::new(r"([0-9]+) (.)").unwrap();
    let v = turn.split(", ").collect::<Vec<&str>>();

    let (mut r, mut g, mut b) = (0, 0, 0);

    for bs in v {
        let caps = re.captures(bs).unwrap();
        let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let colour = caps.get(2).unwrap().as_str();

        match colour {
            "r" => { r += n },
            "g" => { g += n },
            "b" => { b += n },
            _   => unreachable!()
        };
    }

    (r, g, b)
}

fn parse_game(line: &str) -> Game {
    let re = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    let Some(caps) = re.captures(line) else {
        return Game{id: 0, turns: Vec::new()};
    };


    let id = caps.get(1).unwrap().as_str().parse().unwrap();
    let turns = caps.get(2).unwrap().as_str().split(";").map(|turn| parse_turn(turn.trim())).collect();
    
    Game {
        id: id,
        turns: turns
    }
}

fn main() {
    let data = fs::read_to_string("input1.txt").expect("Unable to read file");

    let (tr, tg, tb) = (12, 13, 14);
    let mut total = 0;

    for line in data.split("\n") {
        let game = parse_game(line);

        // Part 1
        /*
        if game.turns.into_iter().all(|(r, g, b)| r <= tr && g <= tg && b <= tb) {
            total += game.id;
            println!("Adding {}", game.id);
        }
        */

        // Part 2
        let (mut mr, mut mg, mut mb) = (0, 0, 0);
        for (r, g, b) in game.turns {
            mr = cmp::max(mr, r);
            mg = cmp::max(mg, g);
            mb = cmp::max(mb, b);
        }

        let power = mr * mg * mb;
        total += power;
    }
    
    println!("{}", total);
}
