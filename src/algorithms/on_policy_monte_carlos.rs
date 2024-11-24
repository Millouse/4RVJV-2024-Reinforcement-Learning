use std::time::{SystemTime, UNIX_EPOCH};
use crate::contracts::model_free_env::ModelFreeEnv;
use rand::prelude::{ThreadRng, Rng, SliceRandom};
use std::collections::HashSet;

pub fn on_policy_monte_carlos<TEnv: ModelFreeEnv>(
    num_episodes: usize,
    gamma: f32,
    epsilon: f32, // epsilon pour epsilon-greedy
) -> (Vec<Vec<f32>>, Vec<usize>) {  // Retourne à la fois les valeurs Q et la politique pi
    let mut q_values = vec![vec![0.0; TEnv::num_actions()]; TEnv::num_states()];
    let mut returns = vec![vec![]; TEnv::num_states()]; // Historique des retours pour chaque état-action
    let mut pi = vec![0; TEnv::num_states()];  // Politique pour chaque état
    let mut env = TEnv::new();
    let mut rng = rand::thread_rng();

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64;

    for _ in 0..num_episodes {
        env.reset();

        let s = env.state_id();
        let a = epsilon_greedy_action(&q_values, s, epsilon, &mut rng);

        let mut episode = vec![(s, a)];

        while !env.is_game_over() {
            let s = env.state_id();
            let a = epsilon_greedy_action(&q_values, s, epsilon, &mut rng);
            env.step(a);
            episode.push((s, a));
        }

        let mut G = 0.0;
        let mut visited = HashSet::new();
        for &(state, action) in episode.iter().rev() {
            if visited.contains(&(state, action)) {
                continue;
            }
            visited.insert((state, action));

            let reward = env.score();

            G = gamma * G + reward;

            let q = &mut q_values[state][action];
            returns[state].push(G);
            *q = returns[state].iter().sum::<f32>() / returns[state].len() as f32;
        }
    }

    for state in 0..TEnv::num_states() {
        let best_action = (0..TEnv::num_actions())
            .max_by(|&a, &b| q_values[state][a].partial_cmp(&q_values[state][b]).unwrap())
            .unwrap();
        pi[state] = best_action;
    }

    println!("time : {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64 - start_time);

    (q_values, pi)
}

fn epsilon_greedy_action(
    q_values: &[Vec<f32>],
    state: usize,
    epsilon: f32,
    rng: &mut ThreadRng
) -> usize {
    let mut actions: Vec<usize> = (0..q_values[0].len()).collect();

    if rng.gen::<f32>() < epsilon {
        *actions.choose(rng).unwrap()
    } else {
        let best_action = actions
            .iter()
            .max_by(|&&a, &&b| q_values[state][a].partial_cmp(&q_values[state][b]).unwrap())
            .unwrap();
        *best_action
    }
}