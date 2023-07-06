#![feature(trait_alias)]

use std::ops::*;

pub mod vec;
pub mod vec2;
pub mod vec3;

pub trait MathType = Add + Sub + Div + Mul + Neg + Clone + Copy + Sized;
