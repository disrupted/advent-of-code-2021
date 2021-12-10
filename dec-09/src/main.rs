mod data;

fn main() {
    println!("Advent of Code: Day 9");

    let result1 = find_low_points(data::DATA);
    println!("result 1 {}", result1);
}

struct Floor {
    heights: Vec<Vec<u8>>,
}

impl Floor {
    fn height(&self) -> isize {
        self.heights.len() as isize
    }

    fn width(&self) -> isize {
        self.heights[0].len() as isize
    }

    fn get_height(&self, point: &Point) -> Option<u8> {
        if point.is_within(self) {
            return Some(self.heights[point.y as usize][point.x as usize]);
        }
        None
    }
}

struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn is_within(&self, floor: &Floor) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < floor.width() && self.y < floor.height()
    }

    fn find_adjacent(&self) -> [Point; 4] {
        [
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

fn find_low_points(data: &str) -> u32 {
    let floor = Floor {
        heights: data
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    };

    let mut risk_level: u32 = 0;

    for y in 0..floor.height() {
        for x in 0..floor.width() {
            let pos = Point { x, y };
            let height = floor.get_height(&pos).unwrap();

            if pos
                .find_adjacent()
                .iter()
                .all(|adj| floor.get_height(adj).unwrap_or(u8::MAX) > height)
            {
                println!("{}", height);
                risk_level += 1 + height as u32;
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
