#![feature(trait_alias)]

pub mod vec_macro;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub mod mat_macros;
pub mod mat2;
pub mod mat3;
pub mod mat4;

pub trait MathType = PartialEq + Clone + Copy + Sized;
