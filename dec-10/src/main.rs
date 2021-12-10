mod data;

fn main() {
    println!("Advent of Code: Day 10");

    // let result = solve(data::DATA);
    // println!("{}", result);
}

fn solve_1(data: &str) -> u32 {
    data.trim()
        .lines()
        .map(find_illegal_char)
        .map(calculate_score)
        .sum()
}

fn get_matching_paren(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

fn is_matching_paren(s: &str) -> bool {
    let char = find_illegal_char(s);
    if char.is_none() {
        return true;
    }
    false
}

fn find_illegal_char(s: &str) -> Option<char> {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            ')' | ']' | '}' | '>' => {
                if stack.pop() != get_matching_paren(c) {
                    return Some(c);
                }
            }
            _ => stack.push(c),
        };
    }
    None
}

fn calculate_score(c: Option<char>) -> u32 {
    match c {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_matching_paren() {
        assert!(is_matching_paren("()"));
        assert!(is_matching_paren("([{<>}])"));
        assert!(!is_matching_paren("(]"));
    }

    #[test]
    fn test_solve_1() {
        const TEST_DATA: &str = "
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        ";
        assert_eq!(solve(TEST_DATA), 26397);
    }
}
