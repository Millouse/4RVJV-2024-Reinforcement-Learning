use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::contracts::mdp_env::MDPEnv;

pub fn on_policy_first_visit_mc_control<TEnv: MDPEnv>(
    gamma: f32,
    num_episodes: usize,
    epsilon: f32,
) -> Vec<usize> {
    let num_states = TEnv::num_states();
    let num_actions = TEnv::num_actions();

    let mut policy = vec![0usize; num_states];
    let mut returns: HashMap<(usize, usize), Vec<f32>> = HashMap::new();
    let mut q_values = vec![vec![0.0f32; num_actions]; num_states];

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64;

    for _ in 0..num_episodes {
        let mut episode = Vec::new();

        let mut state = TEnv::reset();
        loop {
            let action = if rand::random::<f32>() < epsilon {
                rand::random::<usize>() % num_actions
            } else {
                policy[state]
            };

            let (next_state, reward, done) = TEnv::step(state, action);
            episode.push((state, action, reward));
            state = next_state;

            if done {
                break;
            }
        }

        // First-visit Monte Carlo control
        let mut g = 0.0;
        let mut visited = HashMap::new();

        for (t, &(state, action, reward)) in episode.iter().rev().enumerate() {
            g = reward + gamma * g;

            if !visited.contains_key(&(state, action)) {
                visited.insert((state, action), true);

                returns.entry((state, action)).or_insert(Vec::new()).push(g);
                let mean_return = returns[&(state, action)].iter().copied().sum::<f32>()
                    / returns[&(state, action)].len() as f32;
                q_values[state][action] = mean_return;

                policy[state] = (0..num_actions)
                    .max_by(|&a1, &a2| {
                        q_values[state][a1]
                            .partial_cmp(&q_values[state][a2])
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap();
            }
        }
    }
    println!("time: {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64 - start_time);
    policy
}