pub fn map_vec_ref<A, B, MapFn>(vec: &Vec<A>, map_fn: MapFn) -> Vec<B>
where
    MapFn: Fn(&A) -> B,
{
    vec.iter().map(|value| map_fn(value)).collect()
}

// Simple offset_of macro akin to C++ offsetof
#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize
        }
    }};
}
