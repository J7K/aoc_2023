use std::env;
use std::io::*;
use std::fs::File;

struct Mapper
{
    maps: Vec<Vec<usize>>,
}

impl Mapper
{
    fn from(txt: &str) -> Self
    {
        Self { maps: txt.lines().skip(1)
            .map(|l| l.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>()
        }
    }

    fn redirect(&self, input: usize) -> usize
    {
        let mut val = input;
        for map in &self.maps
        {
            let (dst, src, range) = (map[0], map[1], map[2]);
            if val >= src && val <= src + range
            {
                val = dst + (val - src);
                break;
            }
        }
        val
    }
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let(seeds_txt, mappers_txt) = data.split_once("\r\n\r\n").unwrap();
    let(_, seed_data) = seeds_txt.split_once(": ").unwrap();
    let seeds = seed_data.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mappers = mappers_txt.split("\r\n\r\n").map(|x| Mapper::from(x)).collect::<Vec<Mapper>>();
    assert!(mappers.len() == 7, "Expected 7 mappers");

    let silver_ans = seeds.iter()
        .map(|&s| mappers.iter().fold(s, |acc, m| m.redirect(acc)))
        .min()
        .unwrap();
    println!("Silver: {}", silver_ans);

    
//    println!("Gold: {}", gold_ans);
}