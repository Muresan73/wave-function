mod node;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
fn main() -> AppResult<()> {
  // Create an application.

  node::main();
  Ok(())
}
