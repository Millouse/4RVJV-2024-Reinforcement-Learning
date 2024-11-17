use crate::envs::secret::paths::SECRET_ENV_PATH;
use std::ffi::c_void;

type NumStatesFn = extern fn() -> usize;
type NumActionsFn = extern fn() -> usize;
type NumRewardsFn = extern fn() -> usize;
type RewardFn = extern fn(usize) -> f32;
type TransitionProbabilityFn = extern fn(usize, usize, usize, usize) -> f32;

type NewFn = extern fn() -> *mut c_void;
type FromRandomStateFn = extern fn() -> *mut c_void;

type ResetFn = extern fn(*mut c_void);
type StateIdFn = extern fn(*mut c_void) -> usize;
type IsForbiddenFn = extern fn(*mut c_void, usize) -> bool;
type IsGameOverFn = extern fn(*mut c_void) -> bool;
type AvailableActionsFn = extern fn(*mut c_void) -> *mut usize;
type AvailableActionsLenFn = extern fn(*mut c_void) -> usize;
type AvailableActionsDeleteFn = extern fn(*mut usize, usize);

type StepFn = extern fn(*mut c_void, usize);
type ScoreFn = extern fn(*mut c_void) -> f32;

type DeleteFn = extern fn(*mut c_void);

thread_local! {
    static SECRET_LIB: libloading::Library = unsafe {
        libloading::Library::new(SECRET_ENV_PATH).expect("Failed to load library")
    };
}

pub struct SecretEnvWrapper {
    pub num_states_fn: NumStatesFn,
    pub num_actions_fn: NumActionsFn,
    pub num_rewards_fn: NumRewardsFn,
    pub reward_fn: RewardFn,
    pub transition_probability_fn: TransitionProbabilityFn,

    pub new_fn: NewFn,
    pub from_random_state_fn: FromRandomStateFn,

    pub reset_fn: ResetFn,
    pub state_id_fn: StateIdFn,
    pub is_forbidden_fn: IsForbiddenFn,
    pub is_game_over_fn: IsGameOverFn,
    pub available_actions_fn: AvailableActionsFn,
    pub available_actions_len_fn: AvailableActionsLenFn,
    pub available_actions_delete_fn: AvailableActionsDeleteFn,

    pub step_fn: StepFn,
    pub score_fn: ScoreFn,

    pub delete_fn: DeleteFn,
}

impl SecretEnvWrapper {
    pub fn from_secret_id(secret_env_id: u8) -> Self {
        let prefix = format!("secret_env_{}", secret_env_id);

        let num_states_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_num_states\0", prefix).as_bytes()).expect("Failed to load num_states"))
        };

        let num_actions_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_num_actions\0", prefix).as_bytes()).expect("Failed to load num_actions"))
        };

        let num_rewards_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_num_rewards\0", prefix).as_bytes()).expect("Failed to load num_rewards"))
        };

        let reward_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_reward\0", prefix).as_bytes()).expect("Failed to load reward"))
        };

        let transition_probability_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_transition_probability\0", prefix).as_bytes()).expect("Failed to load transition_probability"))
        };

        let new_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_new\0", prefix).as_bytes()).expect("Failed to load new"))
        };

        let from_random_state_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_from_random_state\0", prefix).as_bytes()).expect("Failed to load from_random_state"))
        };

        let reset_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_reset\0", prefix).as_bytes()).expect("Failed to load reset"))
        };

        let state_id_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_state_id\0", prefix).as_bytes()).expect("Failed to load state_id"))
        };

        let is_forbidden_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_is_forbidden\0", prefix).as_bytes()).expect("Failed to load is_forbidden"))
        };

        let is_game_over_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_is_game_over\0", prefix).as_bytes()).expect("Failed to load is_game_over"))
        };

        let available_actions_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_available_actions\0", prefix).as_bytes()).expect("Failed to load available_actions"))
        };

        let available_actions_len_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_available_actions_len\0", prefix).as_bytes()).expect("Failed to load available_actions_len"))
        };

        let available_actions_delete_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_available_actions_delete\0", prefix).as_bytes()).expect("Failed to load available_actions_delete"))
        };

        let step_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_step\0", prefix).as_bytes()).expect("Failed to load step"))
        };

        let score_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_score\0", prefix).as_bytes()).expect("Failed to load score"))
        };

        let delete_fn = unsafe {
            SECRET_LIB.with(|lib| *lib.get(format!("{}_delete\0", prefix).as_bytes()).expect("Failed to load delete"))
        };

        SecretEnvWrapper {
            num_states_fn,
            num_actions_fn,
            num_rewards_fn,
            reward_fn,
            transition_probability_fn,
            new_fn,
            from_random_state_fn,
            reset_fn,
            state_id_fn,
            is_forbidden_fn,
            is_game_over_fn,
            available_actions_fn,
            available_actions_len_fn,
            available_actions_delete_fn,
            step_fn,
            score_fn,
            delete_fn,
        }
    }
}

