static DATA: &str="Time:        46     68     98     66
Distance:   358   1054   1807   1080
";

static TOTAL_TIME: usize = 46689866;
static DIST_TO_BEAT: usize = 358105418071080;

fn binary_search(range_start: usize, range_end: usize, total_time : f64, dist_to_beat: f64) -> usize
{
    if range_start == range_end
    {
        range_start
    }
    else
    {
        let range_mid = (range_start + range_end) / 2;
        let mid = range_mid as f64;
        if (total_time - mid) * mid > dist_to_beat
        {
            binary_search(range_start,range_mid , total_time, dist_to_beat)
        }
        else
        {
            binary_search(range_mid+1, range_end, total_time, dist_to_beat)
        }
    }
}
fn main() {
    let lines: Vec<&str> = DATA.lines().collect();
    let (_, time_txt) = lines[0].split_once(':').unwrap();
    let (_, dist_txt) = lines[1].split_once(':').unwrap();
    
    let times: Vec<usize> = time_txt.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let dists: Vec<usize> = dist_txt.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    
    let mut better_ways: Vec<usize> = Vec::new();
    for (t,d) in times.iter().zip(dists.iter())
    {
        let nb_wins = (0..*t).map(|x| (t-x)*x).filter(|x| x > d).count();
        better_ways.push(nb_wins);
    }
    
    let silver_ans: usize = better_ways.iter().product();
    println!("Silver Ans: {0}", silver_ans);
    
    let gold_ans_brute_force = (0..TOTAL_TIME).map(|x| (TOTAL_TIME-x)*x).filter(|x| *x > DIST_TO_BEAT).count();
    println!("Gold Ans - Brute Force: {0}", gold_ans_brute_force);
    
    // Dist = (total_time - x) * x.
    // Plot is a parabola centered on total_time/2
    // We want to find the first time t0 for which the distance is greater than the distance to beat.
    // Since the parabola is symmetric, all time values that give a better distance than the recorded one are in the range [t0, total_time/2 + t0 ],
    // this is also equal to [t0, total_time-t0 +1]
    // We can also limit search space to [0, total_time/2], for symmetry reasons.
    let first_time_better = binary_search(0,TOTAL_TIME/2, TOTAL_TIME as f64, DIST_TO_BEAT as f64);
    let gold_ans_smarter = TOTAL_TIME - 2*first_time_better + 1;
    println!("Gold Ans - Binary binary_search: {0}",  gold_ans_smarter);
}
