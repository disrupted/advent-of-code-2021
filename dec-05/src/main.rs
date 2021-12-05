fn main() {
    println!("Advent of Code: Day 5");

    create_diagram("");
}

fn str_to_int(s: &str) -> Result<u16, std::num::ParseIntError> {
    s.trim().parse()
}

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct Vent {
    start: Point,
    end: Point,
}

fn parse_coord(input: &str) -> Point {
    let coords = input
        .split(',')
        .map(|s| str_to_int(s).unwrap())
        .collect::<Vec<u16>>();

    Point {
        x: coords[0],
        y: coords[1],
    }
}

fn create_diagram(input: &str) -> &str {
    let vents = parse_vents(input);

    ""
}

fn parse_vents(input: &str) -> Vec<Vent> {
    let mut vents: Vec<Vent> = vec![];

    for line in input.trim().lines() {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        if coords.is_empty() {
            continue;
        }

        let start = parse_coord(coords[0]);
        let end = parse_coord(coords[1]);

        let vent = Vent { start, end };

        println!("{:?}", vent);

        vents.push(vent);
    }

    vents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        ";
        let diagram = ".......1..
            ..1....1..
            ..1....1..
            .......1..
            .112111211
            ..........
            ..........
            ..........
            ..........
            222111....";
        let output = create_diagram(input);

        assert!(output == diagram);
    }
}
