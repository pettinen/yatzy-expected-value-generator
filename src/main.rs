use std::{
    collections::{HashMap, HashSet},
    io::Write as _,
};

use itertools::Itertools as _;
use rayon::iter::{IndexedParallelIterator as _, IntoParallelIterator as _, ParallelIterator as _};
use serde::{Deserialize, Serialize};
use yatzy::{Combo, Dice, Die, Game};

type ExpectedValue = f64;

include!("prob_float.rs");

const COMBOS: [Combo; 15] = [
    Combo::Ones,
    Combo::Twos,
    Combo::Threes,
    Combo::Fours,
    Combo::Fives,
    Combo::Sixes,
    Combo::OnePair,
    Combo::TwoPairs,
    Combo::ThreeOfAKind,
    Combo::FourOfAKind,
    Combo::SmallStraight,
    Combo::LargeStraight,
    Combo::FullHouse,
    Combo::Chance,
    Combo::Yatzy,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum NumberState {
    Empty,
    Filled0,
    Filled1,
    Filled2,
    Filled3,
    Filled4,
    Filled5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum FieldState {
    Empty,
    Filled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct GameState {
    numbers_total: u8,
    ones: FieldState,
    twos: FieldState,
    threes: FieldState,
    fours: FieldState,
    fives: FieldState,
    sixes: FieldState,
    one_pair: FieldState,
    two_pairs: FieldState,
    three_of_a_kind: FieldState,
    four_of_a_kind: FieldState,
    small_straight: FieldState,
    large_straight: FieldState,
    full_house: FieldState,
    chance: FieldState,
    yatzy: FieldState,
}

fn game_from_state(state: GameState, dice: Dice) -> Game {
    let mut numbers_filled = false;
    let ones = match state.ones {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let twos = match state.twos {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let threes = match state.threes {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let fours = match state.fours {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let fives = match state.fives {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                numbers_filled = true;
                Some(state.numbers_total)
            }
        }
    };
    let sixes = match state.sixes {
        FieldState::Empty => None,
        FieldState::Filled => {
            if numbers_filled {
                Some(0)
            } else {
                Some(state.numbers_total)
            }
        }
    };

    Game::new_raw(
        dice,
        2,
        ones,
        twos,
        threes,
        fours,
        fives,
        sixes,
        match state.one_pair {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.two_pairs {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.three_of_a_kind {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.four_of_a_kind {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.small_straight {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.large_straight {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.full_house {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.chance {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
        match state.yatzy {
            FieldState::Empty => None,
            FieldState::Filled => Some(0),
        },
    )
}

fn state_from_game(game: Game) -> GameState {
    GameState {
        numbers_total: game.combo(Combo::Ones).unwrap_or(0)
            + game.combo(Combo::Twos).unwrap_or(0)
            + game.combo(Combo::Threes).unwrap_or(0)
            + game.combo(Combo::Fours).unwrap_or(0)
            + game.combo(Combo::Fives).unwrap_or(0)
            + game.combo(Combo::Sixes).unwrap_or(0),
        ones: match game.combo(Combo::Ones) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        twos: match game.combo(Combo::Twos) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        threes: match game.combo(Combo::Threes) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        fours: match game.combo(Combo::Fours) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        fives: match game.combo(Combo::Fives) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        sixes: match game.combo(Combo::Sixes) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        one_pair: match game.combo(Combo::OnePair) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        two_pairs: match game.combo(Combo::TwoPairs) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        three_of_a_kind: match game.combo(Combo::ThreeOfAKind) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        four_of_a_kind: match game.combo(Combo::FourOfAKind) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        small_straight: match game.combo(Combo::SmallStraight) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        large_straight: match game.combo(Combo::LargeStraight) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        full_house: match game.combo(Combo::FullHouse) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        chance: match game.combo(Combo::Chance) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
        yatzy: match game.combo(Combo::Yatzy) {
            None => FieldState::Empty,
            Some(_) => FieldState::Filled,
        },
    }
}

fn game_states_by_empty_field_count() -> HashMap<u8, HashSet<GameState>> {
    let number_states = [
        NumberState::Empty,
        NumberState::Filled0,
        NumberState::Filled1,
        NumberState::Filled2,
        NumberState::Filled3,
        NumberState::Filled4,
        NumberState::Filled5,
    ];
    let field_states = [FieldState::Empty, FieldState::Filled];

    let mut map = HashMap::with_capacity(15);
    for n in 1..=14 {
        map.insert(n, HashSet::new());
    }

    for (n1, n2, n3, n4, n5, n6) in itertools::iproduct!(
        number_states,
        number_states,
        number_states,
        number_states,
        number_states,
        number_states,
    ) {
        for (f1, f2, f3, f4, f5, f6, f7, f8, f9) in itertools::iproduct!(
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
            field_states,
        ) {
            let mut empty = 0;
            let mut numbers_total = 0;

            for (n, state) in [(1, n1), (2, n2), (3, n3), (4, n4), (5, n5), (6, n6)] {
                match state {
                    NumberState::Empty => {
                        empty += 1;
                    }
                    NumberState::Filled0 => {}
                    NumberState::Filled1 => {
                        numbers_total += n;
                    }
                    NumberState::Filled2 => {
                        numbers_total += 2 * n;
                    }
                    NumberState::Filled3 => {
                        numbers_total += 3 * n;
                    }
                    NumberState::Filled4 => {
                        numbers_total += 4 * n;
                    }
                    NumberState::Filled5 => {
                        numbers_total += 5 * n;
                    }
                }
            }

            for state in [f1, f2, f3, f4, f5, f6, f7, f8, f9] {
                if state == FieldState::Empty {
                    empty += 1;
                }
            }

            if empty == 0 || empty == 15 {
                continue;
            }

            let state = GameState {
                numbers_total,
                ones: match n1 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                twos: match n2 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                threes: match n3 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                fours: match n4 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                fives: match n5 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                sixes: match n6 {
                    NumberState::Empty => FieldState::Empty,
                    _ => FieldState::Filled,
                },
                one_pair: f1,
                two_pairs: f2,
                three_of_a_kind: f3,
                four_of_a_kind: f4,
                small_straight: f5,
                large_straight: f6,
                full_house: f7,
                chance: f8,
                yatzy: f9,
            };

            map.get_mut(&empty).unwrap().insert(state);
        }
    }

    map
}

fn expected_value_0_rerolls(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue>,
) -> ExpectedValue {
    let mut max_expected_value = 0_f64;

    for combo in COMBOS {
        if game.combo(combo).is_some() {
            continue;
        }
        let mut game = game.clone();
        game.set_combo(combo);
        let value = if game.ended() {
            game.score().into()
        } else {
            let state = state_from_game(game);
            *expected_values.get(&state).unwrap()
        };
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}

fn expected_value_1_reroll(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue>,
) -> ExpectedValue {
    let mut max_expected_value = expected_value_0_rerolls(game, expected_values);

    for (d1,) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_1_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1], &replacement_dice).unwrap();
                prob * expected_value_0_rerolls(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    for (d1, d2) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_2_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1, d2], &replacement_dice).unwrap();
                prob * expected_value_0_rerolls(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    for (d1, d2, d3) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_3_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1, d2, d3], &replacement_dice).unwrap();
                prob * expected_value_0_rerolls(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    for (d1, d2, d3, d4) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_4_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1, d2, d3, d4], &replacement_dice)
                    .unwrap();
                prob * expected_value_0_rerolls(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    for (d1, d2, d3, d4, d5) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_5_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1, d2, d3, d4, d5], &replacement_dice)
                    .unwrap();
                prob * expected_value_0_rerolls(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}

fn expected_value_2_rerolls(
    game: Game,
    expected_values: &HashMap<GameState, ExpectedValue>,
) -> ExpectedValue {
    let mut max_expected_value = expected_value_0_rerolls(game, expected_values);

    for (d1,) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_1_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1], &replacement_dice).unwrap();
                prob * expected_value_1_reroll(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    for (d1, d2) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_2_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1, d2], &replacement_dice).unwrap();
                prob * expected_value_1_reroll(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    for (d1, d2, d3) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_3_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1, d2, d3], &replacement_dice).unwrap();
                prob * expected_value_1_reroll(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    for (d1, d2, d3, d4) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_4_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1, d2, d3, d4], &replacement_dice)
                    .unwrap();
                prob * expected_value_1_reroll(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    for (d1, d2, d3, d4, d5) in game.dice().into_iter().tuple_combinations() {
        let value = ROLL_5_PROB
            .into_par_iter()
            .map(|(replacement_dice, prob)| {
                let mut game = game.clone();
                game.replace_dice(&[d1, d2, d3, d4, d5], &replacement_dice)
                    .unwrap();
                prob * expected_value_1_reroll(game, expected_values)
            })
            .sum();
        if value > max_expected_value {
            max_expected_value = value;
        }
    }

    max_expected_value
}

fn fill_expected_values(
    states: &HashSet<GameState>,
    expected_values: &HashMap<GameState, ExpectedValue>,
) -> HashMap<GameState, ExpectedValue> {
    states
        .into_par_iter()
        .map(|&state| {
            let value: ExpectedValue = ROLL_5_PROB
                .into_par_iter()
                .take(1) // DEBUG only
                .map(|(dice_array, prob)| {
                    let dice = Dice::new_raw(dice_array);
                    let game = game_from_state(state, dice);
                    prob * expected_value_2_rerolls(game, expected_values)
                })
                .sum();

            (state, value)
        })
        .collect()
}

fn main() {
    let states = game_states_by_empty_field_count();
    let mut expected_values = HashMap::with_capacity(60_189_631);

    for n in 1..=14 {
        eprintln!(
            "calculating expected values for game states with {} empty field(s) ({} states)",
            n,
            states.get(&n).unwrap().len(),
        );
        expected_values.extend(fill_expected_values(states.get(&n).unwrap(), &expected_values));
    }

    let bytes = postcard::to_allocvec(&expected_values).unwrap();
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&bytes).unwrap();
}
