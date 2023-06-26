let ENV_RECOVERY_VAR = "OR_RECOVERY"

fn get_args() -> IntoIterator<String> {
    match env::var(ENV_RECOVERY_VAR) {
        Err(_) => std::env::args()
        Ok(previous_command) => shellwords::split(previous_command)?
    }
}