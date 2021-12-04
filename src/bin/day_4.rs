fn input_data() -> Vec<&'static str> {
    // Replace these bingo numbers with different combinations
    vec![
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    ]
}

struct BingoCard {
    numbers: [[i32; 5]; 5],
    hits_per_row: [i32; 5],
    hits_per_column: [i32; 5],
    bingo: bool,
}

impl BingoCard {
    fn new(number_rows: &[&str]) -> BingoCard {
        let mut numbers = [[0; 5]; 5];
        for (i, row) in number_rows[1..=5].iter().cloned().enumerate() {
            for (j, number) in row.split(' ').filter(|s| !s.is_empty()).enumerate() {
                numbers[i][j] = number.parse::<i32>().unwrap();
            }
        }
        BingoCard {
            numbers,
            hits_per_row: [0; 5],
            hits_per_column: [0; 5],
            bingo: false,
        }
    }

    fn sum_fields(&self) -> i32 {
        let mut sum = 0;
        for row in 0..5 {
            for column in 0..5 {
                if self.numbers[row][column] != -1 {
                    sum = sum + self.numbers[row][column];
                }
            }
        }
        sum
    }

    fn check_bingo(&mut self, number: i32) -> Option<i32> {
        if !self.bingo {
            let mut hit = (9, 9);
            // Search for the drawn number in the card
            for row in 0..5 {
                for column in 0..5 {
                    if self.numbers[row][column] == number {
                        hit = (row, column);
                        self.numbers[row][column] = -1;
                    }
                }
            }
            // If the card contains the drawn number, check if it is a bingo
            let (hit_row, hit_column) = hit;
            if hit_row != 9 && hit_column != 9 {
                self.hits_per_row[hit_row] = self.hits_per_row[hit_row] + 1;
                self.hits_per_column[hit_column] = self.hits_per_column[hit_column] + 1;
                if self.hits_per_row[hit_row] == 5 || self.hits_per_column[hit_column] == 5 {
                    self.bingo = true;
                    return Some(self.sum_fields() * number);
                }
            }
        }
        None
    }
}

fn parse_bingo_cards(input: Vec<&str>) -> (Vec<i32>, Vec<BingoCard>) {
    // Split input lines into sequence of drawn numbers and the bingo cards
    let drawn_numbers = input[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let cards = input[1..input.len()]
        .chunks(6)
        .map(|card| BingoCard::new(card))
        .collect();

    (drawn_numbers, cards)
}

fn main() {
    let (drawn_numbers, mut bingo_cards) = parse_bingo_cards(input_data());
    let mut winning_score = None;
    let mut last_score = None;
    for number in drawn_numbers {
        for card in bingo_cards.iter_mut() {
            let card_score = card.check_bingo(number);
            if card_score != None {
                if winning_score == None {
                    winning_score = card_score;
                }
                last_score = card_score;
            }
        }
    }
    match (winning_score, last_score) {
        (Some(winning_card_score), Some(last_card_score)) => println!(
            "Score of the winning card is: {} - Score of the last winning card is: {}",
            winning_card_score, last_card_score
        ),
        _ => println!("No card has won"),
    }
}
