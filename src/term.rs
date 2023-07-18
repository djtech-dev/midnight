use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::{widgets::*, Terminal};
use std::error::Error;
use std::{
    io::{self, Stdout},
    time::Duration,
};
use crossterm::event::ModifierKeyCode;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}
pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut waiting_from_left_alt: bool = false;
    let mut waiting_from_left_mod: bool = false;
    let mut waiting_from_left_ctrl: bool = false;

    Ok(loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello World!");
            frame.render_widget(greeting, frame.size());
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('x') => {
                        if waiting_from_left_alt {
                            todo!(); // <= launch Command Prompt
                            waiting_from_left_alt = false;
                            waiting_from_left_ctrl = false;
                            waiting_from_left_mod = false;
                        }
                    }

                    KeyCode::Modifier(ModifierKeyCode::LeftAlt) => {
                        waiting_from_left_alt = true;
                    }
                    KeyCode::Modifier(ModifierKeyCode::LeftControl) => {
                        waiting_from_left_ctrl = true;
                    }
                    KeyCode::Modifier(ModifierKeyCode::LeftSuper) => {
                        waiting_from_left_mod = true;
                    }

                    _ => {}
                }
            }
        }
    })
}
