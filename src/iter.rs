use itertools::Itertools;

struct PartitionsTable {
  partitions_count: Vec<usize>,
  levels: u32,
}

impl PartitionsTable {
  fn new() -> Self {
    Self { partitions_count: vec![], levels: 0 }
  }

  const fn idx(n: u32, m: u32) -> usize {
    debug_assert!(n >= 1);
    debug_assert!(m >= 1);

    let n = n as usize;
    let m = m as usize;
    let level = n + m - 2;
    level * (level + 1) / 2 + (n - 1)
  }

  fn ensure_filled(&mut self, n: u32, m: u32) {
    let required_levels = n + m - 1;
    if self.levels >= required_levels {
      return;
    }

    let num_required_elements = Self::idx(required_levels, 1) + 1;
    self
      .partitions_count
      .reserve(num_required_elements.saturating_sub(self.partitions_count.len()));
    for level in (self.levels + 1)..(required_levels + 1) {
      for idx in 0..level {
        let (n, m) = (idx + 1, level - idx);
        debug_assert_eq!(Self::idx(n, m), self.partitions_count.len());

        let count = if n == 1 {
          1
        } else {
          m as usize * self.partitions_count[Self::idx(n - 1, m)]
            + self.partitions_count[Self::idx(n - 1, m + 1)]
        };
        self.partitions_count.push(count);
      }
    }

    debug_assert_eq!(self.partitions_count.len(), num_required_elements);
    self.levels = required_levels;
  }

  fn num_partitions(&mut self, n: u32, m: u32) -> usize {
    self.ensure_filled(n, m);
    self.partitions_count[Self::idx(n, m)]
  }
}

pub trait IterUtil<T> {
  fn all_combinations(self) -> impl Iterator<Item = Vec<T>>;

  fn all_subset_combinations(self) -> impl Iterator<Item = Vec<T>>;

  fn all_partitions(self) -> impl ExactSizeIterator<Item = Vec<Vec<T>>>;
}

impl<I, T> IterUtil<T> for I
where
  I: ExactSizeIterator<Item = T> + Clone,
  T: Clone,
{
  fn all_combinations(self) -> impl Iterator<Item = Vec<T>> {
    (1..=self.len()).flat_map(move |k| self.clone().combinations(k))
  }

  fn all_subset_combinations(self) -> impl Iterator<Item = Vec<T>> {
    (1..self.len()).flat_map(move |k| self.clone().combinations(k))
  }

  fn all_partitions(self) -> impl ExactSizeIterator<Item = Vec<Vec<T>>> {
    let mut assignments = vec![0; self.len()];

    let arrangements: usize = PartitionsTable::new().num_partitions(self.len() as u32, 1);
    (0..arrangements).map(move |_| {
      let mut v: Vec<Vec<T>> = vec![vec![]; self.len()];

      let max_idx = self
        .clone()
        .enumerate()
        .map(|(i, el)| {
          let idx = assignments[i];
          v[idx].push(el);
          idx
        })
        .max()
        .unwrap();
      v.resize_with(max_idx + 1, || unreachable!());

      for i in (0..self.len()).rev() {
        let max = assignments.iter().cloned().take(i).max().unwrap_or(0) + 1;
        assignments[i] += 1;
        if assignments[i] > max {
          assignments[i] = 0;
        } else {
          break;
        }
      }

      v
    })
  }
}

#[cfg(test)]
mod tests {
  use googletest::{assert_that, prelude::unordered_elements_are};
  use itertools::Itertools;
  use rand::{rng, seq::SliceRandom};

  use crate::iter::PartitionsTable;

  use super::IterUtil;

  #[test]
  fn test_idx() {
    let mut g_idx = 0;
    for level in 0..10 {
      for idx in 0..=level {
        let n = idx + 1;
        let m = level - idx + 1;
        assert_eq!(
          PartitionsTable::idx(n, m),
          g_idx,
          "Expected idx of n={n}, m={m} = {g_idx}"
        );

        g_idx += 1;
      }
    }
  }

  fn naive_partitions_count(n: u32, m: u32) -> usize {
    if n == 1 {
      1
    } else {
      m as usize * naive_partitions_count(n - 1, m) + naive_partitions_count(n - 1, m + 1)
    }
  }

  #[test]
  fn test_partitions_count() {
    let mut partitions_table = PartitionsTable::new();
    for n in 1..=5 {
      for m in 1..=n {
        let table_count = partitions_table.num_partitions(n, m);
        let ref_count = naive_partitions_count(n, m);
        assert_eq!(
          table_count, ref_count,
          "Expected partition count of n={n} m={m} = {ref_count}, but got {table_count}"
        );
      }
    }
  }

  #[test]
  fn test_partitions_count_randomized() {
    let mut partitions_table = PartitionsTable::new();
    let mut params = (1..=8)
      .flat_map(|n| (1..=n).map(move |m| (n, m)))
      .collect_vec();
    params.shuffle(&mut rng());

    for (n, m) in params {
      let table_count = partitions_table.num_partitions(n, m);
      let ref_count = naive_partitions_count(n, m);
      assert_eq!(
        table_count, ref_count,
        "Expected partition count of n={n} m={m} = {ref_count}, but got {table_count}"
      );
    }
  }

  #[test]
  fn test_partitions() {
    let iter = 1..5;
    assert_that!(
      iter.all_partitions().collect::<Vec<Vec<Vec<_>>>>(),
      unordered_elements_are![
        unordered_elements_are![&[1, 2, 3, 4]],
        unordered_elements_are![&[1, 2, 3], &[4]],
        unordered_elements_are![&[1, 2, 4], &[3]],
        unordered_elements_are![&[1, 3, 4], &[2]],
        unordered_elements_are![&[2, 3, 4], &[1]],
        unordered_elements_are![&[1, 2], &[3, 4]],
        unordered_elements_are![&[1, 3], &[2, 4]],
        unordered_elements_are![&[1, 4], &[2, 3]],
        unordered_elements_are![&[1, 2], &[3], &[4]],
        unordered_elements_are![&[1, 3], &[2], &[4]],
        unordered_elements_are![&[1, 4], &[2], &[3]],
        unordered_elements_are![&[1], &[2, 3], &[4]],
        unordered_elements_are![&[1], &[2, 4], &[3]],
        unordered_elements_are![&[1], &[2], &[3, 4]],
        unordered_elements_are![&[1], &[2], &[3], &[4]],
      ]
    );
  }
}
