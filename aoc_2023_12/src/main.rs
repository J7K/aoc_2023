use std::env;
use std::io::*;
use std::fs::File;

struct SpringRow
{
    springs : Vec<char>,
    control_vec : Vec<usize>,
}

impl SpringRow
{
    fn from(data: &str) -> Self
    {
        let (springs_txt, control_txt) = data.split_once(' ').unwrap();
        let springs: Vec<char> = springs_txt.chars().collect();
        let control_vec: Vec<usize> = control_txt.split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Self { springs, control_vec }
    }

    fn from_unfold(data: &str) -> Self
    {
        let mut row = SpringRow::from(data);
        let springs = row.springs.clone();
        let control_vec = row.control_vec.clone();
        for _ in 0..4 
        {
            row.springs.extend(springs.iter());
            row.control_vec.extend(control_vec.iter());
        }
        row
    }

    fn extract_groups(&self) -> Vec<usize>
    {
        let mut groups: Vec<usize> = Vec::new();
        let mut group_len = 0_usize;

        for c in self.springs.iter()
        {
            if *c == '#' { group_len += 1; }
            else if *c == '.' && group_len > 0 
            { 
                groups.push(group_len);
                group_len = 0; 
            }
        }
        if group_len > 0 { groups.push(group_len); }

        groups
    }

    fn verify(&self) -> bool
    {
        let groups = self.extract_groups();
        if groups.len() != self.control_vec.len() { return false; }

        groups.iter().zip(self.control_vec.iter()).all(|(a, b)| a == b)
    }

    fn verify_against(&self, arr: &Vec<char>) -> bool
    {
        let row = SpringRow { springs: arr.clone(), control_vec: self.control_vec.clone() };
        row.verify()
    }

    fn operational_arrangements(&self) -> Vec<Vec<char>>
    {
        let option_count = self.springs.iter().filter(|c| **c == '?').count();
        let mut arrangements: Vec<Vec<char>> = Vec::new();

        for i in 0..(2_usize.pow(option_count as u32))
        {
            let mut arr = self.springs.clone();
            let mut option_bits = i;
            for c in arr.iter_mut()
            {
                if *c == '?'
                {
                    *c = if option_bits % 2 == 0 { '.' } else { '#' };
                    option_bits /= 2;
                }
            }
            if self.verify_against(&arr) 
            {
                arrangements.push(arr.clone());
            }
        }
        arrangements
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let spring_rows: Vec<SpringRow> = data.lines().map(|l| SpringRow::from(l)).collect();
    let silver : usize = spring_rows.iter()
        .map(|r| r.operational_arrangements().len())
        .sum();
    dbg!(silver);

    let unfolded_rows: Vec<SpringRow> = data.lines().map(|l| SpringRow::from_unfold(l)).collect();
    let gold : usize = unfolded_rows.iter()
        .map(|r| r.operational_arrangements().len())
        .sum();
    dbg!(gold);
}