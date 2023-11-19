extern crate test;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::node::bench;
  use test::Bencher;

  #[bench]
  fn benchmark(b: &mut Bencher) {
    b.iter(|| bench());
  }
}
