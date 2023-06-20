#![feature(exclusive_range_pattern)]
#![feature(core_intrinsics)]
#![feature(never_type)]
#![deny(unused_extern_crates)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![warn(unreachable_code)]
#![feature(generators)]
#![feature(generator_trait)]


// crate 相当于project
use crate::chap01::*;
use crate::chap02::*;

// mod 相当于namespace
mod chap01;
mod chap02;

