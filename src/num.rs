use std::fmt::{Debug, Display};

use itertools::Itertools;

#[derive(Clone)]
pub struct PrimeFactorization {
  primes: Vec<(u32, u32)>,
}

impl PrimeFactorization {
  pub fn zero() -> Self {
    Self { primes: vec![] }
  }

  pub fn one() -> Self {
    Self { primes: vec![(2, 0)] }
  }

  pub fn from_primes(primes: impl IntoIterator<Item = (u32, u32)>) -> Self {
    Self { primes: primes.into_iter().collect() }
  }

  pub fn is_zero(&self) -> bool {
    self.primes.is_empty()
  }

  pub fn is_one(&self) -> bool {
    matches!(self.primes.as_slice(), [(2, 0)])
  }

  pub fn is_prime(&self) -> bool {
    self.primes.len() == 1 && !self.is_one()
  }

  pub fn prime_factors(&self) -> impl Iterator<Item = u32> {
    self.primes.iter().map(|&(p, _)| p)
  }

  pub fn val(&self) -> u32 {
    if self.is_zero() {
      0
    } else {
      self.primes.iter().map(|&(p, exp)| p.pow(exp)).product()
    }
  }

  pub fn with_p(self, p: u32, exp: u32) -> Self {
    let is_one = self.is_one();
    let mut primes = self.primes;
    if is_one {
      primes = vec![(p, exp)];
    } else if let Some((_, other_exp)) = primes.iter_mut().find(|(other_p, _)| p == *other_p) {
      *other_exp += exp;
    } else {
      primes.push((p, exp));
    }
    Self { primes }
  }
}

impl Display for PrimeFactorization {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_zero() {
      return write!(f, "0");
    } else if self.is_one() {
      return write!(f, "1");
    }

    let mut first = true;
    for (p, exp) in &self.primes {
      if first {
        first = false;
      } else {
        write!(f, " * ")?;
      }
      write!(f, "{p}^{exp}")?;
    }
    write!(f, " = {}", self.val())
  }
}

impl Debug for PrimeFactorization {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{:?}",
      self.primes.iter().map(|(p, exp)| [p, exp]).collect_vec()
    )
  }
}
