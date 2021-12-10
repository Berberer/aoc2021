fn input_data() -> Vec<&'static str> {
    // Replace these input chunks with a different set of bracket combinations
    vec![
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ]
}

fn parse_input_data(input_lines: Vec<&str>) -> Vec<Vec<char>> {
    // Split input into lists of bracket chars
    input_lines
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn check_line_chunks(line: &Vec<char>) -> Result<Vec<char>, char> {
    // Parse line chunks into matching bracket stack
    // Return Ok(empty stack) in case of a valid line
    // Return Ok(remaining stack) in case an incomplete line
    // Return Err(Illegal character) in case of a corrupted line
    let mut chunk_stack = Vec::new();
    for c in line.clone() {
        match c {
            '(' | '[' | '{' | '<' => chunk_stack.push(c),
            _ => match (chunk_stack.last().unwrap(), c) {
                ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                    chunk_stack.pop();
                }
                _ => {
                    return Err(c);
                }
            },
        };
    }
    Ok(chunk_stack)
}

fn calculate_syntax_error_score(line_chunks: &Vec<Result<Vec<char>, char>>) -> i64 {
    // Calculate syntax error score based on the illegal characters
    line_chunks
        .iter()
        .map(|result| match result {
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            _ => 0,
        })
        .sum()
}

fn calculate_line_autocomplete_score(line_stack: Vec<char>) -> i64 {
    // Calculate the autocomplete score of the line based on the characters the remaining stack
    line_stack.iter().rev().fold(0, |autocomplete_score, c| {
        autocomplete_score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
    })
}

fn calculate_autocomplete_score(line_chunks: &Vec<Result<Vec<char>, char>>) -> i64 {
    // Search for invalid lines and calculate the autocomplete score for each
    let mut line_completion_scores = line_chunks
        .iter()
        .cloned()
        .map(|line_result| line_result.unwrap_or(vec![]))
        .filter(|line_result| line_result.len() > 0)
        .map(calculate_line_autocomplete_score)
        .collect::<Vec<i64>>();

    // Sort the autocomplete line scores and return the middle score
    line_completion_scores.sort();
    line_completion_scores[line_completion_scores.len() / 2]
}

fn main() {
    let input_lines = parse_input_data(input_data());
    let line_chunks = input_lines
        .iter()
        .map(|line| check_line_chunks(line))
        .collect::<Vec<Result<Vec<char>, char>>>();

    // Solution for puzzle 1
    println!(
        "Syntax error score: {}",
        calculate_syntax_error_score(&line_chunks)
    );

    // Solution for puzzle 1
    println!(
        "Autocomplete score: {}",
        calculate_autocomplete_score(&line_chunks)
    );
}
