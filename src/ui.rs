use std::io::{self, Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

use crate::smartlog::SmartLog;

pub fn start_ui_and_get_selected_commit<'a>(smartlog: &'a mut SmartLog) -> Option<&'a str> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();
    render_smartlog(&mut stdout, smartlog);

    let mut commit_hash: Option<&str> = None;
    'terminal_ui: loop {
        let input = crossterm::event::read().unwrap();
        if let Event::Key(key_event) = input {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => break 'terminal_ui,
                KeyCode::Char('c') => {
                    if key_event
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL)
                    {
                        break 'terminal_ui;
                    }
                }
                KeyCode::Up => {
                    smartlog.move_up();
                }
                KeyCode::Down => {
                    smartlog.move_down();
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    commit_hash = Some(smartlog.get_selected_commit_hash().unwrap());
                    break 'terminal_ui;
                }
                _ => {}
            }
        }
        render_smartlog(&mut stdout, smartlog);
    }

    // Cleanup
    stdout.execute(Show).unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();

    commit_hash
}

fn render_smartlog(stdout: &mut Stdout, smartlog: &SmartLog) {
    stdout.queue(Clear(ClearType::All)).unwrap();
    for (idx, line) in smartlog.to_string_vec().iter().enumerate() {
        stdout.queue(MoveTo(0_u16, idx as u16)).unwrap();
        print!("{}", *line);
    }
    stdout.flush().unwrap();
}
