use rand::Rng;
use crate::contracts::mdp_env::MDPEnv;
use crate::contracts::model_free_env::ModelFreeEnv;

pub struct LineWorld<const NB_CELLS: usize> {
    current_cell: usize,
}

impl<const NB_CELLS: usize> MDPEnv for LineWorld<NB_CELLS> {
    fn num_states() -> usize {
        NB_CELLS
    }

    fn num_actions() -> usize {
        2
    }

    fn num_rewards() -> usize {
        3
    }

    fn reward(index: usize) -> f32 {
        match index {
            0 => -1.0,
            1 => 0.0,
            2 => 1.0,
            _ => panic!("Invalid reward index"),
        }
    }

    fn transition_probability(state: usize, action: usize, next_state: usize, reward_index: usize) -> f32 {
        match (state, action, next_state, reward_index) {
            (0, _, _, _) => 0.0,
            (s, _, _, _) if s == NB_CELLS - 1 => 0.0,
            (s, 1, s_p, 1) if (1..(NB_CELLS - 2)).contains(&s) && s + 1 == s_p => 1.0,
            (s, 0, s_p, 1) if (2..(NB_CELLS - 1)).contains(&s) && s == s_p + 1 => 1.0,
            (1, 0, 0, 0) => 1.0,
            (s, 1, s_p, 2) if s == NB_CELLS - 2 && s + 1 == s_p => 1.0,
            _ => 0.0,
        }
    }
}

impl<const NB_CELLS: usize> ModelFreeEnv for LineWorld<NB_CELLS> {
    fn new() -> Self {
        LineWorld {
            current_cell: NB_CELLS / 2,
        }
    }

    fn from_random_state(rng: &mut impl Rng) -> Self {
        LineWorld {
            current_cell: rng.gen_range(0..NB_CELLS)
        }
    }

    fn num_states() -> usize {
        NB_CELLS
    }

    fn num_actions() -> usize {
        2
    }

    fn reset(&mut self) {
        *self = LineWorld::new();
    }

    fn is_game_over(&self) -> bool {
        self.current_cell == 0 || self.current_cell == NB_CELLS - 1
    }

    fn score(&self) -> f32 {
        if self.current_cell == 0 {
            -1.0
        } else if self.current_cell == NB_CELLS - 1 {
            1.0
        } else {
            0.0
        }
    }

    fn state_id(&self) -> usize {
        self.current_cell
    }

    fn is_forbidden(&self, action: usize) -> bool {
        self.is_game_over() && !(0..=1).contains(&action)
    }

    fn available_actions(&self) -> Vec<usize> {
        if self.is_game_over() {
            vec![]
        } else {
            vec![0, 1]
        }
    }

    fn step(&mut self, action: usize) {
        if self.is_forbidden(action) {
            eprintln!("Forbidden action");
            std::process::exit(42);
        }

        match action {
            0 => self.current_cell -= 1,
            1 => self.current_cell += 1,
            _ => panic!("Invalid action, should not happen"),
        }
    }
}