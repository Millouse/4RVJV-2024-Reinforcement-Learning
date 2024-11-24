use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::contracts::model_free_env::ModelFreeEnv;
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::Rng;

pub fn dyna_q<TEnv: ModelFreeEnv>(
    n: usize,
    num_episodes: usize,
    learning_rate: f32,
    gamma: f32,
    epsilon: f32,) -> Vec<Vec<f32>> {

    let mut q_values = vec![vec![0.0; TEnv::num_actions()]; TEnv::num_states()];
    let mut model: Vec<Vec<(f32, usize)>> = vec![vec![(0.0, 0); TEnv::num_actions()]; TEnv::num_states()];

    let mut parcouredMap: HashMap<(usize, usize), (f32, usize)> = HashMap::new();

    let mut env = TEnv::new();
    let mut rng = rand::thread_rng();

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64;

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

            model[s][a] = (r, s_p);
            parcouredMap.insert((s, a), (r, s_p));
            for l in 0..n{
                let ((l_s, l_a),(l_r, l_s_p)) = parcouredMap.iter().choose(&mut rng).unwrap();
                let q_s_p = q_values[*l_s_p].iter().max_by(|q1, q2| q1.partial_cmp(q2).unwrap()).unwrap();
                q_values[*l_s][*l_a] += learning_rate * (*l_r + gamma * q_s_p - q_values[*l_s][*l_a]);
            }
        }
    }

    println!("time : {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64 - start_time);
    q_values
}