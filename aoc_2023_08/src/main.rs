use std::env;
use std::io::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use gcd::euclid_usize;

#[derive(Debug)]
struct Node<'a> 
{
    value: &'a str,
    left: &'a str,
    right: &'a str
}
impl Node<'_>
{
    fn from(line: &str) -> Node
    {
        let nodes  = Regex::new(r"([A-Z0-9]{2}[A-Z])").unwrap()
            .captures_iter(line)
            .map(|capture| capture.get(1).unwrap().as_str())
            .collect::<Vec<&str>>();
        assert!(nodes.len() == 3, "Invalid line");
        Node {  value: nodes[0], left: nodes[1], right: nodes[2] }
    }
}

struct NodeMap<'a>
{
    nodes: HashMap<&'a str, (&'a str, &'a str)>
}
impl NodeMap<'_>
{
    fn from(nodes_txt: &str) -> NodeMap
    {
        let nodes: Vec<Node> = nodes_txt.lines()
            .map(|line| Node::from(line))
            .collect();

        let nodes_map: HashMap<&str, (&str, &str)> = nodes.iter()
            .map(|node| (node.value, (node.left, node.right)))
            .collect();
        NodeMap { nodes: nodes_map }
    }
    
    fn follow_cycle<'a>(&'a self, start_node: &'a str, path: &Vec<char>) -> Vec<usize>
    {
        let mut cycle: Vec<&str> = Vec::new();
        let mut cycle_count: Vec<usize> = Vec::new();

        let mut steps_count = 0;
        let mut current_step = 0;
        let mut current_node = start_node;
        let mut keep_looping = true;

        while  keep_looping
        {            
            while current_node.chars().last().unwrap() != 'Z'
            {
                let (left, right) = self.nodes.get(current_node).unwrap();
                if path[current_step] == 'L' 
                {
                    current_node = left; 
                }
                else 
                {
                    current_node = right; 
                }
                current_step += 1;
                if current_step == path.len() 
                {
                    current_step = 0;
                }
                steps_count += 1;
            }
            if cycle.contains(&current_node) 
            {
                keep_looping = false;
            }
            cycle.push(current_node);
            cycle_count.push(steps_count);
            steps_count = 0;
        }
        cycle_count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((args.len() > 1), "Missing input file argument");

    let file = File::open(&args[1]).unwrap(); 
    let mut data: String = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Failed to read file");

    let (path_txt, nodes_txt) = data.split_once("\r\n\r\n").unwrap();
    let path: Vec<char> = path_txt.chars().collect();

    let node_map = NodeMap::from(nodes_txt);
    let cycles: Vec<_> = node_map.nodes.iter()
        .filter(|(node, _)| node.chars().last().unwrap() == 'A')
        .map(|(node, _)| node_map.follow_cycle(node, &path))
        .collect();

    // In data set, all cycles are: start -> end -> end with equal length between start->end and end->end
    // So, they'll all come in sync when the number of steps is the smallest common multiple of all the periods.
    let periods: Vec<usize> = cycles.iter()
        .map(|cycle| cycle[0])
        .collect();

    let gold_ans_lcm = periods.iter().skip(1).fold(periods[0], |acc, p| acc * p / euclid_usize(acc, *p));    

    println!("Gold: {}", gold_ans_lcm);
}
