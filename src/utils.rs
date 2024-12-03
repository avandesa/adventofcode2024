/// Return a sorted clone of the given slice
pub fn sorted<T>(s: &[T]) -> Vec<T>
where
    T: Clone + Ord,
{
    let mut vec = s.to_owned();
    vec.sort();
    vec
}
