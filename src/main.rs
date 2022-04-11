use std::env;
use std::process;
use rand::{Rng};

fn main() {
    /*
        Required CLI arguments:
        0 - executable name
        1 - calculating intervals (i32)
    */
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Intervals argument is missing!");
        process::exit(1);
    }

    let intervals: i64 = args[1].parse().unwrap_or_else(| _ | {
        println!("Intervals argument does not appear to be integer!");
        process::exit(1);
    });

    if intervals <= 0 {
        println!("Intervals value should be possitive (> 0)!");
        process::exit(1);
    }

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

    let pi_value = 4.0 * (circle_points / square_points);

    println!("{}", pi_value);
}
