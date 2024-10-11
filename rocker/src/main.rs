use std::io::Error;

use crossterm::event::{self,Event,KeyCode,KeyEvent,KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block,Paragraph};



fn main() {
    let mut screen: Terminal<CrosstermBackend<std::io::Stdout>> = ratatui::init();
}

fn draw_home(mut screen: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(),Error> {
    /* pretty self-explanatory what this should do */
    loop {
        screen.draw( |frame| {
            frame.render_widget(Paragraph::new("Welcome to the Rocker TUI tool.").block(Block::new().title("Rocker Home")),frame.area())
        })?;
      if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e') {
            break;
        }
      }
    }
    /* the restore will be a test for now, as the code gets rewritten */
    ratatui::restore();
    Ok(())
    
}
/* rest will be in other files */