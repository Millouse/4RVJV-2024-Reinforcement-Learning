use std::cmp::max;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::contracts::mdp_env::MDPEnv;

pub fn value_iteration<TEnv: MDPEnv>(gamma: f32, theta: f32) -> (Vec<usize>, Vec<f32>) {

    let mut pi = vec![0usize; TEnv::num_states()];
    let mut value_function = vec![0.0; TEnv::num_states()];

    let num_states = TEnv::num_states();
    let num_actions = TEnv::num_actions();
    let num_rewards = TEnv::num_rewards();
    let rewards = (0..num_rewards).map(|r_index| TEnv::reward(r_index)).collect::<Vec<f32>>();
    let num_floats = num_states * num_actions * num_states * num_rewards;

    let mut cached_transition_probabilities = vec![42f32; num_floats];

    for s in 0..num_states {
        let index = s * num_actions * num_states * num_rewards;
        for a in 0..num_actions {
            let index = index + a * num_states * num_rewards;
            for s_p in 0..num_states {
                let index = index + s_p * num_rewards;
                for r_index in 0..num_rewards {
                    let index = index + r_index;
                    cached_transition_probabilities[index] = TEnv::transition_probability(s, a, s_p, r_index);
                }
            }
        }
    }

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64;

    loop {
        let mut delta = 0.0f32;
        for s in 0..num_states {
            let v = value_function[s];
            let mut actions = vec![0.0; num_actions];
            for a in 0..num_actions{
                let mut total = 0f32;
                let s_index = s * num_actions * num_states * num_rewards;
                for s_p in 0..num_states {
                    let s_p_index = s_index + a * num_states * num_rewards + s_p * num_rewards;
                    for r_index in 0..num_rewards {
                        let index = s_p_index + r_index;
                        total += cached_transition_probabilities[index] * (rewards[r_index] + gamma * value_function[s_p]);
                    }
                }
                actions[a] = total;
            }

            value_function[s] = actions[0];
            for i in actions{
                if i > value_function[s] {
                    value_function[s] = i;
                }
            }
            
            delta = delta.max((v - value_function[s]).abs());
        }
        if delta < theta {
            break;
        }
    }
    println!("time : {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64 - start_time);
    (pi, value_function)
}


