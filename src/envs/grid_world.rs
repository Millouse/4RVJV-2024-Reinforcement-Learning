use rand::Rng;
use crate::contracts::mdp_env::MDPEnv;
use crate::contracts::model_free_env::ModelFreeEnv;

pub struct GridWorld<const ROWS: usize, const COLUMNS: usize> {
    current_row: usize,
    current_column: usize,
}

impl<const ROWS: usize, const COLUMNS: usize> MDPEnv for GridWorld<ROWS, COLUMNS> {
    fn num_states() -> usize {
        ROWS * COLUMNS
    }

    fn num_actions() -> usize {
        4 // up, down, left, right
    }

    fn num_rewards() -> usize {
        3 // -3, 0, +1
    }

    fn reward(index: usize) -> f32 {
        match index {
            0 => -3.0, // Coin supérieur droit
            1 => 0.0,  // Autres cases
            2 => 1.0,  // Coin inférieur gauche
            _ => panic!("Invalid reward index"),
        }
    }

    fn transition_probability(state: usize, action: usize, next_state: usize, reward_index: usize) -> f32 {
        let (row, col) = (state / COLUMNS, state % COLUMNS);
        let (next_row, next_col) = (next_state / COLUMNS, next_state % COLUMNS);

        // Vérifie les états de début et de fin
        if state == 0 || state == (ROWS - 1) * COLUMNS + (COLUMNS - 1) {
            return 0.0; // Pas de transition depuis l'état 0 ou l'état final
        }

        // Conditions de transition en fonction de l'action
        match action {
            0 => { // Action "up"
                if row > 0 && next_row == row - 1 && next_col == col {
                    if reward_index == 0 {
                        return 0.5; // Moins probable si c'est une récompense négative
                    } else if reward_index == 1 {
                        return 0.7; // Moyennement probable pour une case neutre
                    } else if reward_index == 2 {
                        return 0.9; // Plus probable si c'est une récompense positive
                    }
                }
            },
            1 => { // Action "down"
                if row < (ROWS - 1) && next_row == row + 1 && next_col == col {
                    if reward_index == 0 {
                        return 0.6;
                    } else if reward_index == 1 {
                        return 0.8;
                    } else if reward_index == 2 {
                        return 0.95;
                    }
                }
            },
            2 => { // Action "left"
                if col > 0 && next_row == row && next_col == col - 1 {
                    if reward_index == 0 {
                        return 0.4;
                    } else if reward_index == 1 {
                        return 0.75;
                    } else if reward_index == 2 {
                        return 0.85;
                    }
                }
            },
            3 => { // Action "right"
                if col < (COLUMNS - 1) && next_row == row && next_col == col + 1 {
                    if reward_index == 0 {
                        return 0.3;
                    } else if reward_index == 1 {
                        return 0.7;
                    } else if reward_index == 2 {
                        return 0.9;
                    }
                }
            },
            _ => return 0.0, // Action non reconnue
        }

        0.0 // Retourne 0 pour toutes les autres transitions non valides
    }
}

impl<const ROWS: usize, const COLUMNS: usize> ModelFreeEnv for GridWorld<ROWS, COLUMNS> {
    fn new() -> Self {
        GridWorld {
            current_row: 0,
            current_column: 0,
        }
    }

    fn from_random_state(rng: &mut impl Rng) -> Self {
        GridWorld {
            current_row: rng.gen_range(0..ROWS),
            current_column: rng.gen_range(0..COLUMNS),
        }
    }

    fn num_states() -> usize {
        ROWS * COLUMNS
    }

    fn num_actions() -> usize {
        4 // up, down, left, right
    }

    fn reset(&mut self) {
        *self = GridWorld::new();
    }

    fn is_game_over(&self) -> bool {
        // La partie est terminée si on est dans l'état (0, 3) ou (3, 0)
        (self.current_row == 0 && self.current_column == COLUMNS - 1) ||
            (self.current_row == ROWS - 1 && self.current_column == 0)
    }

    fn score(&self) -> f32 {
        if self.current_row == 0 && self.current_column == COLUMNS - 1 {
            GridWorld::<ROWS, COLUMNS>::reward(0) // Coin supérieur droit
        } else if self.current_row == ROWS - 1 && self.current_column == 0 {
            GridWorld::<ROWS, COLUMNS>::reward(2) // Coin inférieur gauche
        } else {
            GridWorld::<ROWS, COLUMNS>::reward(1) // Autres cases
        }
    }

    fn state_id(&self) -> usize {
        self.current_row * COLUMNS + self.current_column
    }

    fn is_forbidden(&self, action: usize) -> bool {
        // Vérifier si l'action est interdite
        match action {
            0 => self.current_row == 0, // up
            1 => self.current_row == ROWS - 1, // down
            2 => self.current_column == 0, // left
            3 => self.current_column == COLUMNS - 1, // right
            _ => false,
        }
    }

    fn available_actions(&self) -> Vec<usize> {
        let mut actions = vec![];
        if self.current_row > 0 {
            actions.push(0); // up
        }
        if self.current_row < ROWS - 1 {
            actions.push(1); // down
        }
        if self.current_column > 0 {
            actions.push(2); // left
        }
        if self.current_column < COLUMNS - 1 {
            actions.push(3); // right
        }
        actions
    }

    fn step(&mut self, action: usize) {
        if self.is_forbidden(action) {
            eprintln!("Forbidden action: {:?}", action);  // Affiche l'action interdite
            eprintln!("Current position: ({}, {})", self.current_row, self.current_column);
            std::process::exit(42);
        }

        match action {
            0 => self.current_row = self.current_row.wrapping_sub(1), // up
            1 => self.current_row += 1, // down
            2 => self.current_column = self.current_column.wrapping_sub(1), // left
            3 => self.current_column += 1, // right
            _ => panic!("Invalid action, should not happen"),
        }
    }
}
