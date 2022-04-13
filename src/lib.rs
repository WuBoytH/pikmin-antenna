#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod pikmin;

#[skyline::main(name = "pikmin_antenna")]
pub fn main() {
    pikmin::install();
}