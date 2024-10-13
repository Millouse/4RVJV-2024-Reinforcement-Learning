use rand::Rng;
use crate::contracts::mdp_env::MDPEnv;
use crate::contracts::model_free_env::ModelFreeEnv;

pub struct GridWorld<const WIDTH: usize, const HEIGHT: usize> {
    current_pos: (usize, usize),
}
impl<const WIDTH: usize, const HEIGHT: usize> MDPEnv for GridWorld<WIDTH, HEIGHT> {
    fn num_states() -> usize {
        WIDTH * HEIGHT
    }

    fn num_actions() -> usize {
        4
    }

    fn num_rewards() -> usize {
        3
    }

    fn reward(index: usize) -> f32 {
        match index {
            0 => -3.0,
            1 => 0.0,
            2 => 1.0,
            _ => panic!("Invalid reward index"),
        }
    }

    fn transition_probability(state: usize, action: usize, next_state: usize, reward_index: usize) -> f32 {
        let (x, y) = (state % WIDTH, state / WIDTH);
        let (next_x, next_y) = (next_state % WIDTH, next_state / WIDTH);
        let max_x = WIDTH - 1;
        let max_y = HEIGHT - 1;

        match (x, y, action, next_x, next_y, reward_index) {
            (0, _, 2, _, _, _) => 0.0, // Impossible d'aller à gauche en étant sur le bord gauche
            (_, 0, 0, _, _, _) => 0.0, // Impossible de monter en étant sur le bord supérieur
            (max_x, _, 3, _, _, _) => 0.0, // Impossible d'aller à droite en étant sur le bord droit
            (_, max_y, 1, _, _, _) => 0.0, // Impossible de descendre en étant sur le bord inférieur
            (max_x, max_y, 1, _, _, 2) => 1.0, // Récompense positive à la fin
            (0, 0, 0, _, _, 0) => 1.0, // Récompense négative au début
            _ => 0.0,
        }
    }

}

impl<const WIDTH: usize, const HEIGHT: usize> ModelFreeEnv for GridWorld<WIDTH, HEIGHT> {
fn new() -> Self {
GridWorld {
current_pos: (WIDTH / 2, HEIGHT / 2),
}
}

    fn from_random_state(rng: &mut impl Rng) -> Self {
        GridWorld {
            current_pos: (rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT)),
        }
    }

    fn num_states() -> usize {
        WIDTH * HEIGHT
    }

    fn num_actions() -> usize {
        4
    }

    fn reset(&mut self) {
        *self = GridWorld::new();
    }

    fn is_game_over(&self) -> bool {
        self.current_pos == (0, 0) || self.current_pos == (WIDTH - 1, HEIGHT - 1)
    }

    fn score(&self) -> f32 {
        if self.current_pos == (0, 0) {
            -1.0
        } else if self.current_pos == (WIDTH - 1, HEIGHT - 1) {
            1.0
        } else {
            0.0
        }
    }

    fn state_id(&self) -> usize {
        let (x, y) = self.current_pos;
        y * WIDTH + x
    }

    fn is_forbidden(&self, action: usize) -> bool {
        self.is_game_over() && !(0..=3).contains(&action)
    }

    fn available_actions(&self) -> Vec<usize> {
        if self.is_game_over() {
            vec![]
        } else {
            vec![0, 1, 2, 3]
        }
    }

    fn step(&mut self, action: usize) {
        if self.is_forbidden(action) {
            eprintln!("Forbidden action");
            std::process::exit(42);
        }

        let (x, y) = self.current_pos;
        match action {
            0 if y > 0 => self.current_pos = (x, y - 1),     // Haut
            1 if y < HEIGHT - 1 => self.current_pos = (x, y + 1), // Bas
            2 if x > 0 => self.current_pos = (x - 1, y),     // Gauche
            3 if x < WIDTH - 1 => self.current_pos = (x + 1, y), // Droite
            _ => panic!("Invalid action, should not happen"),
        }
    }
}