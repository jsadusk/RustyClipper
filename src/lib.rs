#![feature(ptr_offset_from)]
#![feature(try_from)]
extern crate libc;
#[macro_use]
extern crate failure;

pub mod cpp;
pub mod error;
pub mod traits;
pub mod simple_polygon;
