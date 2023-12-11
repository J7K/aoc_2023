use std::env;
use std::io::*;
use std::fs::File;

fn derive(sequence: &Vec<isize>) -> Vec<isize>
{
    let mut next_seq: Vec<isize> = vec![0; sequence.len() - 1];
    for i in 0..(sequence.len()-1)
    {
        next_seq[i] = sequence[i+1] - sequence[i];
    }
    next_seq
}

fn compute_next(sequence: &Vec<isize>) -> isize
{
    let mut stack: Vec<isize> = Vec::new();
    let mut current_seq = sequence.clone();
    while !current_seq.iter().all(|x| *x == 0)
    {
        stack.push(*current_seq.last().unwrap());
        current_seq = derive(&current_seq);
    }
    stack.iter().sum()
}

fn compute_first(sequence: &Vec<isize>) -> isize
{
    let mut stack: Vec<isize> = Vec::new();
    let mut current_seq = sequence.clone();
    while !current_seq.iter().all(|x| *x == 0)
    {
        stack.push(*current_seq.first().unwrap());
        current_seq = derive(&current_seq);
    }
    stack.reverse();
    stack.iter().fold(0_isize, |acc, v| v - acc)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");
    
    let lines: Vec<Vec<isize>>= data.lines().map(|l| l.split(' ').map(|x| x.parse::<isize>().unwrap()).collect()).collect();
    let silver: isize = lines.iter().map(|l| compute_next(l)).sum();
    dbg!(silver);
    let gold: isize = lines.iter().map(|l| compute_first(l)).sum();
    dbg!(gold);
}