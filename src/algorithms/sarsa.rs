use std::time::{SystemTime, UNIX_EPOCH};
use crate::contracts::model_free_env::ModelFreeEnv;
use rand::prelude::SliceRandom;
use rand::Rng;

pub fn sarsa<TEnv: ModelFreeEnv>(
    num_episodes: usize,
    learning_rate: f32,
    gamma: f32,
    epsilon: f32,
) -> Vec<Vec<f32>> {
    let mut q_values = vec![vec![0.0; TEnv::num_actions()]; TEnv::num_states()];
    let mut env = TEnv::new();
    let mut rng = rand::thread_rng();

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64;

    for _ in 0..num_episodes {
        env.reset();
        let mut s = env.state_id();
        let mut available_actions = env.available_actions();
        let mut a = if rng.gen::<f32>() < epsilon {
            *available_actions.choose(&mut rng).expect("No available actions")
        } else {
            q_values[s]
                .iter()
                .enumerate()
                .filter(|(action, _)| available_actions.contains(action))
                .max_by(|(_, q1), (_, q2)| q1.partial_cmp(q2).unwrap())
                .map(|(action, _)| action)
                .expect("No valid action")
        };

        while !env.is_game_over() {
            let previous_score = env.score();
            env.step(a);

            let s_p = env.state_id();
            available_actions = env.available_actions();
            let r = env.score() - previous_score;

            let a_p = if rng.gen::<f32>() < epsilon {
                *available_actions.choose(&mut rng).expect("No available actions")
            } else {
                q_values[s_p]
                    .iter()
                    .enumerate()
                    .filter(|(action, _)| available_actions.contains(action))
                    .max_by(|(_, q1), (_, q2)| q1.partial_cmp(q2).unwrap())
                    .map(|(action, _)| action)
                    .expect("No valid action")
            };

            q_values[s][a] += learning_rate * (r + gamma * q_values[s_p][a_p] - q_values[s][a]);

            s = s_p;
            a = a_p;
        }
    }

    println!(
        "time : {}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64 - start_time
    );
    q_values
}