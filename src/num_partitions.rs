use either::Either;

pub fn num_partitions(n: u32, k: u32) -> impl Iterator<Item = impl Iterator<Item = u32>> {
  num_partitions_helper(n, k, n)
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

  use crate::num_partitions::num_partitions;

  #[test]
  fn test_one() {
    assert_that!(
      num_partitions(1, 1)
        .map(|partition| partition.collect_vec())
        .collect_vec(),
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
  }

  #[test]
  fn test_two_two() {
    assert_that!(
      num_partitions(2, 2)
        .map(|partition| partition.collect_vec())
        .collect_vec(),
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
        unordered_elements_are![&2, &2]
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
  }
}
