mod data;

fn main() {
    println!("Advent of Code: Day 11");

    let mut map = Map::new(data::DATA);
    let result1 = solve1(&map);
    println!("result 1 {}", result1);
}

#[derive(Debug)]
struct Map {
    octos: Vec<Vec<u8>>,
}

impl Map {
    fn new(input: &str) -> Self {
        Self {
            octos: input
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
        self.octos.len() as isize
    }

    fn width(&self) -> isize {
        self.octos[0].len() as isize
    }
}

// struct Octo {
//     energy: u8,
//     pos: Point,
// }

struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn is_within(&self, map: &Map) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < map.width() && self.y < map.height()
    }

    fn find_adjacent(&self) -> [Point; 8] {
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
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
    }

    // fn find_adjacent(&self) -> [(isize, isize); 8] {
    //     [
    //         (self.x - 1, self.y),
    //         (self.x + 1, self.y),
    //         (self.x, self.y - 1),
    //         (self.x, self.y + 1),
    //         (self.x - 1, self.y - 1),
    //         (self.x + 1, self.y + 1),
    //         (self.x + 1, self.y - 1),
    //         (self.x - 1, self.y + 1),
    //     ]
    // }
}

fn solve1(map: &Map) -> u32 {
    println!("{:?}", map);
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // PART 1
    #[test]
    fn test_find_low_points() {
        const TEST_DATA: &str = "
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        ";
        let map = Map::new(TEST_DATA);
        assert_eq!(solve1(&map), 1656);
    }
}