pub fn fibonacci_sequence(n: i32) -> Vec<i32> {
    let mut seq = vec![0, 1];

    while seq[seq.len() - 1] + seq[seq.len() - 2] <= n {
        seq.push(seq[seq.len() - 1] + seq[seq.len() - 2]);
    }

    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_sequence_negative_iterations() {
        assert_eq!(fibonacci_sequence(-1), vec![0, 1]);
    }

    #[test]
    fn fibonacci_sequence_zero_iterations() {
        assert_eq!(fibonacci_sequence(0), vec![0, 1]);
    }

    #[test]
    fn fibonacci_sequence_leq_second_element() {
        assert_eq!(fibonacci_sequence(1), vec![0, 1, 1]);
    }

    #[test]
    fn fibonacci_sequence_positive_case() {
        assert_eq!(fibonacci_sequence(50), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
