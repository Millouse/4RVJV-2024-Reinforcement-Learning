use crate::contracts::mdp_env::MDPEnv;
use crate::contracts::model_free_env::ModelFreeEnv;
use crate::envs::secret::secret_env_wrapper::SecretEnvWrapper;
use rand::Rng;
use std::ffi::c_void;

pub struct SecretEnv<const ENV_ID: u8> {
    wrapper: SecretEnvWrapper,
    instance: *mut c_void,
}

impl<const ENV_ID: u8> MDPEnv for SecretEnv<ENV_ID> {
    fn num_states() -> usize {
        (SecretEnvWrapper::from_secret_id(ENV_ID).num_states_fn)()
    }

    fn num_actions() -> usize {
        (SecretEnvWrapper::from_secret_id(ENV_ID).num_actions_fn)()
    }

    fn num_rewards() -> usize {
        (SecretEnvWrapper::from_secret_id(ENV_ID).num_rewards_fn)()
    }

    fn reward(index: usize) -> f32 {
        (SecretEnvWrapper::from_secret_id(ENV_ID).reward_fn)(index)
    }

    fn transition_probability(state: usize, action: usize, next_state: usize, reward_index: usize) -> f32 {
        (SecretEnvWrapper::from_secret_id(ENV_ID).transition_probability_fn)(state, action, next_state, reward_index)
    }
}

impl<const ENV_ID: u8> ModelFreeEnv for SecretEnv<ENV_ID> {
    fn new() -> Self {
        let wrapper = SecretEnvWrapper::from_secret_id(ENV_ID);
        let instance = (wrapper.new_fn)();
        SecretEnv {
            wrapper,
            instance,
        }
    }

    fn from_random_state(_rng: &mut impl Rng) -> Self {
        let wrapper = SecretEnvWrapper::from_secret_id(ENV_ID);
        let instance = (wrapper.from_random_state_fn)();
        SecretEnv {
            wrapper,
            instance,
        }
    }

    fn num_states() -> usize {
        (SecretEnvWrapper::from_secret_id(ENV_ID).num_states_fn)()
    }

    fn num_actions() -> usize {
        (SecretEnvWrapper::from_secret_id(ENV_ID).num_actions_fn)()
    }

    fn reset(&mut self) {
        (self.wrapper.reset_fn)(self.instance)
    }

    fn is_game_over(&self) -> bool {
        (self.wrapper.is_game_over_fn)(self.instance)
    }

    fn score(&self) -> f32 {
        (self.wrapper.score_fn)(self.instance)
    }

    fn state_id(&self) -> usize {
        (self.wrapper.state_id_fn)(self.instance)
    }

    fn is_forbidden(&self, action: usize) -> bool {
        (self.wrapper.is_forbidden_fn)(self.instance, action)
    }

    fn available_actions(&self) -> Vec<usize> {
        let mut vec = Vec::new();
        let available_actions_raw = (self.wrapper.available_actions_fn)(self.instance);
        let available_actions_len = (self.wrapper.available_actions_len_fn)(self.instance);

        for i in 0..available_actions_len {
            vec.push(
                unsafe {
                    *available_actions_raw.add(i)
                }
            );
        }

        (self.wrapper.available_actions_delete_fn)(available_actions_raw, available_actions_len);
        vec
    }

    fn step(&mut self, action: usize) {
        (self.wrapper.step_fn)(self.instance, action)
    }
}

impl<const ENV_ID: u8> Drop for SecretEnv<ENV_ID> {
    fn drop(&mut self) {
        (self.wrapper.delete_fn)(self.instance)
    }
}