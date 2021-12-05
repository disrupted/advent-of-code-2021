fn main() {
    println!("Advent of Code: Day 3");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_rate() {
        let gamma = "
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        ";

        let output = gamma_rate(gamma);
        assert!(output == 5);
    }
}
