#![feature(trait_alias)]

pub mod identity;

pub mod vec_macro;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub mod quaternion;

pub mod matrix;
pub mod matrix_types;
pub mod matrix_identity;
pub mod matrix_transform;

pub trait MathType = PartialEq + Clone + Copy + Default + Sized;
