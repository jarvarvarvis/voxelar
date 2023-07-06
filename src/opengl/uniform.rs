use std::marker::PhantomData;

use gl::types::*;

pub struct Uniform<'prog> {
    pub location: GLint,
    phantom: PhantomData<&'prog ()>
}

impl Uniform<'_> {
    pub fn new(location: GLint) -> Self {
        Self { 
            location,
            phantom: PhantomData
        }
    }

    pub fn set_float(&mut self, value: GLfloat) {
        unsafe {
            gl::Uniform1f(self.location, value);
        }
    }
}
