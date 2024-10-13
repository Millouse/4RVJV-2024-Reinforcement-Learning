use rand::Rng;

pub trait ModelFreeEnv {
    fn new() -> Self;
    fn from_random_state(rng: &mut impl Rng) -> Self;
    fn num_states() -> usize;
    fn num_actions() -> usize;
    fn reset(&mut self);
    fn is_game_over(&self) -> bool;
    fn score(&self) -> f32;
    fn state_id(&self) -> usize;
    fn is_forbidden(&self, action: usize) -> bool;
    fn available_actions(&self) -> Vec<usize>;
    fn step(&mut self, action: usize);
}