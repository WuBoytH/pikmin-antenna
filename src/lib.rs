#![feature(proc_macro_hygiene)]
#![allow(unsafe_op_in_unsafe_fn)]

mod pikmin;

#[skyline::main(name = "pikmin_antenna")]
pub fn main() {
    pikmin::install();
}