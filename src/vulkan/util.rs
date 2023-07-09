pub fn map_vec_ref<A, B, MapFn>(vec: &Vec<A>, map_fn: MapFn) -> Vec<B>
where
    MapFn: Fn(&A) -> B,
{
    vec.iter().map(|value| map_fn(value)).collect()
}
