#![feature(extern_prelude)]
#![cfg_attr(not(feature="test"), no_std)]
#![cfg_attr(not(feature="test"), feature(alloc))]
#![feature(use_extern_macros)]
#![feature(proc_macro_hygiene)]
#![cfg(test)]

extern crate owasm_std;
extern crate owasm_ethereum;
extern crate pwasm_test;
extern crate owasm_abi;
extern crate owasm_abi_derive;

mod erc20;
mod arrays;
mod trivia;
mod payable;
mod multiple_return;
mod general;
