macro_rules! DATA_PATH {
    () => {
        "../data/day10.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum Token {
    ParenR,
    BracketR,
    CurlyBracketR,
    Gt,
    ParenL,
    BracketL,
    CurlyBracketL,
    Lt,
}

fn is_openning(token: &Token) -> bool {
    *token as u8 >= Token::ParenL as u8
}

fn token_match_openning_element(token: &Token, openning: &Token) -> bool {
    *openning as u8 - *token as u8 == 4
}

type Input<'a> = Vec<Token>;

fn process(raw: &str) -> Vec<Input> {
    let mut result = vec![];
    for line in raw.split('\n') {
        if line.is_empty() {
            continue;
        }
        let tokens: Vec<Token> = line
            .chars()
            .map(|c| match c {
                '(' => Token::ParenL,
                ')' => Token::ParenR,
                '[' => Token::BracketL,
                ']' => Token::BracketR,
                '{' => Token::CurlyBracketL,
                '}' => Token::CurlyBracketR,
                '<' => Token::Lt,
                '>' => Token::Gt,
                _ => unreachable!(),
            })
            .collect();
        result.push(tokens);
    }
    result
}

fn p1(input: &[Input]) -> usize {
    let closing_token_score: Vec<usize> = vec![3, 57, 1197, 25137];
    let mut sum: usize = 0;
    for tokens in input {
        let mut openning_stack = vec![];
        for token in tokens {
            if is_openning(token) {
                openning_stack.push(token);
            } else if token_match_openning_element(token, openning_stack.last().unwrap()) {
                openning_stack.pop();
            } else {
                sum += closing_token_score[*token as usize];
                break;
            }
        }
    }
    sum
}

fn p2(input: &[Input]) -> usize {
    let closing_token_point: Vec<usize> = vec![1, 2, 3, 4];
    let mut scores = vec![];
    'outer: for tokens in input {
        let mut openning_stack = vec![];
        for token in tokens {
            if is_openning(token) {
                openning_stack.push(token);
            } else if token_match_openning_element(token, openning_stack.last().unwrap()) {
                openning_stack.pop();
            } else {
                // ignore corrupted lines
                continue 'outer;
            }
        }
        let mut score: usize = 0;
        while let Some(openning_element) = openning_stack.pop() {
            score *= 5;
            score += closing_token_point[*openning_element as usize - 4]
        }
        if score > 0 {
            scores.push(score);
        }
    }
    scores.sort_unstable();
    if scores.len() % 2 == 0 {
        (scores[scores.len() / 2] + scores[scores.len() / 2 - 1]) / 2
    } else {
        scores[scores.len() / 2]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 216297);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 2165057169);
    }
}
