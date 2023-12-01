use std::collections::VecDeque;
use std::env;
use std::io::*;
use std::fs::*;

fn read_line_silver(line: &str) -> u32 {
    let nums: VecDeque<u32> = line.chars()
                            .filter(|c| c.is_digit(10))
                            .map(|c| c.to_digit(10).unwrap())
                            .collect();
    assert!(nums.len() > 0, "No numbers found");

    nums.front().unwrap() * 10 + nums.back().unwrap()
}

fn read_line_gold(line: &str) -> u32 {
    let written_digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut nums: VecDeque<u32> = VecDeque::new();
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            nums.push_back(c.to_digit(10).unwrap());
        }
        else {
            let s = String::from(&line[i..]);
            for (j, wd) in written_digits.iter().enumerate() {
                if s.starts_with(wd) {
                    nums.push_back((j) as u32);
                    break;
                }
            }
        }
    }
                                                 

    assert!(nums.len() > 0, "No numbers found");
    nums.front().unwrap() * 10 + nums.back().unwrap()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let silver_ans : u32 = data.lines().map(|line| read_line_silver(line)).sum();    
    println!("Silver: {}", silver_ans);
    
    let gold_ans : u32 = data.lines().map(|line| read_line_gold(line)).sum();
    println!("Gold: {}", gold_ans);
}