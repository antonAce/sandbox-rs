use rand::Rng;

pub fn estimate_pi(iters: i32) -> f32 {
    let mut rng = rand::thread_rng();
    let mut circle_hits: i32 = 0; // How many points lay in a circle

    if iters < 1 {
        return 0.0;
    }

    for _ in 0..iters {
        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);

        let distance_to_origin: f32 = f32::powf(x, 2.0) + f32::powf(y, 2.0);

        if distance_to_origin <= 1.0 {
            circle_hits += 1;
        }
    }

    4.0 * (circle_hits as f32) / (iters as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimate_pi_negative_iterations() {
        assert_eq!(estimate_pi(-1), 0.0);
    }

    #[test]
    fn estimate_pi_zero_iterations() {
        assert_eq!(estimate_pi(0), 0.0);
    }

    #[test]
    fn estimate_pi_test_accuracy() {
        assert!(estimate_pi(10000) - std::f32::consts::PI < 0.05);
    }
}
