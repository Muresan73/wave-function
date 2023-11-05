use std::{fmt::Display, usize};

use ndarray::prelude::*;
use rayon::prelude::*;
enum Feature {
  T1,
  T2,
  T3,
}

enum Direction {
  Left,
  Right,
  Up,
  Down,
  Direct,
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

impl Display for Node<3> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{:?}",
      match self.data {
        [1, 0, 0] => "T1".to_string(),
        [0, 1, 0] => "T2".to_string(),
        [0, 0, 1] => "T3".to_string(),
        [_, _, _] => format!("S{}", self.get_entropy()),
      }
    )
  }
}

fn get3<const N: usize>(value: &Node<N>) -> Node<3> {
  Node::<3> {
    data: [value.data[0], value.data[1], value.data[2]],
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

fn zip_mod<const N: usize>(origin: &mut Node<N>, trigger: &Features<N>) {
  origin.data.iter_mut().zip(trigger.iter()).for_each(|(o, t)| {
    *o *= *t;
  });
}

impl<const N: usize> System<N> {
  // convoltion layer
  fn propagate(&mut self, trigger: &Features<N>, direction: Direction, x: usize, y: usize) {
    let target = self.grid.get_mut((x, y));
    match target {
      Some(node) if node.get_entropy() > 1 => {
        zip_mod(node, trigger);
        if node.get_entropy() == 1 {
          let _ = rule_infer(&self.rules, trigger);
          println!();
          println!("{}", self.grid.map(get3));
          self.propagate(trigger, Direction::Direct, x + 1, y);
          self.propagate(trigger, Direction::Direct, x, y + 1);
          self.propagate(trigger, Direction::Direct, x.saturating_sub(1), y);
          self.propagate(trigger, Direction::Direct, x, y.saturating_sub(1));
        }
      }
      Some(_) | None => {}
    }
  }
}

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

pub fn main() {
  use Direction::*;

  let mut system = System {
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
  let x = 1;
  let y = 1;
  system.grid.get((x, y));
  println!("RND Start");
  println!("{}", system.grid);

  system.propagate(&trigger, Direct, 1, 1);
  println!("RND 1");
  println!("{}", system.grid);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn propagation() {
    use Direction::*;
    let mut system = System {
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
    system.propagate(&trigger, Direct, 1, 1);
  }

  #[test]
  fn iter_test() {
    let mut original1 = Node::<3>::default();
    let trigger1 = [1, 0, 0];
    zip_mod1(&mut original1, &trigger1);

    let mut original2 = Node::<3>::default();
    let trigger2 = [1, 0, 0];
    zip_mod2(&mut original2, &trigger2);

    println!("a{:?}", original1.data);
    println!("b{:?}", original2.data);
    assert_eq!(original1.data, original2.data);
  }
}
