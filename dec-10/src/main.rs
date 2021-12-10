mod data;

fn main() {
    println!("Advent of Code: Day 10");

    let result1 = solve1(data::DATA);
    println!("result 1: {}", result1);

    let result2 = solve2(data::DATA);
    println!("result 2: {}", result2);
}

fn solve1(data: &str) -> u32 {
    data.trim()
        .lines()
        .map(find_illegal_char)
        .map(calculate_error_char_score)
        .sum()
}

fn calculate_error_char_score(c: Option<char>) -> u32 {
    match c {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        _ => 0,
    }
}

fn calculate_completion_char_score(c: char) -> u8 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn solve2(data: &str) -> u64 {
    let mut scores: Vec<u64> = data
        .trim()
        .lines()
        .filter_map(complete)
        .map(|s| calculate_completion_score(&s))
        .collect();

    let middle = scores.len() / 2;
    scores.select_nth_unstable(middle).1.to_owned()
}

fn calculate_completion_score(completion: &str) -> u64 {
    let mut score: u64 = 0;
    completion
        .chars()
        .for_each(|c| score = score * 5 + calculate_completion_char_score(c) as u64);
    score
}

fn get_matching_paren(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
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

fn complete(s: &str) -> Option<String> {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            ')' | ']' | '}' | '>' => {
                if stack.pop() != get_matching_paren(c) {
                    return None;
                }
            }
            _ => stack.push(c),
        };
    }

    // completion
    let mut completion = "".to_string();
    while let Some(c) = stack.pop() {
        if let Some(s) = get_matching_paren(c) {
            completion.push(s)
        };
    }
    Some(completion)
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
    fn test_solve1() {
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
        assert_eq!(solve1(TEST_DATA), 26397);
    }

    #[test]
    fn test_complete() {
        assert_eq!(complete("(").unwrap(), ")");
        assert_eq!(complete("[({(<(())[]>[[{[]{<()<>>").unwrap(), "}}]])})]");
        assert_eq!(complete("[(()[<>])]({[<{<<[]>>(").unwrap(), ")}>]})");
        assert_eq!(complete("(((({<>}<{<{<>}{[]{[]{}").unwrap(), "}}>}>))))");
        assert_eq!(complete("{<[[]]>}<{[{[{[]{()[[[]").unwrap(), "]]}}]}]}>");
        assert_eq!(complete("<{([{{}}[<[[[<>{}]]]>[]]").unwrap(), "])}>");
    }

    #[test]
    fn test_calculate_completion_score() {
        assert_eq!(calculate_completion_score("}}]])})]"), 288957);
        assert_eq!(calculate_completion_score(")}>]})"), 5566);
        assert_eq!(calculate_completion_score("}}>}>))))"), 1480781);
        assert_eq!(calculate_completion_score("]]}}]}]}>"), 995444);
        assert_eq!(calculate_completion_score("])}>"), 294);
    }

    #[test]
    fn test_solve2() {
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
        assert_eq!(solve2(TEST_DATA), 288957);
    }
}
