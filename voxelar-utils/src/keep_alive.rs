use std::marker::PhantomData;

/// A utility type that keeps a value of `T` alive for the lifetime of a reference
/// to the type `Other`.
pub struct KeepAlive<'other, T, Other> {
    value: T,
    phantom: PhantomData<&'other Other>,
}

impl<'other, T, Other> KeepAlive<'other, T, Other> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<'other, T, Other> std::ops::Deref for KeepAlive<'other, T, Other> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

