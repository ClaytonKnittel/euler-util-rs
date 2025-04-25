use std::{iter::Cloned, slice};

use either::Either;

use crate::owned_iterator::OwnedIterator;

pub fn num_partitions(n: u32, k: u32) -> impl Iterator<Item = impl Iterator<Item = u32>> {
  num_partitions_helper(n, k, n)
}

pub fn num_partitions2(n: u32, k: u32) -> NumPartitiosIter {
  NumPartitiosIter::new(n, k)
}

pub struct NumPartitiosIter {
  n: u32,
  k: u32,
  stack: Vec<u32>,
}

impl NumPartitiosIter {
  fn new(n: u32, k: u32) -> Self {
    debug_assert!(k <= n);
    debug_assert!(n > 0);
    debug_assert!(k > 0);

    let mut stack = Vec::with_capacity(k as usize);
    stack.push(n - k + 2);
    Self { n, k, stack }
  }
}

impl OwnedIterator for NumPartitiosIter {
  type Item<'a>
    = Cloned<slice::Iter<'a, u32>>
  where
    Self: 'a;

  fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
    let mut total: u32 = self.stack.iter().sum();
    let mut prev_choice = self.stack.pop()?;
    total -= prev_choice;
    while prev_choice == 1
      || (prev_choice - 1) * (self.k - self.stack.len() as u32) < (self.n - total)
    {
      prev_choice = self.stack.pop()?;
      total -= prev_choice.min(self.n);
    }

    let next_choice = prev_choice - 1;
    self.stack.push(next_choice);
    total += next_choice;

    for remaining in (0..(self.k - self.stack.len() as u32)).rev() {
      let choice = (self.n - total - remaining).min(next_choice);
      self.stack.push(choice);
      total += choice;
    }

    Some(self.stack.iter().cloned())
  }
}

fn num_partitions_helper(
  n: u32,
  k: u32,
  largest: u32,
) -> Box<dyn Iterator<Item = Box<dyn Iterator<Item = u32>>>> {
  Box::new(
    (n == 0 && k == 0)
      .then(|| {
        Either::Left(std::iter::once(
          Box::new(std::iter::empty()) as Box<dyn Iterator<Item = u32>>
        ))
      })
      .or_else(|| {
        (k > 0 && n >= k && k * largest >= n).then(move || {
          Either::Right((1..=largest.min(n)).flat_map(move |next_largest| {
            num_partitions_helper(n - next_largest, k - 1, next_largest).map(
              move |p_iter| -> Box<dyn Iterator<Item = _>> {
                Box::new(std::iter::once(next_largest).chain(p_iter))
              },
            )
          }))
        })
      })
      .into_iter()
      .flatten(),
  )
}

#[cfg(test)]
mod tests {
  use googletest::{assert_that, prelude::unordered_elements_are};
  use itertools::Itertools;

  use crate::{num_partitions::num_partitions, owned_iterator::OwnedIterator};

  use super::NumPartitiosIter;

  fn collect_partitions(mut partitions_iter: NumPartitiosIter) -> Vec<Vec<u32>> {
    let mut result = vec![];
    while let Some(partition) = partitions_iter.next() {
      result.push(partition.collect());
    }
    result
  }

  #[test]
  fn test_one() {
    assert_that!(
      num_partitions(1, 1)
        .map(|partition| partition.collect_vec())
        .collect_vec(),
      unordered_elements_are![unordered_elements_are![&1]]
    );

    assert_that!(
      collect_partitions(NumPartitiosIter::new(1, 1)),
      unordered_elements_are![unordered_elements_are![&1]]
    );
  }

  #[test]
  fn test_two() {
    assert_that!(
      num_partitions(2, 1)
        .map(|partition| partition.collect_vec())
        .collect_vec(),
      unordered_elements_are![unordered_elements_are![&2]]
    );

    assert_that!(
      collect_partitions(NumPartitiosIter::new(2, 1)),
      unordered_elements_are![unordered_elements_are![&2]]
    );
  }

  #[test]
  fn test_two_two() {
    assert_that!(
      num_partitions(2, 2)
        .map(|partition| partition.collect_vec())
        .collect_vec(),
      unordered_elements_are![unordered_elements_are![&1, &1]]
    );

    assert_that!(
      collect_partitions(NumPartitiosIter::new(2, 2)),
      unordered_elements_are![unordered_elements_are![&1, &1]]
    );
  }

  #[test]
  fn test_four_two() {
    assert_that!(
      num_partitions(4, 2)
        .map(|partition| partition.collect_vec())
        .collect_vec(),
      unordered_elements_are![
        unordered_elements_are![&1, &3],
        unordered_elements_are![&2, &2],
      ]
    );

    assert_that!(
      collect_partitions(NumPartitiosIter::new(4, 2)),
      unordered_elements_are![
        unordered_elements_are![&1, &3],
        unordered_elements_are![&2, &2],
      ]
    );
  }

  #[test]
  fn test_seven_three() {
    assert_that!(
      num_partitions(7, 3)
        .map(|partition| partition.collect_vec())
        .collect_vec(),
      unordered_elements_are![
        unordered_elements_are![&1, &1, &5],
        unordered_elements_are![&1, &2, &4],
        unordered_elements_are![&1, &3, &3],
        unordered_elements_are![&2, &2, &3],
      ]
    );

    assert_that!(
      collect_partitions(NumPartitiosIter::new(7, 3)),
      unordered_elements_are![
        unordered_elements_are![&1, &1, &5],
        unordered_elements_are![&1, &2, &4],
        unordered_elements_are![&1, &3, &3],
        unordered_elements_are![&2, &2, &3],
      ]
    );
  }
}
