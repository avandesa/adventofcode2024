use std::collections::HashMap;
use std::hash::Hash;

/// Return a sorted clone of the given `Vec`
pub fn sorted<T>(vec: &Vec<T>) -> Vec<T>
where
    T: Clone + Ord,
{
    let mut vec = vec.clone();
    vec.sort();
    vec
}

/// Return a map of the number of times each value appears in `iter`
pub fn occurrences<I, T>(iter: I) -> HashMap<T, u32>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    let mut occ = HashMap::new();
    for val in iter {
        occ.entry(val).and_modify(|count| *count += 1).or_default();
    }

    occ
}
