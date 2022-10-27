#![deny(
    dead_code,
    nonstandard_style,
    unreachable_patterns,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unused_variables
)]

pub mod api;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
