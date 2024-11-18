pub mod algorithms {
    pub mod policy_iteration;
    pub mod value_iteration;
    pub mod q_learning;
    pub mod monte_carlo_exploring_starts;
    pub mod sarsa;
}

pub mod contracts {
    pub mod mdp_env;
    pub mod model_free_env;
}

pub mod envs {
    pub mod line_world;
    pub mod grid_world;
    pub mod secret;
    pub mod mounty_hall_3;
}

fn main() {
    println!("Policy Iteration on GridWorld(4x4)...");
    let (pi, value_function) = algorithms::policy_iteration::policy_iteration::<envs::grid_world::GridWorld<4, 4>>(0.999, 0.001);
    for (s, a) in pi.iter().enumerate() {
        println!("π(s={}) = {}", s, a);
    }
    println!();
    for (s, v) in value_function.iter().enumerate() {
        println!("V(s={}) = {}", s, v);
    }
    println!();
    println!();

    println!("Q-Learning on GridWorld(4x4)...");
    let q_values = algorithms::q_learning::q_learning::<envs::grid_world::GridWorld<4, 4>>(10_000, 0.1, 0.999, 1.0);
    for (s, q_s) in q_values.iter().enumerate() {
        for (a, q) in q_s.iter().enumerate() {
            println!("Q(s={}, a={}) = {}", s, a, q);
        }
    }
    println!("Monte Carlo Exploring Starts on GridWorld(4x4)...");
    let q_values = algorithms::monte_carlo_exploring_starts::monte_carlo_exploring_starts::<envs::grid_world::GridWorld<4, 4>>(15_000, 0.999);
    for (s, q_s) in q_values.iter().enumerate() {
        for (a, q) in q_s.iter().enumerate() {
            println!("Q(s={}, a={}) = {}", s, a, q);
        }
    }
    // Calcul et affichage de la politique optimale π(s)
    let pi: Vec<usize> = q_values.iter().map(|q_s| {
        q_s.iter().enumerate().max_by(|(_, q1), (_, q2)| q1.partial_cmp(q2).unwrap()).unwrap().0
    }).collect();

    println!("\nPolitique optimale π(s):");
    for (s, a) in pi.iter().enumerate() {
        println!("π(s={}) = {}", s, a);
    }
    println!();

    println!("Monte Carlo Exploring Starts on LineWorld(5)...");
    let q_values = algorithms::monte_carlo_exploring_starts::monte_carlo_exploring_starts::<envs::line_world::LineWorld<5>>(10_000, 0.999);
    for (s, q_s) in q_values.iter().enumerate() {
        for (a, q) in q_s.iter().enumerate() {
            println!("Q(s={}, a={}) = {}", s, a, q);
        }
    }
    // Calcul et affichage de la politique optimale π(s)
    let pi: Vec<usize> = q_values.iter().map(|q_s| {
        q_s.iter().enumerate().max_by(|(_, q1), (_, q2)| q1.partial_cmp(q2).unwrap()).unwrap().0
    }).collect();

    println!("\nPolitique optimale π(s):");
    for (s, a) in pi.iter().enumerate() {
        println!("π(s={}) = {}", s, a);
    }

    //println!("SARSA on GridWorld(4x4)...");
    //let q_values = algorithms::sarsa::sarsa::<envs::grid_world::GridWorld<4, 4>>(10_000, 0.1, 0.999, 1.0);
    //for (s, q_s) in q_values.iter().enumerate() {
    //    for (a, q) in q_s.iter().enumerate() {
    //        println!("Q(s={}, a={}) = {}", s, a, q);
    //    }
    //}
    //println!();

    //println!("Policy Iteration on Monty Hall Problem...");

    // Exécution de l'algorithme Policy Iteration pour le Monty Hall Problem
    //let (pi, value_function) = algorithms::policy_iteration::policy_iteration::<envs::mounty_hall_3::MontyHall>(0.999, 0.001);

    // Affichage de la politique optimale pour chaque état
    //println!("Optimal Policy (π):");
    //for (s, a) in pi.iter().enumerate() {
    //    println!("π(s={}) = {}", s, a);
    //}

    //println!();

    // Affichage de la fonction de valeur optimale pour chaque état
    //println!("Optimal Value Function (V):");
    //for (s, v) in value_function.iter().enumerate() {
    //    println!("V(s={}) = {}", s, v);
    //}

    //println!();

    // println!("Policy Iteration on LineWorld(5)...");
    // let (pi, value_function) = algorithms::policy_iteration::policy_iteration::<envs::line_world::LineWorld<5>>(0.999, 0.001);
    // for (s, a) in pi.iter().enumerate() {
    //     println!("π(s={}) = {}", s, a);
    // }
    // println!();
    // for (s, v) in value_function.iter().enumerate() {
    //     println!("V(s={}) = {}", s, v);
    // }
    // println!();
    // println!();




    // println!("Q-Learning on LineWorld(5)...");
    // let q_values = algorithms::q_learning::q_learning::<envs::line_world::LineWorld<5>>(10_000, 0.1, 0.999, 1.0);
    // for (s, q_s) in q_values.iter().enumerate() {
    //     for (a, q) in q_s.iter().enumerate() {
    //         println!("Q(s={}, a={}) = {}", s, a, q);
    //     }
    // }

    //Be careful, this will take a long time to run if you uncomment this
    // println!("Policy Iteration on SecretEnv0... (be careful, it will be very long as is)");
    // let (pi, value_function) = algorithms::policy_iteration::policy_iteration::<envs::secret::SecretEnv0>(0.999, 0.001);
    // println!("π(s=0) = {}", pi[0]);
    // println!();
    // println!("V(s=0) = {}", value_function[0]);
    // println!();
    // println!();

    // println!("Q-Learning on SecretEnv0...");
    // let q_values = algorithms::q_learning::q_learning::<envs::secret::SecretEnv0>(10_000, 0.1, 0.999, 1.0);
    // for (a, q) in q_values[0].iter().enumerate() {
    //     println!("Q(s=0, a={}) = {}", a, q);
    // }
    //
    // println!("Q-Learning on SecretEnv1...");
    // let q_values = algorithms::q_learning::q_learning::<envs::secret::SecretEnv1>(10_000, 0.1, 0.999, 1.0);
    // for (a, q) in q_values[0].iter().enumerate() {
    //     println!("Q(s=0, a={}) = {}", a, q);
    // }
    //
    // println!("Q-Learning on SecretEnv2...");
    // let q_values = algorithms::q_learning::q_learning::<envs::secret::SecretEnv2>(10_000, 0.1, 0.999, 1.0);
    // for (a, q) in q_values[0].iter().enumerate() {
    //     println!("Q(s=0, a={}) = {}", a, q);
    // }
    //
    // println!("Q-Learning on SecretEnv3...");
    // let q_values = algorithms::q_learning::q_learning::<envs::secret::SecretEnv3>(10_000, 0.1, 0.999, 1.0);
    // for (a, q) in q_values[0].iter().enumerate() {
    //     println!("Q(s=0, a={}) = {}", a, q);
    // }
}
