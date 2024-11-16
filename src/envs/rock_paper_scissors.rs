use rand::Rng;
use crate::contracts::model_free_env::ModelFreeEnv;

pub struct RockPaperScissors {
    round: usize,
    next_move: usize,
    score: isize,
}

impl ModelFreeEnv for RockPaperScissors {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        RockPaperScissors {
            round: 0,
            next_move: rng.gen_range(0..3),
            score: 0,
        }
    }

    fn from_random_state(rng: &mut impl Rng) -> Self {
        RockPaperScissors {
            round: 0,
            next_move: rng.gen_range(0..3),
            score: 0,
        }
    }

    fn num_states() -> usize {
        2
    }

    fn num_actions() -> usize {
        3
    }

    fn reset(&mut self) {
        *self = RockPaperScissors::new();
    }

    fn is_game_over(&self) -> bool {
        self.round >= 1
    }

    fn score(&self) -> f32 {
        self.score as f32
    }

    fn state_id(&self) -> usize {
        self.round
    }

    fn is_forbidden(&self, action: usize) -> bool {
        self.is_game_over() && !(0..=2).contains(&action)
    }

    fn available_actions(&self) -> Vec<usize> {
        if self.is_game_over() {
            vec![]
        } else {
            vec![0, 1, 2]
        }
    }

    fn step(&mut self, action: usize) {
        if self.is_forbidden(action) {
            eprintln!("Forbidden action");
            std::process::exit(42);
        }

        self.score += beats_me(action, self.next_move);

        self.next_move = action;

        self.round += 1;

    }
}

fn beats_me(action: usize, my_move: usize) -> isize{
    match action {
        0 => {
            match my_move {
                0 => {
                    0
                }
                1 => {
                    -1
                }
                2 => {
                    1
                }
                _ => panic!("Invalid action by environment, should definitely not happen WTF ???"),
            }
        }
        1 => {
            match my_move {
                0 => {
                    1
                }
                1 => {
                    0
                }
                2 => {
                    -1
                }
                _ => panic!("Invalid action by environment, should definitely not happen WTF ???"),
            }
        }
        2 => {
            match my_move {
                0 => {
                    -1
                }
                1 => {
                    1
                }
                2 => {
                    0
                }
                _ => panic!("Invalid action by environment, should definitely not happen WTF ???"),
            }
        }
        _ => panic!("Invalid action, should not happen"),
    }
}