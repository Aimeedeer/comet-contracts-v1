#![no_std]

use soroban_sdk::contractimpl;
pub mod c_consts;
pub mod c_math;
pub mod c_num;
pub mod c_pool;

#[cfg(test)]
mod tests;

#[cfg(test)]
extern crate std;
