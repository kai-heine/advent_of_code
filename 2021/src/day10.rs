const INPUT: &str = include_str!("../inputs/day10");

enum LineValidity {
    Valid,
    Incomplete(Vec<char>),
    SyntaxError(char),
}

fn check_line(line: &str) -> LineValidity {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if stack.pop().unwrap_or('!') != c {
                    return LineValidity::SyntaxError(c);
                }
            }
            _ => panic!("unexpected character"),
        }
    }

    if !stack.is_empty() {
        stack.reverse();
        return LineValidity::Incomplete(stack);
    }

    LineValidity::Valid
}

fn puzzle1(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        match check_line(line) {
            LineValidity::SyntaxError(c) => {
                score += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("unexpected character"),
                }
            }
            _ => {}
        }
    }

    score
}

fn puzzle2(input: &str) -> i64 {
    let mut scores: Vec<i64> = input
        .lines()
        .filter_map(|line| match check_line(line) {
            LineValidity::Incomplete(missing_chars) => {
                let score = missing_chars.iter().fold(0, |score, c| {
                    score * 5
                        + match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => panic!("unexpected character"),
                        }
                });
                Some(score)
            }
            _ => None,
        })
        .collect();

    scores.sort();
    scores[(scores.len() - 1) / 2]
}

pub fn day10() {
    println!("\nDay 10:");

    println!("Puzzle 1:");
    let syntax_error_score = puzzle1(&INPUT);
    println!("The total syntax error score is {}", syntax_error_score);

    println!("Puzzle 2:");
    let middle_score = puzzle2(&INPUT);
    println!("The middle score of incomplete lines is {}", middle_score);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn puzzle1_example() {
        let syntax_error_score = puzzle1(&TEST_INPUT);
        assert_eq!(syntax_error_score, 26397);
    }

    #[test]
    fn puzzle2_example() {
        let incomplete_score = puzzle2(&TEST_INPUT);
        assert_eq!(incomplete_score, 288957);
    }
}
