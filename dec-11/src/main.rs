mod data;

fn main() {
    println!("Advent of Code: Day 11");

    let map = Map::new(data::DATA);
    let result1 = solve1(map);
    println!("result 1 {}", result1);
}

#[derive(Debug)]
struct Map {
    pub octos: Vec<Vec<i8>>,
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
                        .map(|c| c.to_digit(10).unwrap() as i8)
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

    fn get(&self, pos: &Point) -> i8 {
        if pos.is_within(self) {
            return self.octos[pos.y as usize][pos.x as usize];
        }
        i8::MIN
    }

    fn increase(&mut self, pos: &Point) -> u32 {
        self.octos[pos.y as usize][pos.x as usize] += 1;

        // check if flashing
        if self.get(pos) == 10 {
            return self.set_flashed(pos);
        }
        0
    }

    fn set_flashed(&mut self, pos: &Point) -> u32 {
        self.octos[pos.y as usize][pos.x as usize] = -1;

        let mut flashes = 1;
        pos.find_adjacent()
            .iter()
            // .filter(|pos| pos.is_within(&self))
            .for_each(|pos| {
                if self.get(pos) >= 0 {
                    flashes += self.increase(pos);
                }
            });
        flashes
    }

    fn reset_flashed(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Point { x, y };
                if self.get(&pos) == -1 {
                    self.octos[y as usize][x as usize] = 0;
                }
            }
        }
    }
}

#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
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
}

fn solve1(mut map: Map) -> u32 {
    println!("before\n{:?}", map);
    // let mut stack: Vec<Point> = Vec::new();
    let mut flashes: u32 = 0;

    for i in 1..=100 {
        for y in 0..map.height() {
            for x in 0..map.width() {
                let pos = Point { x, y };
                if map.get(&pos) >= 0 {
                    flashes += map.increase(&pos);
                }
            }
        }
        map.reset_flashed();
        println!("stage {}\n{:?}", i, map);
    }

    flashes
}

#[cfg(test)]
mod tests {
    use super::*;

    // PART 1
    #[test]
    fn test_solve1() {
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
        assert_eq!(solve1(map), 1656);
    }
}
