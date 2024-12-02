/// Return a sorted clone of the given `Vec`
pub fn sorted<T>(vec: &Vec<T>) -> Vec<T>
where
    T: Clone + Ord,
{
    let mut vec = vec.clone();
    vec.sort();
    vec
}
