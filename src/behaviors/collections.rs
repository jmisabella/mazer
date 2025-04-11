use std::collections::HashSet;
use std::hash::Hash;

pub trait SetDifference<T>
where
    T: Eq + Hash + Clone,
{
    /// Compute the set difference between self and any other collection.
    fn diff<U: AsRef<[T]>>(&self, other: U) -> Vec<T>;
}

impl<T, C> SetDifference<T> for C
where
    C: AsRef<[T]>,
    T: Eq + Hash + Clone,
{
    fn diff<U: AsRef<[T]>>(&self, other: U) -> Vec<T> {
        let other_set: HashSet<_> = other.as_ref().iter().collect();
        self.as_ref()
            .iter()
            .filter(|item| !other_set.contains(item))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::SetDifference;

    #[test]
    fn test_set_difference() {
        let vec1 = vec![1, 2, 3, 4, 5];
        let vec2 = vec![2, 4];

        // This should return [1, 3, 5] as those are in `vec1` but not in `vec2`
        let diff = vec1.diff(&vec2);
        assert_eq!(diff, vec![1, 3, 5]);
    }
}
