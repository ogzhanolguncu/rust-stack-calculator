use std::env;

pub fn read_command_args() -> Vec<String> {
    env::args().skip(1).collect()
}
pub fn first<T>(v: &[T]) -> Option<&T> {
    v.first()
}
