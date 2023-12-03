use std::collections::HashSet;
use std::env;
use std::io::*;
use std::fs::*;

struct Schematics
{
    matrix: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
}

impl Schematics
{
    fn from(data: &String) -> Schematics
    {
        Schematics { matrix: data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>(),
                     max_x: data.lines().next().unwrap().len(),
                     max_y: data.lines().count() }
    }

    fn get(&self, x: usize, y: usize) -> char
    {
        self.matrix[y][x]
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)>
    {
        let mut neighbors = Vec::new();
        let directions = [
                    (-1, -1), (0, -1), (1, -1),
                    (-1, 0),           (1, 0),
                    (-1, 1),  (0, 1),  (1, 1),
                ];

        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < self.max_x as isize && ny >= 0 && ny < self.max_y as isize 
            {
                neighbors.push((nx as usize, ny as usize));
            }
        }
        neighbors
    }

    fn get_first_digit_coords(&self, x: usize, y: usize) -> (usize, usize)
    {
        assert!(self.get(x, y).is_digit(10), "Not a digit");
        let mut nx = x;
        while nx > 0 && self.get(nx-1, y).is_digit(10)
        {
            nx -= 1;
        }

        (nx, y)
    }

    fn get_part_at(&self, x: usize, y: usize) -> u32
    {
        assert!(self.get(x, y).is_digit(10), "Not a digit");
        let mut acc: Vec<char> = vec![self.get(x, y)];
        let mut nx = x + 1;
        while nx < self.max_x && self.get(nx, y).is_digit(10)
        {
            acc.push(self.get(nx, y));
            nx += 1;
        }
        acc.iter().collect::<String>().parse::<u32>().unwrap()
    }

    fn parse_part_numbers(&self) -> Vec<u32>
    {
        let mut parts = HashSet::new();

        for y in 0..self.max_y
        {
            for x in 0..self.max_x
            {
                let c = self.get(x, y);
                if !c.is_digit(10) && c != '.'
                {
                    let adjacent_nbs = self.neighbors(x, y).into_iter()
                        .filter(|(nx, ny)| self.get(*nx, *ny).is_digit(10))
                        .collect::<Vec<_>>();
                    for (x, y) in adjacent_nbs
                    {
                        parts.insert(self.get_first_digit_coords(x, y));
                    }                   
                }
            }
        }
        parts.iter().map(|(x, y)| self.get_part_at(*x, *y)).collect::<Vec<_>>()
    }
    

    fn parse_ratios(&self) -> Vec<u32> 
    {
        let mut ratios = Vec::new();

        for y in 0..self.max_y
        {
            for x in 0..self.max_x
            {
                if self.get(x, y) == '*'
                {
                    let adjacent_nbs = self.neighbors(x, y).into_iter()
                        .filter(|(nx, ny)| self.get(*nx, *ny).is_digit(10))
                        .collect::<Vec<_>>();

                    let mut part_set: HashSet<(usize, usize)> = HashSet::new();
                    for pos in adjacent_nbs
                    {
                        part_set.insert(self.get_first_digit_coords(pos.0, pos.1));
                    }
                    if part_set.len() == 2
                    {
                        ratios.push(part_set.iter().map(|(x, y)| self.get_part_at(*x, *y)).product::<u32>());
                    }
                }
            }
        }

        ratios
    }

}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let silver_ans = Schematics::from(&data)
        .parse_part_numbers()
        .iter()
        .sum::<u32>();
    println!("Silver: {}", silver_ans);

   let gold_ans = Schematics::from(&data)
        .parse_ratios()
        .iter()
        .sum::<u32>();
    println!("Gold: {}", gold_ans);
}