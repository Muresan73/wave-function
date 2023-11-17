use log::{debug, info};
use ndarray::prelude::*;
use rayon::prelude::*;
use std::{fmt::Display, usize};
use Feature::*;
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

#[derive(Clone, PartialEq, Debug)]
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
      "{}",
      match self.data {
        [1, 0, 0] => "\x1b[30;41mT1\x1b[0m".to_string(),
        [0, 1, 0] => "\x1b[30;42mT2\x1b[0m".to_string(),
        [0, 0, 1] => "\x1b[30;43mT3\x1b[0m".to_string(),
        [0, 0, 0] => "\x1b[0;31mXX\x1b[0m".to_string(),
        [_, _, _] => match self.get_entropy() {
          2 => "\x1b[0;34mS2\x1b[0m".to_string(),
          3 => "\x1b[0;36mS3\x1b[0m".to_string(),
          _ => format!("S{}", self.get_entropy()),
        },
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

impl System<3> {
  // convoltion layer
  fn propagate(&mut self, trigger: &Features<3>, direction: Direction, x: usize, y: usize) {
    info!("Intermidiate state:\n{}", &self.grid);
    let target = self.grid.get_mut((x, y));
    if let Some(node) = target {
      if node.get_entropy() > 1 {
        debug!("trigger {:?}", trigger);
        let rule = rule_infer(&direction, trigger);
        debug!("rule: {:?}", rule);
        debug!("node: {:?}", node);
        zip_mod(node, &rule);
        debug!("res: {:?}", node);
        debug!("");

        if node.get_entropy() == 1 {
          let data = node.data;
          self.propagate(&data, Direction::Right, x + 1, y);
          self.propagate(&data, Direction::Down, x, y + 1);
          self.propagate(&data, Direction::Left, x.saturating_sub(1), y);
          self.propagate(&data, Direction::Up, x, y.saturating_sub(1));
        }
      }
    }
  }
}

fn rule_infer(rules: &Direction, trigger: &Features<3>) -> Features<3> {
  rule3(trigger)
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
    [1, 0, 0] => [0, 1, 1],
    [0, 1, 0] => [1, 0, 1],
    [0, 0, 1] => [1, 1, 0],
    [_, _, _] => [1, 1, 1],
  }
}

#[derive(Default)]
struct Game {
  round: u32,
  system: System<3>,
}

impl Game {
  fn new(system: System<3>) -> Self {
    println!("Game Start");
    println!("{}", system.grid);
    Game { round: 0, system }
  }
  fn run(&mut self, trigger: &Features<3>, x: usize, y: usize) {
    use Direction::*;
    self.system.propagate(trigger, Direct, x, y);
    self.round += 1;
    println!("RND {}", self.round);
    println!("{}", self.system.grid);
  }
}

impl Default for System<3> {
  fn default() -> Self {
    Self {
      weights: [0.25; 3],
      grid: array![
        [Node::default(), Node::default(), Node::default()],
        [Node::default(), Node::default(), Node::default()],
        [Node::default(), Node::default(), Node::default()],
      ],
      #[rustfmt::skip]
        rules: array![
        [T1, T1, T1, T2],
        [T1, T1, T1, T3],
        [T1, T1, T1, T3],
        ],
    }
  }
}

pub fn main() {
  env_logger::init();
  let mut game = Game::default();
  game.run(&[1, 0, 0], 1, 1);
  game.run(&[0, 1, 0], 1, 1);
  game.run(&[1, 0, 0], 1, 2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn same_game() {
    use Direction::*;

    let mut system = System::default();
    let trigger = [1, 0, 0];
    system.propagate(&trigger, Direct, 1, 1);
    let trigger = [0, 1, 0];
    system.propagate(&trigger, Direct, 1, 1);

    let mut game = Game::default();
    game.run(&[1, 0, 0], 1, 1);
    game.run(&[0, 1, 0], 1, 1);

    assert_eq!(system.grid, game.system.grid);
  }

  #[test]
  fn propagation() {
    use Direction::*;
    let mut system = System {
      weights: [0.25; 3],
      grid: array![
        [Node::default(), Node::default(), Node::default()],
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
}
