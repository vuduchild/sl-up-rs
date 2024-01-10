use std::{
    error::Error,
    io::{self, Stdout, Write},
    process::Output,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use sl_up::{sapling_cmd::get_smartlog, smartlog::SmartLog};

fn main() -> Result<(), Box<dyn Error>> {
    let raw_smartlog = get_smartlog().unwrap();
    let mut smartlog = SmartLog::new(&raw_smartlog);

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    render_smartlog(&mut stdout, &mut smartlog);

    let mut output: Option<Output> = None;
    'main: loop {
        loop {
            let input = crossterm::event::read()?;
            match input {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => break 'main,
                    KeyCode::Up => {
                        smartlog.move_up();
                    }
                    KeyCode::Down => {
                        smartlog.move_down();
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        output = Some(smartlog.checkout_selected_commit().unwrap());
                        break 'main;
                    }
                    _ => {}
                },
                _ => {}
            }
            render_smartlog(&mut stdout, &smartlog);
        }
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    if let Some(output) = output {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }

    Ok(())
}

fn render_smartlog(stdout: &mut Stdout, smartlog: &SmartLog) {
    stdout.queue(Clear(ClearType::All)).unwrap();
    for (idx, line) in smartlog.to_string_vec().iter().enumerate() {
        stdout.queue(MoveTo(0 as u16, idx as u16)).unwrap();
        print!("{}", *line);
    }
    stdout.flush().unwrap();
}
