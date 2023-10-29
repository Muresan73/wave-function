use std::usize;

use ndarray::prelude::*;
use rayon::prelude::*;
enum Feature {
  T1,
  T2,
  T3,
}

fn get_adjacent<const N: usize>(grid: &Array2<Node<N>>, x: usize, y: usize) -> [Option<&Node<N>>; 4] {
  [
    grid.get((x - 1, y)),
    grid.get((x + 1, y)),
    grid.get((x, y - 1)),
    grid.get((x, y + 1)),
  ]
}

type Features<const N: usize> = [usize; N];

struct Node<const N: usize> {
  data: Features<N>,
}

impl<const N: usize> Node<N> {
  fn get_entropy(&self) -> usize {
    self.data.iter().sum()
  }
}

impl<const N: usize> Default for Node<N> {
  fn default() -> Self {
    Self { data: [1; N] }
  }
}

struct System<const N: usize> {
  weights: [f64; N],
  grid: Array2<Node<N>>,
  rules: Array2<Feature>,
}

impl<const N: usize> System<N> {
  // convoltion layer
  fn propagate(&self, trigger: &Features<N>, x: usize, y: usize) {
    let mut target = self.grid.get((x, y));
    match target {
      Some(node) if node.get_entropy() > 1 => {
        node.data.iter().zip(trigger.iter()).map(|(o, t)| {
          let _ = *o * t;
        });
        if node.get_entropy() == 1 {
          let _ = rule_infer(&self.rules, trigger);
        }
      }
      Some(_) | None => {}
    }
  }
}

fn rule3(trigger: &Features<3>) -> Features<3> {
  match trigger {
    [1, 0, 0] => [1, 0, 1],
    [0, 1, 0] => [1, 1, 0],
    [0, 0, 1] => [0, 0, 1],
    [_, 0, _] => [1, 0, 1],
    [_, _, _] => todo!(),
  }
}
use Feature::*;

fn rule_infer<const N: usize>(rules: &Array2<Feature>, trigger: &Features<N>) -> Features<N> {
  [0; N]
}

impl From<Feature> for Features<3> {
  fn from(value: Feature) -> Self {
    use Feature::*;
    match value {
      T1 => [1, 0, 0],
      T2 => [0, 1, 0],
      T3 => [0, 0, 1],
      _ => todo!(),
    }
  }
}

fn main() {
  let system = System {
    weights: [0.25; 3],
    grid: array![
      [Node::<3>::default(), Node::default(), Node::default()],
      [Node::default(), Node::default(), Node::default()],
      [Node::default(), Node::default(), Node::default()],
    ],
    #[rustfmt::skip]
    rules: array![
    [T1, T1, T1, T2],
    [T1, T1, T1, T3],
    [T1, T1, T1, T3],
    ],
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn propagation() {
    let system = System {
      weights: [0.25; 3],
      grid: array![
        [Node::<3>::default(), Node::default(), Node::default()],
        [Node::default(), Node::default(), Node::default()],
        [Node::default(), Node::default(), Node::default()],
      ],
      #[rustfmt::skip]
      rules: array![
      [T1, T1, T1, T2],
      [T1, T1, T1, T3],
      [T1, T1, T1, T3],
      ],
    };
    let trigger = [1, 0, 0];
    system.propagate(&trigger, 1, 1);
  }
}
