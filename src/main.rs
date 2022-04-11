use std::env;
use std::process;
use rand::{Rng};

fn calculate_pi_monte_carlo(intervals: i64) -> f64 {
    let mut current_interval: i64 = 0;
    let mut circle_points: f64 = 0.0;
    let mut square_points: f64 = 0.0;
    let mut rng = rand::thread_rng();

    while current_interval < intervals {
        let rand_x = rng.gen_range(0.0..1.0);
        let rand_y = rng.gen_range(0.0..1.0);

        let origin_dist: f64 = f64::powf(rand_x, 2.0) + f64::powf(rand_y, 2.0);

        if origin_dist <= 1.0 { circle_points += 1.0; }

        square_points += 1.0;
        current_interval += 1;
    }

    return 4.0 * (circle_points / square_points);
}

fn parse_intervals_from_args(args: Vec<String>) -> Result<i64, String> {
    if args.len() < 2 { return Err("Intervals argument is missing!".to_string()); }

    match args[1].parse() {
        Ok(intervals) if intervals <= 0 => Err("Intervals value should be possitive (> 0)!".to_string()),
        Ok(intervals) => Ok(intervals),
        Err(_) => Err("Intervals argument does not appear to be integer!".to_string())
    }
}

fn main() {
    /*
        Required CLI arguments:
        0 - executable name
        1 - calculating intervals (i32)
    */
    let args: Vec<String> = env::args().collect();
    let intervals = parse_intervals_from_args(args).unwrap_or_else( |err| { println!("{}", err); process::exit(1); });
    let pi_value = calculate_pi_monte_carlo(intervals);
    println!("{}", pi_value);
}
