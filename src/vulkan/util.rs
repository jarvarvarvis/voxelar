pub fn map_vec_ref<A, B, MapFn>(vec: &Vec<A>, map_fn: MapFn) -> Vec<B>
where
    MapFn: Fn(&A) -> B,
{
    vec.iter().map(|value| map_fn(value)).collect()
}

/// Taken from [this](https://stackoverflow.com/a/42186553) answer
///
///
/// SAFETY
///
/// This function is unsafe because any padding bytes in the struct may be uninitialized memory,
/// so the probability of undefined behavior is high.
pub const unsafe fn transmute_as_u8_slice<T: Sized>(value: &T) -> &[u8] {
    std::slice::from_raw_parts((value as *const T) as *const u8, std::mem::size_of::<T>())
}
