use log::{debug, info};
use ndarray::prelude::*;
use rayon::prelude::*;
use std::{fmt::Display, usize};

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

impl<const N: usize> Default for Node<N> {
  fn default() -> Self {
    Self { data: [1; N] }
  }
}

struct System<const N: usize> {
  weights: [f64; N],
  grid: Array2<Node<N>>,
}

fn zip_mod<const N: usize>(origin: &mut Node<N>, trigger: &Features<N>) {
  origin.data.iter_mut().zip(trigger.iter()).for_each(|(o, t)| {
    *o *= *t;
  });
}

trait Wave<const N: usize> {
  fn propagate(grid: &mut Array2<Node<N>>, trigger: &Features<N>, direction: Direction, x: usize, y: usize) {
    // info!("Intermidiate state:\n{}", grid);
    let target = grid.get_mut((x, y));
    if let Some(node) = target {
      if node.get_entropy() > 1 {
        debug!("trigger {:?}", trigger);
        let effect = Self::rule(&direction, trigger);
        debug!("rule: {:?}", effect);
        debug!("node: {:?}", node);
        zip_mod(node, &effect);
        debug!("res: {:?}", node);
        debug!("");

        if node.get_entropy() == 1 {
          let data = node.data;
          Self::propagate(grid, &data, Direction::Right, x + 1, y);
          Self::propagate(grid, &data, Direction::Down, x, y + 1);
          Self::propagate(grid, &data, Direction::Left, x.saturating_sub(1), y);
          Self::propagate(grid, &data, Direction::Up, x, y.saturating_sub(1));
        }
      }
    }
  }
  fn rule(direction: &Direction, trigger: &Features<N>) -> Features<N>;
  fn show(&self) -> String;
}

const FEATURE_SIZE: usize = 3;

impl Wave<FEATURE_SIZE> for System<FEATURE_SIZE> {
  fn rule(direction: &Direction, trigger: &Features<FEATURE_SIZE>) -> Features<FEATURE_SIZE> {
    match trigger {
      [1, 0, 0] => [0, 1, 1],
      [0, 1, 0] => [1, 0, 1],
      [0, 0, 1] => [1, 1, 0],
      [_, _, _] => [1, 1, 1],
    }
  }

  fn show(&self) -> String {
    todo!()
  }
}

impl Display for Node<FEATURE_SIZE> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self.data {
        [1, 0, 0] => "\x1b[30;41mT1\x1b[0m".to_string(),
        [0, 1, 0] => "🌊".to_string(),
        [0, 0, 1] => "\x1b[30;43mT3\x1b[0m".to_string(),
        // [0, 0, 1] => "🌳".to_string(),
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

#[derive(Default)]
struct Game {
  round: u32,
  system: System<FEATURE_SIZE>,
}

impl Game {
  fn run(&mut self, trigger: &Features<FEATURE_SIZE>, x: usize, y: usize) {
    use Direction::*;
    System::propagate(&mut self.system.grid, trigger, Direct, x, y);
    self.round += 1;
    println!("RND {}", self.round);
    println!("{}", self.system.grid);
  }
}

impl<const N: usize> Default for System<N> {
  fn default() -> Self {
    Self {
      weights: [0.25; N],
      grid: array![
        [Node::default(), Node::default(), Node::default()],
        [Node::default(), Node::default(), Node::default()],
        [Node::default(), Node::default(), Node::default()],
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
  game.run(&[0, 1, 0], 2, 1);
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn same_game() {
  //   use Direction::*;

  //   let mut system = System::default();
  //   let trigger = [1, 0, 0];
  //   system.propagate(&trigger, Direct, 1, 1);
  //   let trigger = [0, 1, 0];
  //   system.propagate(&trigger, Direct, 1, 1);

  //   let mut game = Game::default();
  //   game.run(&[1, 0, 0], 1, 1);
  //   game.run(&[0, 1, 0], 1, 1);

  //   assert_eq!(system.grid, game.system.grid);
  // }
}
