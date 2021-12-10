mod data;

fn main() {
    println!("Advent of Code: Day 9");

    let result = find_low_points(data::DATA);
    println!("{}", result);
}

fn find_low_points(data: &str) -> u32 {
    let heights: Vec<Vec<u8>> = data
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut risk_level: u32 = 0;

    for y in 0..heights.len() {
        for x in 0..heights[y].len() {
            let current = heights[y][x];
            // check if current point is lower than surrounding points
            if (y == 0 || current < heights[y - 1][x]) // up
            && (x == 0 || current < heights[y][x - 1]) // left
            && (x + 1 == heights[y].len() || current < heights[y][x + 1]) // right
            && (y + 1 == heights.len() || current < heights[y + 1][x])
            // down
            {
                risk_level += 1 + current as u32;
            }
        }
    }
    risk_level
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
