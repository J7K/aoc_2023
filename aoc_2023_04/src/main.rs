use std::collections::VecDeque;
use std::env;
use std::io::*;
use std::fs::*;

struct ScratchCard
{
    winning_numbers: Vec<u32>,
    ticket_numbers: Vec<u32>,
}

impl ScratchCard
{
    fn from(line: &str) -> ScratchCard
    {
        let (_, nums) = line.split_once(": ").unwrap();
        let (winning, ticket) = nums.split_once(" | ").unwrap();
        let wn: Vec<_> = winning.split_whitespace().map(|n| n.trim().parse::<u32>().unwrap()).collect();
        let tn: Vec<_> = ticket.split_ascii_whitespace().map(|n| n.trim().parse::<u32>().unwrap()).collect();

        ScratchCard { winning_numbers: wn, ticket_numbers: tn }
    }

    fn nb_matches(&self) -> usize
    {
        self.ticket_numbers.iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count()
    }

    fn score(&self) -> u32
    {
        let matches = self.nb_matches() as u32;
        if matches > 0 { 2_u32.pow(matches-1) } else { 0 }        
    }
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    let cards: Vec<_> = data.lines()
        .map(|line| ScratchCard::from(line))
        .collect();

    let silver_ans: u32 = cards.iter()
        .map(|card| card.score())
        .sum();
    println!("Silver: {}", silver_ans);

 
    let mut winnings:Vec<Vec<usize>> = Vec::new();   
    for (i, c) in cards.iter().enumerate()
    {
        let v = (i+1..=(i+c.nb_matches() as usize)).collect();
        winnings.push(v);
    }

    let mut gold_ans = 0;
    let mut acc:VecDeque<usize> = (0..cards.len()).collect();
    while !acc.is_empty()
    {
        gold_ans += 1;
        let id = acc.pop_front().unwrap();        
        acc.extend(winnings[id].iter());
    }
    println!("Gold: {}", gold_ans);
}