#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod throwmove;

#[skyline::main(name = "throwmove")]
pub fn main() {
    throwmove::install();
}