use ndarray::prelude::*;
use rayon::prelude::*;
use std::{fmt::Display, thread::sleep, usize};

pub fn main() {
  env_logger::init();
  let system = System {
    weights: [0.25; FEATURE_SIZE],
    grid: Array2::default((50, 70)),
  };
  let mut game = Game::new(system);

  use crossterm::cursor::*;
  use crossterm::{execute, ExecutableCommand};
  sleep(std::time::Duration::from_millis(600));
  println!("Strtaing simulation");

  std::io::stdout().execute(SavePosition);
  std::io::stdout().execute(RestorePosition);

  while game.next_round().is_some() {
    // sleep(std::time::Duration::from_secs(1));
    game.next_round();
    if game.round % 100 == 0 {
      println!("RND {},{:?}", game.round, crossterm::cursor::position());

      println!("{}", game.system.show());
      std::io::stdout().execute(RestorePosition);
      std::io::stdout().execute(MoveTo(0, 0));
    }
  }
  println!("{}", game.system.show());
}

trait Wave<const N: usize> {
  fn propagate(
    grid: &mut Array2<Node<N>>,
    trigger: &Features<N>,
    direction: Direction,
    x: usize,
    y: usize,
  ) -> Option<()> {
    // info!("Intermidiate state:\n{}", grid);
    let node = grid.get_mut((x, y))?;
    if node.get_entropy() > 1 {
      let effect = Self::rule(&direction, trigger);
      zip_mut(node, &effect);

      if node.get_entropy() == 1 {
        let data = node.data;
        Self::propagate(grid, &data, Direction::Right, x + 1, y);
        Self::propagate(grid, &data, Direction::Down, x, y + 1);
        Self::propagate(grid, &data, Direction::Left, x.saturating_sub(1), y);
        Self::propagate(grid, &data, Direction::Up, x, y.saturating_sub(1));
      }
    }
    Some(())
  }
  fn rule(direction: &Direction, trigger: &Features<N>) -> Features<N>;
  fn show(&self) -> String;
}

impl Wave<FEATURE_SIZE> for System<FEATURE_SIZE> {
  fn rule(direction: &Direction, trigger: &Features<FEATURE_SIZE>) -> Features<FEATURE_SIZE> {
    use Tile::*;
    match direction {
      Direction::Direct => *trigger,
      #[rustfmt::skip]
      _add_direction => match trigger.into() {
        Water => [1, 1, 0, 0, 0],
        Sand  => [1, 1, 1, 0, 0],
        Grass => [0, 1, 1, 1, 1],
        Tree  => [0, 0, 1, 1, 0],
        Rock  => [0, 1, 1, 0, 0],
        _     => [1, 1, 1, 1, 1],
      },
    }
  }

  fn show(&self) -> String {
    self
      .grid
      .axis_iter(Axis(0))
      .map(|x| x.iter().map(|x| x.to_string()).collect::<String>())
      .collect::<Vec<String>>()
      .join("\n")
  }
}

// ==============
// Enums
// ==============

#[derive(PartialEq)]
enum Direction {
  Left,
  Right,
  Up,
  Down,
  Direct,
}

type Features<const N: usize> = [usize; N];
const FEATURE_SIZE: usize = 5;

enum Tile {
  Tree,
  Water,
  Sand,
  Rock,
  Grass,
  Invalid,
  SP,
}

impl From<&Features<FEATURE_SIZE>> for Tile {
  #[rustfmt::skip]
  fn from(value: &Features<FEATURE_SIZE>) -> Self {
    match value {
      [1, 0, 0, 0, 0] => Tile::Water,
      [0, 1, 0, 0, 0] => Tile::Sand,
      [0, 0, 1, 0, 0] => Tile::Grass,
      [0, 0, 0, 1, 0] => Tile::Tree,
      [0, 0, 0, 0, 1] => Tile::Rock,
      [0, 0, 0, 0, 0] => Tile::Invalid,
      _               => Tile::SP,
    }
  }
}

impl From<Tile> for &Features<FEATURE_SIZE> {
  #[rustfmt::skip]
  fn from(value: Tile) -> Self {
    match value {
      Tile::Water   => &[1, 0, 0, 0, 0],
      Tile::Sand    => &[0, 1, 0, 0, 0],
      Tile::Grass   => &[0, 0, 1, 0, 0],
      Tile::Tree    => &[0, 0, 0, 1, 0],
      Tile::Rock    => &[0, 0, 0, 0, 1],
      Tile::Invalid => &[0, 0, 0, 0, 0],
      Tile::SP      => panic!("Superposition has no one to one mapping to a feature vector."),
    }
  }
}

// ==============
// Data types
// ==============

struct System<const N: usize> {
  weights: [f64; N],
  grid: Array2<Node<N>>,
}

impl<const N: usize> Default for System<N> {
  fn default() -> Self {
    Self {
      weights: [0.25; N],
      grid: Array2::default((10, 10)),
    }
  }
}

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord)]
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

impl Display for Node<FEATURE_SIZE> {
  #[rustfmt::skip]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Tile::*;
    write!( f, "{}",
      match (&self.data).into() {
        Water   => "\x1b[00;44mT1\x1b[0m".to_string(),
        Sand    => "\x1b[30;43mT2\x1b[0m".to_string(),
        Grass   => "\x1b[30;42mT3\x1b[0m".to_string(),
        Tree    => "🌳".to_string(),
        Rock    => "🪨 ".to_string(),
        Invalid => "\x1b[0;31mXX\x1b[0m".to_string(),
        _ => match self.get_entropy() {
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
  fn new(system: System<FEATURE_SIZE>) -> Self {
    Self { round: 0, system }
  }
  fn run(&mut self, trigger: &Features<FEATURE_SIZE>, x: usize, y: usize) {
    use Direction::*;
    System::propagate(&mut self.system.grid, trigger, Direct, x, y);
    self.round += 1;
  }
  fn next_round(&mut self) -> Option<()> {
    let ((x, y), low_entropy_node) = self
      .system
      .grid
      .indexed_iter()
      .filter(|((_, _), node)| node.get_entropy() > 1)
      .min()?;

    let mut rnd = rand::rngs::ThreadRng::default();
    let trigger = get_rnd_tile(&low_entropy_node.data, &mut rnd);
    self.run(&trigger, x, y);
    Some(())
  }
}

// ================
// Helper functions
// ================

fn zip_mut<const N: usize>(origin: &mut Node<N>, trigger: &Features<N>) {
  origin.data.iter_mut().zip(trigger.iter()).for_each(|(o, t)| {
    *o *= *t;
  });
}

fn get_rnd_tile<const N: usize>(superposition: &Features<N>, rnd: &mut impl rand::RngCore) -> Features<N> {
  use rand::seq::SliceRandom;

  let index_list: Vec<_> = superposition
    .iter()
    .enumerate()
    .filter(|(_, &value)| value != 0)
    .map(|(index, _)| index)
    .collect();

  let index = index_list.choose(rnd).expect("Node is not in superposition.");
  let mut result = [0; N];
  result[*index] = 1;
  result
}

// ================
// Benchmark
// ================

pub fn bench() {
  let system = System {
    weights: [0.25; FEATURE_SIZE],
    grid: Array2::default((30, 30)),
  };
  let mut game = Game::new(system);
  while game.next_round().is_some() {
    game.next_round();
  }
}

//test get_rnd_tile
mod tests {
  use super::*;
  #[test]
  fn test_get_rnd_tile() {
    use rand::prelude::*;
    let superposition = [0, 1, 0, 1, 0];
    let mut rnd = rand::rngs::StdRng::seed_from_u64(1);
    let result = get_rnd_tile(&superposition, &mut rnd);
    assert_eq!(result, [0, 0, 0, 1, 0]);

    let superposition = [0, 1, 0, 1, 0, 0, 1, 1, 1];
    let result = get_rnd_tile(&superposition, &mut rnd);
    assert_eq!(result, [0, 0, 0, 0, 0, 0, 1, 0, 0]);
  }
}
