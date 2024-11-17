pub trait MDPEnv {
    fn num_states() -> usize;
    fn num_actions() -> usize;
    fn num_rewards() -> usize;
    fn reward(index: usize) -> f32;
    fn transition_probability(state: usize, action: usize, next_state: usize, reward_index: usize) -> f32;
}