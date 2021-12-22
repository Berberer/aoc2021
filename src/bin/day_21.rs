use std::collections::HashMap;
use std::hash::Hash;

fn input_data() -> Vec<&'static str> {
    vec![
        "Player 1 starting position: 4",
        "Player 2 starting position: 8",
    ]
}

fn parse_input_data(input_data: Vec<&str>) -> (u64, u64) {
    (
        input_data[0].chars().last().unwrap().to_digit(10).unwrap() as u64,
        input_data[1].chars().last().unwrap().to_digit(10).unwrap() as u64,
    )
}

#[derive(Debug, Clone)]
struct DeterministicDice {
    next_roll_result: u64,
    number_of_rolls: u64,
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice {
            next_roll_result: 1,
            number_of_rolls: 0,
        }
    }

    fn roll_dice(&self) -> (u64, DeterministicDice) {
        // Get sum of the three next deterministic dice rolls and update counters
        (
            (self.next_roll_result..(self.next_roll_result + 3)).sum(),
            DeterministicDice {
                next_roll_result: self.next_roll_result + 3,
                number_of_rolls: self.number_of_rolls + 3,
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Player {
    points: u64,
    position: u64,
    has_active_turn: bool,
}

impl Player {
    fn new(position: u64, has_active_turn: bool) -> Player {
        Player {
            points: 0,
            position,
            has_active_turn,
        }
    }

    fn move_on_board(&self, rolls_sum: u64) -> Player {
        // Update position and points with dice roll if it is the players turn
        let next_position = if self.has_active_turn {
            (self.position + rolls_sum - 1) % 10 + 1
        } else {
            self.position
        };
        let updated_points = if self.has_active_turn {
            self.points + next_position
        } else {
            self.points
        };
        Player {
            points: updated_points,
            position: next_position,
            has_active_turn: !self.has_active_turn,
        }
    }
}

fn play_game_with_deterministic_dice(
    (starting_position_1, starting_position_2): &(u64, u64),
) -> (Player, Player, DeterministicDice) {
    // Play a single game until 1000 points with a deterministic dice
    let mut player_1 = Player::new(*starting_position_1, true);
    let mut player_2 = Player::new(*starting_position_2, false);
    let mut dice = DeterministicDice::new();

    while u64::max(player_1.points, player_2.points) < 1000 {
        let dice_roll = dice.roll_dice();
        dice = dice_roll.1;
        player_1 = player_1.move_on_board(dice_roll.0);
        player_2 = player_2.move_on_board(dice_roll.0);
    }

    return (player_1, player_2, dice);
}

fn play_game_with_dirac_dice(
    (starting_position_1, starting_position_2): &(u64, u64),
) -> (u64, u64) {
    // Simulate all universes of the Dirac dice and count the total wins of each player
    let player_1 = Player::new(*starting_position_1, true);
    let player_2 = Player::new(*starting_position_2, false);

    // Recursive solution to traverse the universe tree and create a memoization cache for already calculated sub trees
    fn simulate_from_player_positions(
        p_1: Player,
        p_2: Player,
        cache: HashMap<(Player, Player), (u64, u64)>,
    ) -> (u64, u64, HashMap<(Player, Player), (u64, u64)>) {
        if let Some(wins) = cache.get(&(p_1.clone(), p_2.clone())) {
            (wins.0, wins.1, cache)
        } else if p_1.points >= 21 {
            (1, 0, HashMap::new())
        } else if p_2.points >= 21 {
            (0, 1, HashMap::new())
        } else {
            let mut player_1_wins = 0;
            let mut player_2_wins = 0;
            let mut updated_cache = cache;
            for dirac_dice_roll_1 in 1..=3 {
                for dirac_dice_roll_2 in 1..=3 {
                    for dirac_dice_roll_3 in 1..=3 {
                        let dirac_dice_roll =
                            dirac_dice_roll_1 + dirac_dice_roll_2 + dirac_dice_roll_3;
                        let p_1_updated = p_1.move_on_board(dirac_dice_roll);
                        let p_2_updated = p_2.move_on_board(dirac_dice_roll);
                        let (p_1_w, p_2_w, sub_cache) = simulate_from_player_positions(
                            p_1_updated.clone(),
                            p_2_updated.clone(),
                            updated_cache.clone(),
                        );
                        player_1_wins += p_1_w;
                        player_2_wins += p_2_w;
                        updated_cache.extend(sub_cache);
                        updated_cache.insert((p_1_updated, p_2_updated), (p_1_w, p_2_w));
                    }
                }
            }
            updated_cache.insert((p_1.clone(), p_2.clone()), (player_1_wins, player_2_wins));
            (player_1_wins, player_2_wins, updated_cache)
        }
    }

    let (player_1_wins, player_2_wins, _) =
        simulate_from_player_positions(player_1, player_2, HashMap::new());

    (player_1_wins, player_2_wins)
}

fn main() {
    let starting_positions = parse_input_data(input_data());

    // Solution for puzzle 1
    let (player_1, player_2, dice) = play_game_with_deterministic_dice(&starting_positions);
    let loosing_score = u64::min(player_1.points, player_2.points);
    println!(
        "Points of the loosing player {} * Number of deterministic dice rolls {} = {}",
        loosing_score,
        dice.number_of_rolls,
        loosing_score * dice.number_of_rolls
    );

    // Solution for puzzle 2
    let (player_1_wins, player_2_wins) = play_game_with_dirac_dice(&starting_positions);
    println!(
        "With a Dirac dice, Player 1 wins {} games and Player 2 {} games. Higher number of won games: {}",
        player_1_wins,
        player_2_wins,
        u64::max(player_1_wins, player_2_wins)
    );
}
