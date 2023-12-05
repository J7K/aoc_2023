use std::cmp::{min, max};
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

    fn redirect_range(&self, (start, end): (usize, usize)) -> Vec<(usize, usize)>
    {
        let mut ranges = vec!((start, end); 1);
        let mut mapped_ranges: Vec<(usize, usize)> = Vec::new();

        while ranges.len() > 0
        {  
            let (start, end) = ranges.pop().unwrap();
            let mut pass_thru = true;
            for map in &self.maps
            {
                let (dst, mstart, mend) = (map[0], map[1], map[1] + map[2] - 1);                                      
                let overlap_start = max(start, mstart);            
                let overlap_end = min(end, mend);
                if overlap_start <= overlap_end
                {
                    let overlap_range = overlap_end - overlap_start + 1;
                    let overlap_dst = dst + overlap_start - mstart;
                    if start < mstart
                    {
                        ranges.push((start, mstart - 1));
                    }
                    if end > mend
                    {
                        ranges.push((mend+1, end));
                    }
                    mapped_ranges.push((overlap_dst, overlap_dst + overlap_range - 1));
                    pass_thru = false;
                }
            }
            if pass_thru
            {
                mapped_ranges.push((start, end));
            }
        }   
        mapped_ranges
    }
    
    fn redirect_ranges(&self, ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)>
    {
        ranges.iter().map(|(start, end)| self.redirect_range((*start, *end))).flatten().collect::<Vec<(usize, usize)>>()
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

    let seed_ranges = seeds.chunks(2)
        .map(|x| (x[0], x[0]+x[1]-1))
        .collect::<Vec<(usize, usize)>>();
    let loc_ranges = mappers.iter()
       .fold(seed_ranges, |acc, m| m.redirect_ranges(acc));
    let gold_ans = loc_ranges.iter()
        .map(|(start, _)| start)
        .min().unwrap();
    println!("Gold: {}", gold_ans);
}