use crate::contracts::model_free_env::ModelFreeEnv;
use rand::prelude::SliceRandom;
use rand::Rng;

pub fn q_learning<TEnv: ModelFreeEnv>(
    num_episodes: usize,
    learning_rate: f32,
    gamma: f32,
    epsilon: f32,
) -> Vec<Vec<f32>> {
    let mut q_values = vec![vec![0.0; TEnv::num_actions()]; TEnv::num_states()];
    let mut env = TEnv::new();
    let mut rng = rand::thread_rng();

    for _ in 0..num_episodes {
        env.reset();
        while !env.is_game_over() {
            let s = env.state_id();
            let available_actions = env.available_actions();
            let a = if rng.gen::<f32>() < epsilon {
                *available_actions.choose(&mut rng).unwrap()
            } else {
                q_values[s].iter().enumerate().max_by(|(_, q1), (_, q2)| q1.partial_cmp(q2).unwrap()).unwrap().0
            };
            let previous_score = env.score();
            env.step(a);
            let r = env.score() - previous_score;
            let s_p = env.state_id();
            let q_s_p = q_values[s_p].iter().max_by(|q1, q2| q1.partial_cmp(q2).unwrap()).unwrap();
            q_values[s][a] += learning_rate * (r + gamma * q_s_p - q_values[s][a]);
        }
    }

    q_values
}