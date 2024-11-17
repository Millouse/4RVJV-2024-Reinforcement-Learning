#[cfg(target_os = "linux")]
pub const SECRET_ENV_PATH: &str = "./libs/libsecret_envs.so";
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const SECRET_ENV_PATH: &str = "./libs/libsecret_envs_intel_macos.dylib";
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const SECRET_ENV_PATH: &str = "./libs/libsecret_envs.dylib";
#[cfg(windows)]
pub const SECRET_ENV_PATH: &str = "./libs/secret_envs.dll";