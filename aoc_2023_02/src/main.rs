use std::env;
use std::io::*;
use std::fs::*;
use std::collections::HashMap;
use std::cmp::max;

struct Game
{
    rounds: Vec<HashMap<String, u32>>,
}

impl Game
{
    fn new() -> Game
    {
        Game { rounds: Vec::new() }
    }

    fn from(line: &str) -> Game
    {
        let mut game = Game::new();
        let rounds: Vec<_> = line
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split(';')
            .collect();

         for r in rounds
         {
             let mut round = HashMap::new();
             let data: Vec<_> = r.split(',').collect();
             for d in data
             {
                 let kv: Vec<_> = d[1..].split(' ').collect();
                 round.insert(kv[1].to_string(), kv[0].parse::<u32>().unwrap());
             }
             game.rounds.push(round);
        }
        game
    }    

    fn is_round_valid(&self, round: &HashMap<String, u32>, max_red: u32, max_green: u32, max_blue: u32) -> bool
    {
        let (red, green, blue) = Game::round_rgb(round);

        red <= max_red && green <= max_green && blue <= max_blue
    }

    fn is_game_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool
    {
        self.rounds.iter().all(|r| self.is_round_valid(r, max_red, max_green, max_blue))        
    }

    fn round_rgb(round: &HashMap<String, u32>) -> (u32, u32, u32)
    {
        let red = round.get("red").unwrap_or(&0);
        let green = round.get("green").unwrap_or(&0);
        let blue = round.get("blue").unwrap_or(&0);

        (*red, *green, *blue)
    }

    fn min_req_cubes(&self) -> (u32, u32, u32)
    {
        self.rounds.iter()
            .map(|r| Game::round_rgb(r))
            .fold((0,0,0), |acc, rgb| (max(acc.0, rgb.0), max(acc.1, rgb.1), max(acc.2, rgb.2)))
    }
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let games = data.lines().map(|l| Game::from(l)).collect::<Vec<_>>();
    let silver_ans = games.iter()
        .enumerate()
        .filter(|(_,g)| g.is_game_valid(12, 13, 14))
        .map(|(i,_)| i+1)
        .sum::<usize>();

    println!("Silver: {}", silver_ans);
    
    let gold_ans = games.iter()
        .map(|g| g.min_req_cubes())
        .map(|(r,g,b)| r*g*b)
        .sum::<u32>();
    println!("Gold: {}", gold_ans);
}