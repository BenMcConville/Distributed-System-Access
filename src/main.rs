#![allow(warnings)]

use std::{borrow::BorrowMut, error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};
use crate::ui::ui;
pub mod app;
pub mod network;
pub mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // let n1 = Node::new_server();
    // println!("{}", n1.get_node_name());
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new();
    let _res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    println!("Servers: {:?}", app.get_servers());
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.current_screen {
                CurrentScreen::Editing => match key.code {
                    KeyCode::Esc => app.current_screen = CurrentScreen::Main,
                    KeyCode::Enter => match app.currently_editing {
                        Some(CurrentlyEditing::Key) => {
                            app.currently_editing = Some(CurrentlyEditing::Value);
                        }
                        Some(CurrentlyEditing::Value) => {
                            app.currently_editing = Some(CurrentlyEditing::Key);
                            app.current_screen = CurrentScreen::Main;
                            // app.save_key_value();
                        }
                        _ => (),
                    },
                    KeyCode::Backspace => match app.currently_editing {
                        Some(CurrentlyEditing::Key) => {
                            app.node_input.pop();
                        }
                        Some(CurrentlyEditing::Value) => {
                            app.value_input.pop();
                        }
                        _ => (),
                    },
                    KeyCode::Char(c) => match app.currently_editing {
                        Some(CurrentlyEditing::Key) => app.node_input.push(c),
                        Some(CurrentlyEditing::Value) => app.value_input.push(c),
                        _ => (),
                    },
                    _ => (),
                },
                _ => {
                    if key.code == KeyCode::Char('q') {
                        app.current_screen = CurrentScreen::Exiting;
                        return Ok(false);
                    }
                    if key.code == KeyCode::Char('e') {
                        app.current_screen = CurrentScreen::Editing;
                    }
                    if key.code == KeyCode::Tab {
                        app.toggle_editing();
                    }
                    if key.code == KeyCode::Up {
                        //('a') {
                        app.move_up();
                    }
                    if key.code == KeyCode::Right {
                        app.toggle_selected_node();
                    }
                    if key.code == KeyCode::Down {
                        //('z') {
                        app.move_down();
                    }
                }
            }
        }
    }
}
