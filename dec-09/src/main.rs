fn main() {
    println!("Advent of Code: Day 9");
}

fn find_low_points(data: &str) -> u32 {
    let heights: Vec<Vec<u32>> = data
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    println!("{:?}", heights);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_low_points() {
        const TEST_DATA: &str = "
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ";
        assert_eq!(find_low_points(TEST_DATA), 15);
    }
}
