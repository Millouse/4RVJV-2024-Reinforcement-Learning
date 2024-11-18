use rand::Rng;
use crate::contracts::mdp_env::MDPEnv;
use crate::contracts::model_free_env::ModelFreeEnv;

pub struct MontyHall {
    current_door: usize,  // La porte choisie par le joueur
    prize_door: usize,    // La porte contenant le prix
    host_door: usize,     // La porte que l'animateur révèle
    game_over: bool,      // Indique si le jeu est terminé
}

impl MDPEnv for MontyHall {
    fn num_states() -> usize {
        3 * 3 * 2 // 3 portes, 3 portes d'animateur possibles, 2 états (jeu en cours/terminé)
    }

    fn num_actions() -> usize {
        2 // 0: Garder la porte, 1: Changer de porte
    }

    fn num_rewards() -> usize {
        2 // 0: Perdre, 1: Gagner
    }

    fn reward(index: usize) -> f32 {
        match index {
            0 => -1.0, // Récompense négative pour une perte
            1 => 1.0,  // Récompense positive pour une victoire
            _ => panic!("Invalid reward index"),
        }
    }

    fn transition_probability(state: usize, action: usize, next_state: usize, reward_index: usize) -> f32 {
        // Le Monty Hall Problem a des transitions déterministes basées sur les règles
        // On simplifie ici pour ne pas définir toutes les combinaisons possibles
        if reward_index == 1 && action == 1 {
            // Transition possible : changer de porte mène au prix
            1.0
        } else if reward_index == 0 && action == 0 {
            // Transition possible : garder la porte mène à une perte
            1.0
        } else {
            0.0
        }
    }
}

impl ModelFreeEnv for MontyHall {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let prize_door = rng.gen_range(0..3); // La porte contenant le prix
        let current_door = rng.gen_range(0..3); // La porte initialement choisie par le joueur
        let host_door = (0..3)
            .filter(|&d| d != prize_door && d != current_door)
            .next()
            .unwrap(); // L'animateur révèle une porte sans prix et non choisie
        MontyHall {
            current_door,
            prize_door,
            host_door,
            game_over: false,
        }
    }

    fn from_random_state(rng: &mut impl Rng) -> Self {
        let prize_door = rng.gen_range(0..3);
        let current_door = rng.gen_range(0..3);
        let host_door = (0..3)
            .filter(|&d| d != prize_door && d != current_door)
            .next()
            .unwrap();
        MontyHall {
            current_door,
            prize_door,
            host_door,
            game_over: false,
        }
    }

    fn num_states() -> usize {
        3 * 3 * 2 // Les combinaisons possibles d'états
    }

    fn num_actions() -> usize {
        2 // 0: Garder la porte, 1: Changer de porte
    }

    fn reset(&mut self) {
        *self = MontyHall::new();
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn score(&self) -> f32 {
        if self.game_over {
            if self.current_door == self.prize_door {
                1.0 // Victoire
            } else {
                -1.0 // Défaite
            }
        } else {
            0.0
        }
    }

    fn state_id(&self) -> usize {
        let game_over_flag = if self.game_over { 1 } else { 0 };
        self.current_door + self.prize_door * 3 + game_over_flag * 9
    }

    fn is_forbidden(&self, action: usize) -> bool {
        !(0..=1).contains(&action) || self.game_over
    }

    fn available_actions(&self) -> Vec<usize> {
        if self.game_over {
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
            0 => {
                // Garder la porte choisie
                self.game_over = true;
            }
            1 => {
                // Changer de porte
                self.current_door = (0..3)
                    .filter(|&d| d != self.current_door && d != self.host_door)
                    .next()
                    .unwrap();
                self.game_over = true;
            }
            _ => panic!("Invalid action, should not happen"),
        }
    }
}