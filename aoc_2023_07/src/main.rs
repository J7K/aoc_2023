use core::panic;
use std::env;
use std::io::*;
use std::fs::File;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

enum CardStrength
{
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

lazy_static!{
    static ref CARD_VALUES: HashMap<char, usize> = vec![
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ].into_iter().collect();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand
{
    cards: Vec<char>,
    bid: usize,
    strength: usize,    
}

impl Hand
{
    fn from(txt: &str) -> Self
    {         
        let (cards_txt, bid_txt) = txt.split_once(' ').unwrap();        
        let mut hand: Hand =  Hand { cards: cards_txt.chars().collect::<Vec<char>>(),
            bid: bid_txt.parse::<usize>().unwrap(),
            strength: 0};        

        let card_groups: HashMap<char, usize> = hand.cards.iter().fold(HashMap::new(), |mut acc, &c| { *acc.entry(c).or_insert(0) += 1; acc });
        match card_groups.len()
        {
            1 => hand.strength = CardStrength::FiveOfAKind as usize,
            2 => match card_groups.values().max().unwrap()
                {
                    4 => hand.strength = CardStrength::FourOfAKind as usize,
                    3 => hand.strength = CardStrength::FullHouse as usize,
                    _ => panic!("Invalid hand"),
                },
            3 => match card_groups.values().max().unwrap()
                {
                    3 => hand.strength = CardStrength::ThreeOfAKind as usize,
                    2 => hand.strength = CardStrength::TwoPair as usize,
                    _ => panic!("Invalid hand"),
                },
            4 => hand.strength = CardStrength::Pair as usize,
            5 => hand.strength = CardStrength::HighCard as usize,
            _=> panic!("Invalid hand"),
        }
        
        hand
    }
}

impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        
        if self.strength == other.strength
        {
            let cv1 = self.cards.iter().map(|&c| CARD_VALUES.get(&c).unwrap()).collect::<Vec<&usize>>();
            let cv2 = other.cards.iter().map(|&c| CARD_VALUES.get(&c).unwrap()).collect::<Vec<&usize>>();
            Some(cv1.cmp(&cv2))
        }
        else
        {
            Some(self.strength.cmp(&other.strength))
        }
    }
}

impl Ord for Hand
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.partial_cmp(other).unwrap()
    }
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let mut hands: Vec<_> = data.lines().map(|line| Hand::from(line)).collect::<Vec<Hand>>();
    hands.sort();
    
    let silver_ans = hands.iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i+1))
        .sum::<usize>();
    println!("Silver: {}", silver_ans);    
}