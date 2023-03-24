use std::error::Error;

use ratatui::{
  layout::{Constraint, Direction, Layout, Rect},
  text::Span,
  widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
  ui::util::*,
  ui::{prompt_value, Frame},
  Greeter,
};

pub fn draw(greeter: &mut Greeter, f: &mut Frame) -> Result<(u16, u16), Box<dyn Error>> {
  let size = f.size();

  let width = greeter.width();
  let height = get_height(greeter);
  let container_padding = greeter.container_padding();
  let x = (size.width - width) / 2;
  let y = (size.height - height) / 2;

  let container = Rect::new(x, y, width, height);
  let frame = Rect::new(x + container_padding, y + container_padding, width - container_padding, height - container_padding);

  let block = Block::default().title(titleize(&fl!("title_command"))).borders(Borders::ALL).border_type(BorderType::Plain);

  f.render_widget(block, container);

  let constraints = [
    Constraint::Length(1), // Username
  ];

  let chunks = Layout::default().direction(Direction::Vertical).constraints(constraints.as_ref()).split(frame);
  let cursor = chunks[0];

  let command_label_text = prompt_value(Some(fl!("new_command")));
  let command_label = Paragraph::new(command_label_text);
  let command_value_text = Span::from(greeter.new_command.clone());
  let command_value = Paragraph::new(command_value_text);

  f.render_widget(command_label, chunks[0]);
  f.render_widget(
    command_value,
    Rect::new(1 + chunks[0].x + fl!("new_command").len() as u16, chunks[0].y, get_input_width(greeter, &Some(fl!("new_command"))), 1),
  );

  let new_command = greeter.new_command.clone();
  let offset = get_cursor_offset(greeter, new_command.chars().count());

  Ok((2 + cursor.x + fl!("new_command").len() as u16 + offset as u16, cursor.y + 1))
}
