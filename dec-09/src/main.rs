mod data;

fn main() {
    println!("Advent of Code: Day 9");

    let floor = Floor::new(data::DATA);
    let result1 = calc_risk(&floor);
    println!("result 1 {}", result1);
}

#[derive(Clone)]
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

fn find_low_points(floor: &Floor) -> Vec<Point> {
    let mut low_points = Vec::new();

    for y in 0..floor.height() {
        for x in 0..floor.width() {
            let pos = Point { x, y };
            let height = floor.get_height(&pos).unwrap();

            pos.find_adjacent()
                .iter()
                .all(|adj| floor.get_height(adj).unwrap_or(u8::MAX) > height)
                .then(|| low_points.push(pos));
        }
    }
    low_points
}

fn calc_risk(floor: &Floor) -> u32 {
    find_low_points(floor)
        .iter()
        .map(|p| floor.get_height(p).unwrap() as u32 + 1)
        .sum()
}

fn find_basins(floor: Floor) -> u32 {
    let mut basins = find_low_points(&floor)
        .iter()
        .map(|pos| {
            // floor.set_visited(pos);
            calc_basin_size(floor.clone(), pos, 0)
        })
        .collect::<Vec<u32>>();

    basins.sort_by(|a, b| b.cmp(a));
    basins.iter().take(3).product()
}

fn calc_basin_size(mut floor: Floor, pos: &Point, mut basin_size: u32) -> u32 {
    floor.set_visited(pos);

    pos.find_adjacent()
        .iter()
        .filter(|adj| floor.get_height(adj).unwrap_or(u8::MAX) < 9)
        .for_each(|adj| {
            if floor.get_height(adj).unwrap_or(u8::MAX) < 9 {
                basin_size += 1 + calc_basin_size(floor.clone(), adj, basin_size);
            }
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
        assert_eq!(calc_risk(&floor), 15);
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
