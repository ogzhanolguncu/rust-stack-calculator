use std::env;

pub fn read_command_args() -> Vec<String> {
    env::args().skip(1).collect()
}
