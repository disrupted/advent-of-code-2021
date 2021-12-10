mod data;

fn main() {
    println!("Advent of Code: Day 9");

    let floor = Floor::new(data::DATA);
    let result1 = find_low_points(&floor);
    println!("result 1 {}", result1);
}

struct Floor {
    heights: Vec<Vec<u8>>,
}

impl Floor {
    fn new(input: &str) -> Self {
        Self {
            heights: input
                .trim()
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
        }
    }

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

    fn set_visited(&mut self, point: &Point) {
        self.heights[point.y as usize][point.x as usize] = u8::MAX;
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

fn find_low_points(floor: &Floor) -> u32 {
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

fn find_basins(mut floor: Floor) -> u32 {
    let mut basins: Vec<u32> = Vec::new();

    for y in 0..floor.height() {
        for x in 0..floor.width() {
            let pos = Point { x, y };
            floor.set_visited(&pos);

            let basin_size = calc_basin_size(&floor, &pos, 0);
            basins.push(basin_size);
        }
    }
    // basins.sort_by(|a, b| b.cmp(a))
    println!("{:?}", basins);
    0
}

fn calc_basin_size(floor: &Floor, pos: &Point, mut basin_size: u32) -> u32 {
    pos.find_adjacent()
        .iter()
        .filter(|adj| floor.get_height(adj).unwrap_or(u8::MAX) < 9)
        .for_each(|adj| {
            basin_size += 1 + calc_basin_size(floor, adj, basin_size);
        });

    basin_size
}

#[cfg(test)]
mod tests {
    use super::*;

    // PART 1
    #[test]
    fn test_find_low_points() {
        const TEST_DATA: &str = "
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ";
        let floor = Floor::new(TEST_DATA);
        assert_eq!(find_low_points(&floor), 15);
    }

    // PART 2
    #[test]
    fn test_basins() {
        const TEST_DATA: &str = "
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ";
        let floor = Floor::new(TEST_DATA);
        assert_eq!(find_basins(floor), 1134);
    }
}
