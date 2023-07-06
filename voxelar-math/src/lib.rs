#![feature(trait_alias)]

use std::ops::*;

pub mod vec_macro;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub trait MathType = Add + Sub + Div + Mul + PartialEq + Clone + Copy + Sized;
