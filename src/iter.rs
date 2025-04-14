use itertools::Itertools;

pub fn all_combinations<T: Clone>(list: &[T]) -> impl Iterator<Item = Vec<T>> {
  (1..=list.len()).flat_map(|k| list.iter().cloned().combinations(k))
}

pub fn all_subset_combinations<T: Clone>(list: &[T]) -> impl Iterator<Item = Vec<T>> {
  (1..list.len()).flat_map(|k| list.iter().cloned().combinations(k))
}
