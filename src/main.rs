pub mod algorithms {
    pub mod policy_iteration;
    pub mod q_learning;
}

pub mod contracts {
    pub mod mdp_env;
    pub mod model_free_env;
}

pub mod envs {
    pub mod line_world;

    pub mod secret;
}

fn main() {
    println!("Policy Iteration on LineWorld(5)...");
    let (pi, value_function) = algorithms::policy_iteration::policy_iteration::<envs::line_world::LineWorld<5>>(0.999, 0.001);
    for (s, a) in pi.iter().enumerate() {
        println!("π(s={}) = {}", s, a);
    }
    println!();
    for (s, v) in value_function.iter().enumerate() {
        println!("V(s={}) = {}", s, v);
    }
    println!();
    println!();

    println!("Q-Learning on LineWorld(5)...");
    let q_values = algorithms::q_learning::q_learning::<envs::line_world::LineWorld<5>>(10_000, 0.1, 0.999, 1.0);
    for (s, q_s) in q_values.iter().enumerate() {
        for (a, q) in q_s.iter().enumerate() {
            println!("Q(s={}, a={}) = {}", s, a, q);
        }
    }

    // //Be careful, this will take a long time to run if you uncomment this
    // println!("Policy Iteration on SecretEnv0... (be careful, it will be very long as is)");
    // let (pi, value_function) = algorithms::policy_iteration::policy_iteration::<envs::secret::SecretEnv0>(0.999, 0.001);
    // println!("π(s=0) = {}", pi[0]);
    // println!();
    // println!("V(s=0) = {}", value_function[0]);
    // println!();
    // println!();

    println!("Q-Learning on SecretEnv0...");
    let q_values = algorithms::q_learning::q_learning::<envs::secret::SecretEnv0>(10_000, 0.1, 0.999, 1.0);
    for (a, q) in q_values[0].iter().enumerate() {
        println!("Q(s=0, a={}) = {}", a, q);
    }

    println!("Q-Learning on SecretEnv1...");
    let q_values = algorithms::q_learning::q_learning::<envs::secret::SecretEnv1>(10_000, 0.1, 0.999, 1.0);
    for (a, q) in q_values[0].iter().enumerate() {
        println!("Q(s=0, a={}) = {}", a, q);
    }

    println!("Q-Learning on SecretEnv2...");
    let q_values = algorithms::q_learning::q_learning::<envs::secret::SecretEnv2>(10_000, 0.1, 0.999, 1.0);
    for (a, q) in q_values[0].iter().enumerate() {
        println!("Q(s=0, a={}) = {}", a, q);
    }

    println!("Q-Learning on SecretEnv3...");
    let q_values = algorithms::q_learning::q_learning::<envs::secret::SecretEnv3>(10_000, 0.1, 0.999, 1.0);
    for (a, q) in q_values[0].iter().enumerate() {
        println!("Q(s=0, a={}) = {}", a, q);
    }
}
