use crate::app::App;
use tui::{
  backend::Backend,
  layout::{Alignment, Constraint::*, Direction, Layout},
  style::{Color, Style},
  widgets::{
    canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    Block, BorderType, Borders, Paragraph,
  },
  Frame,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
  // This is where you add new widgets.
  // See the following resources:
  // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
  // - https://github.com/ratatui-org/ratatui/tree/master/examples

  let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![Percentage(50), Length(15),Min(0)])
    .split(frame.size());
  let work_area = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Length(25 ),
      Length(9),
      Min(0),
    ])
    .split(layout[1]);

  frame.render_widget(
    Canvas::default()
      .block(Block::default().title("Canvas").borders(Borders::ALL))
      .x_bounds([-180.0, 180.0])
      .y_bounds([-90.0, 90.0])
      .paint(|ctx| {
        ctx.draw(&Line {
          x1: 0.0,
          y1: 10.0,
          x2: 10.0,
          y2: 10.0 + app.counter as f64,
          color: Color::White,
        });
        ctx.draw(&Rectangle {
          x: 10.0,
          y: 20.0,
          width: 10.0,
          height: 10.0,
          color: Color::Red,
        });
      }),
    layout[0],
  );
  frame.render_widget(
    Paragraph::new("Hmmm....")
      .style(Style::default().bg(Color::Blue).fg(Color::Yellow))
      .block(Block::default().title("Work area").borders(Borders::ALL)),
      work_area[0],
  );
  frame.render_widget(
    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
      .style(Style::default().bg(Color::Blue).fg(Color::Yellow))
      .block(Block::default().title("B2").borders(Borders::ALL)),
      work_area[1],
  );
}
