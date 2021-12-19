fn input_data() -> Vec<&'static str> {
    vec![
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ]
}

#[derive(Debug, Clone, PartialEq)]
enum SnailfishNumberToken {
    PairStart,
    PairEnd,
    Number(u32),
}

fn parse_snailfish_numbers(input_string: &str) -> Vec<SnailfishNumberToken> {
    // Parse a Snailfish number string into a Vector of tokens
    input_string
        .chars()
        .filter(|c| *c != ',')
        .map(|c| match c {
            '[' => SnailfishNumberToken::PairStart,
            ']' => SnailfishNumberToken::PairEnd,
            _ => SnailfishNumberToken::Number(c.to_digit(10).unwrap()),
        })
        .collect()
}

fn add_snailfish_numbers(
    number_1: Vec<SnailfishNumberToken>,
    number_2: Vec<SnailfishNumberToken>,
) -> Vec<SnailfishNumberToken> {
    // Create a new snailfish number as the sum by setting the two terms as first and second pair element
    let mut snailfish_number_sum = Vec::new();
    snailfish_number_sum.push(SnailfishNumberToken::PairStart);
    snailfish_number_sum.extend(number_1.clone());
    snailfish_number_sum.extend(number_2.clone());
    snailfish_number_sum.push(SnailfishNumberToken::PairEnd);

    // Apply explosions and splits as long as the sum is not reduced sufficiently
    let mut is_valid_number = false;
    while !is_valid_number {
        is_valid_number = true;

        // Check for Snailfish number explodes
        let mut nesting_depth_counter = 0;
        let mut exploding_pair_indices: Option<(usize, usize)> = None;
        for (i, token) in snailfish_number_sum.iter().cloned().enumerate() {
            match token {
                SnailfishNumberToken::PairStart => {
                    nesting_depth_counter += 1;
                }
                SnailfishNumberToken::PairEnd => {
                    nesting_depth_counter -= 1;
                }
                SnailfishNumberToken::Number(_) if nesting_depth_counter > 4 => {
                    exploding_pair_indices = Some((i, i + 1));
                    break;
                }
                _ => (),
            }
        }
        if let Some((left_index, right_index)) = exploding_pair_indices {
            // Explosion necessary
            let mut snailfish_number_sum_after_explosion = Vec::new();
            snailfish_number_sum_after_explosion
                .extend(snailfish_number_sum[..(left_index - 1)].iter().cloned());

            // Add first exploding pair element to next number to the left
            if let SnailfishNumberToken::Number(explosion_left_number) =
                snailfish_number_sum[left_index]
            {
                for i in (0..=(left_index - 2)).rev() {
                    if let SnailfishNumberToken::Number(n) = snailfish_number_sum_after_explosion[i]
                    {
                        snailfish_number_sum_after_explosion[i] =
                            SnailfishNumberToken::Number(n + explosion_left_number);
                        break;
                    }
                }
            }

            // Add a 0 as a replacement for the exploded pair
            snailfish_number_sum_after_explosion.push(SnailfishNumberToken::Number(0));

            // Add second exploding pair element to next number to the right
            if let SnailfishNumberToken::Number(explosion_right_number) =
                snailfish_number_sum[right_index]
            {
                for i in (right_index + 1)..snailfish_number_sum.len() {
                    if let SnailfishNumberToken::Number(n) = snailfish_number_sum[i] {
                        snailfish_number_sum[i] =
                            SnailfishNumberToken::Number(n + explosion_right_number);
                        break;
                    }
                }
            }
            snailfish_number_sum_after_explosion
                .extend(snailfish_number_sum[(right_index + 2)..].iter().cloned());

            is_valid_number = false;
            snailfish_number_sum = snailfish_number_sum_after_explosion;
            continue;
        }

        // Check for Snailfish number splits
        let mut executed_split = false;
        snailfish_number_sum = snailfish_number_sum
            .iter()
            .cloned()
            .map(|token| match token {
                SnailfishNumberToken::Number(n) if n > 9 && !executed_split => {
                    is_valid_number = false;
                    executed_split = true;
                    let n = n as f32;
                    vec![
                        SnailfishNumberToken::PairStart,
                        SnailfishNumberToken::Number((n / 2.0).floor() as u32),
                        SnailfishNumberToken::Number((n / 2.0).ceil() as u32),
                        SnailfishNumberToken::PairEnd,
                    ]
                }
                _ => vec![token],
            })
            .flatten()
            .collect();
    }
    snailfish_number_sum
}

fn split_snailfish_number_pair(
    snailfish_number: Vec<SnailfishNumberToken>,
) -> (Vec<SnailfishNumberToken>, Vec<SnailfishNumberToken>) {
    // Split a (possibly nested) snailfish number into the two pair elements
    let mut depth_counter = 0;
    let mut pair_middle = 0;
    for (i, token) in snailfish_number.iter().enumerate() {
        match token {
            SnailfishNumberToken::PairStart => {
                depth_counter += 1;
            }
            SnailfishNumberToken::PairEnd => {
                depth_counter -= 1;
                if depth_counter == 0 {
                    pair_middle = i;
                    break;
                }
            }
            SnailfishNumberToken::Number(_) if depth_counter == 0 => {
                pair_middle = i;
                break;
            }
            _ => (),
        }
    }
    (
        snailfish_number[..=pair_middle].to_vec(),
        snailfish_number[(pair_middle + 1)..].to_vec(),
    )
}

fn calculate_magnitude(snailfish_number: Vec<SnailfishNumberToken>) -> u32 {
    // Calculate the magnitude of a snailfish number
    if snailfish_number.len() == 1 {
        if let SnailfishNumberToken::Number(n) = snailfish_number[0] {
            return n;
        } else {
            return 0;
        }
    }
    let snailfish_number = &snailfish_number.clone()[1..snailfish_number.len() - 1];

    let (first, second) = split_snailfish_number_pair(snailfish_number.to_vec());

    3 * calculate_magnitude(first) + 2 * calculate_magnitude(second)
}

fn main() {
    let snailfish_numbers = input_data()
        .iter()
        .cloned()
        .map(parse_snailfish_numbers)
        .collect::<Vec<Vec<SnailfishNumberToken>>>();

    // Solution for puzzle 1
    let snailfish_number_sum = snailfish_numbers
        .iter()
        .cloned()
        .reduce(add_snailfish_numbers)
        .unwrap();
    let sum_magnitude = calculate_magnitude(snailfish_number_sum);
    println!(
        "Magnitude of the sum of snailfish numbers: {}",
        sum_magnitude
    );

    // Solution for puzzle 2
    let mut max_magnitude = 0;
    for (i, n_1) in snailfish_numbers.iter().cloned().enumerate() {
        for (j, n_2) in snailfish_numbers.iter().cloned().enumerate() {
            if i != j {
                let snailfish_number_sum = add_snailfish_numbers(n_1.clone(), n_2);
                let sum_magnitude = calculate_magnitude(snailfish_number_sum);
                max_magnitude = u32::max(sum_magnitude, max_magnitude);
            }
        }
    }
    println!(
        "Maximal possible magnitude of the sum of two snailfish numbers: {}",
        max_magnitude
    );
}
