use std::time::{SystemTime, UNIX_EPOCH};
use crate::contracts::model_free_env::ModelFreeEnv;
use rand::prelude::SliceRandom;

pub fn monte_carlo_exploring_starts<TEnv: ModelFreeEnv>(
    num_episodes: usize,
    gamma: f32,
) -> Vec<Vec<f32>> {
    let mut q_values = vec![vec![0.0; TEnv::num_actions()]; TEnv::num_states()];
    let mut returns = vec![vec![]; TEnv::num_states()]; // Historique des retours pour chaque état-action
    let mut env = TEnv::new();
    let mut rng = rand::thread_rng();

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64;

    for _ in 0..num_episodes {
        env.reset();

        // Initialisation des épisodes avec des états et actions aléatoires
        let s = env.state_id();
        let available_actions = env.available_actions();
        let a = *available_actions.choose(&mut rng).unwrap(); // Choisir une action aléatoire pour "exploring starts"

        let mut episode = vec![(s, a)]; // Episode sous forme (état, action)

        while !env.is_game_over() {
            let s = env.state_id();
            let available_actions = env.available_actions();
            let a = *available_actions.choose(&mut rng).unwrap(); // Choisir une action aléatoire
            env.step(a);
            episode.push((s, a));
        }

        // Calculer la somme des récompenses futures pour cet épisode
        let mut G = 0.0;
        let mut visited = std::collections::HashSet::new();
        for &(state, action) in episode.iter().rev() {
            if visited.contains(&(state, action)) {
                continue;
            }
            visited.insert((state, action));

            // Récompense immédiate : ici on suppose que la fonction score() donne la récompense de l'étape
            let reward = env.score();

            G = gamma * G + reward;

            // Mettre à jour les valeurs de q
            let q = &mut q_values[state][action];
            returns[state].push(G);
            *q = returns[state].iter().sum::<f32>() / returns[state].len() as f32;
        }
    }

    println!("time : {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64 - start_time);
    q_values
}
