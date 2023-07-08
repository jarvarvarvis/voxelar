#![feature(trait_alias)]

pub mod vec_macro;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub mod matrix;

pub trait MathType = PartialEq + Clone + Copy + Default + Sized;
