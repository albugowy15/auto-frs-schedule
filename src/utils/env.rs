use std::env;

use env_logger::{Builder, Env};

pub fn setup_env() {
    env::set_var("RUST_BACKTRACE", "1");
    let env = Env::default()
        .filter("AUTO_FRS_SCHEDULE_LOG_LEVEL")
        .write_style("AUTO_FRS_SCHEDULE_LOG_STYLE");
    env::set_var("AUTO_FRS_SCHEDULE_LOG_LEVEL", "INFO");
    env::set_var("AUTO_FRS_SCHEDULE_LOG_STYLE", "AUTO");
    Builder::from_env(env)
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_env() {
        setup_env();

        assert_eq!(env::var("RUST_BACKTRACE").unwrap(), "1");
        assert_eq!(env::var("AUTO_FRS_SCHEDULE_LOG_LEVEL").unwrap(), "INFO");
        assert_eq!(env::var("AUTO_FRS_SCHEDULE_LOG_STYLE").unwrap(), "AUTO");
    }
}
